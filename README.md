# cracked-stack

A production-ready full-stack application starter. Clone it, replace the demo `item` model with your domain, and ship.

---

## Stack

| Layer | Technology |
|---|---|
| **Frontend** | SvelteKit 2 · Svelte 5 runes · Tailwind CSS · DaisyUI |
| **Backend** | Rust · Rocket.rs · async/await |
| **Database** | SurrealDB (multi-model · row-level permissions · GraphQL auto-generated) |
| **Deployment** | Docker Compose · nginx (static) · Caddy (TLS reverse proxy) |
| **Mobile / Desktop** | Tauri (optional — SPA mode works out of the box) |

---

## Features

- **Cookie-based auth** — httpOnly · Secure · SameSite=Strict · 24h TTL
- **Argon2 password hashing** via SurrealDB's native `crypto::argon2` functions
- **Password reset** — time-limited tokens stored in DB, email delivery via SMTP
- **GraphQL API** — SurrealDB auto-generates a typed GraphQL schema from your tables; Rocket proxies it to the frontend
- **Route guards** — SvelteKit layout checks auth on every navigation; unauthenticated users are redirected to `/login`
- **Toast notifications** — lightweight store-driven toast system
- **Rate limiting** — Rocket Governor guards on all auth endpoints
- **CORS** — configured per environment in `main.rs`

---

## HTTP API (mounted routes)

All routes are defined in `server/src/main.rs` `routes![]`. This is the authoritative list.

| Method | Path | Auth | Description |
|---|---|---|---|
| `POST` | `/login` | — | Issue auth cookie |
| `POST` | `/signup` | — | Create account + issue auth cookie |
| `POST` | `/logout` | — | Clear auth cookie |
| `GET` | `/profile` | ✓ | Return current user info |
| `GET` | `/validate` | ✓ | Validate token (health check for auth) |
| `GET` | `/health` | — | Server health check |
| `POST` | `/request-password-reset` | — | Send password reset email |
| `POST` | `/reset-password` | — | Apply new password via token |
| `POST` | `/graphql` | ✓ | GraphQL proxy to SurrealDB |
| `POST` | `/upload` | ✓ | Upload a file (multipart/form-data, field `file`, optional `public` bool); returns `{ id, url }` |
| `GET` | `/files/<id>` | ✓* | Serve a file's bytes. Public files are open to anyone; private files require the owner or an admin (`*` — guard is optional, authorization enforced in the handler) |
| `DELETE` | `/files/<id>` | ✓ | Delete a file (owner or admin only) — removes both the DB record and the file on disk |

---


## Project Structure

```
cracked-stack/
├── src/                        # SvelteKit frontend
│   ├── routes/
│   │   ├── +layout.svelte      # Auth guard + Navbar + Dock
│   │   ├── +page.svelte        # Public landing page
│   │   ├── login/              # Login page
│   │   ├── signup/             # Signup page
│   │   ├── forgot-password/    # Password reset request
│   │   ├── reset-password/     # Password reset confirm
│   │   ├── dashboard/          # Authenticated home
│   │   ├── items/              # Demo CRUD resource ← replace with your domain
│   │   └── profile/            # User profile + logout
│   └── lib/
│       ├── api.ts              # apiFetch + graphqlQuery + item helpers
│       ├── auth/guards.ts      # Route permission helpers
│       ├── stores/auth.ts      # Svelte auth store
│       ├── stores/toast.ts     # Toast notification store
│       └── components/         # Navbar, Dock, Toast, Modal, etc.
│
├── server/                     # Rocket.rs backend
│   └── src/
│       ├── main.rs             # Routes, CORS, Rocket config
│       ├── auth.rs             # Db struct, AuthUser guard, SurrealDB connection
│       ├── graphql.rs          # GraphQL proxy handler
│       ├── uploads.rs          # File upload/serve/delete (POST /upload, GET & DELETE /files/<id>)
│       ├── notifications.rs    # Password reset email (SMTP)
│       └── error.rs            # Shared Error type

│
├── schema.surql                # SurrealDB schema (run once on a fresh DB)
├── seed.surql                  # Demo seed data
├── docker-compose.yml          # SurrealDB + Rust API + nginx
├── nginx.conf                  # Serves SvelteKit build/
├── Dockerfile.rust             # Builds the Rocket server
└── deploy.sh                   # Build + push + restart on server
```

---

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org) ≥ 20
- [Rust](https://rustup.rs) (stable)
- [SurrealDB](https://surrealdb.com/install) ≥ 2.x
- [Docker](https://docker.com) (for production deployment)

### 1. Clone the repo

```bash
git clone https://github.com/brendandonahue/cracked-stack.git
cd cracked-stack
```

### 2. Configure environment variables

Create a `.env` file in the root and in `server/`:

```bash
# .env  (frontend — SvelteKit reads PUBLIC_ vars)
PUBLIC_API_URL=http://localhost:8000

# server/.env  (Rocket reads these)
SURREAL_URL=ws://127.0.0.1:8001
SURREAL_USER=root
SURREAL_PASS=root
SURREAL_NS=crackedstack
SURREAL_DB=crackedstack
SMTP_HOST=smtp.example.com
SMTP_PORT=587
SMTP_USER=you@example.com
SMTP_PASS=yourpassword
SMTP_FROM=noreply@example.com
APP_URL=http://localhost:5173
```

### 3. Start SurrealDB

```bash
surreal start --user root --pass root --bind 127.0.0.1:8001 file:./surreal.db
```

### 4. Apply the schema and seed data

```bash
surreal import --conn ws://127.0.0.1:8001 --user root --pass root \
  --ns crackedstack --db crackedstack schema.surql

surreal import --conn ws://127.0.0.1:8001 --user root --pass root \
  --ns crackedstack --db crackedstack seed.surql
```

**Demo credentials (from `seed.surql`):**

| Email | Password | Role |
|---|---|---|
| `admin@example.com` | `Admin1234` | admin |
| `demo@example.com` | `Demo1234` | user |

### 5. Start the Rust API

```bash
cd server
cargo run
# API listens on http://localhost:8000
```

### 6. Start the SvelteKit frontend

```bash
cd ..
npm install
npm run dev
# Frontend at http://localhost:5173
```

---

## How Auth Works

```
Browser → POST /login → Rocket → SurrealDB.signin()
                                        ↓
                              Issues JWT token
                                        ↓
Rocket → Set-Cookie: auth_token=<jwt>; HttpOnly; Secure; SameSite=Strict
                                        ↓
Browser stores cookie (invisible to JS)
                                        ↓
Every apiFetch() → credentials:'include' → cookie sent automatically
                                        ↓
AuthUser guard (Rocket) → validates token with SurrealDB
                                        ↓
SurrealDB executes query as authenticated user → row-level perms enforced
```

The `AuthUser` guard in `server/src/auth.rs` extracts and validates the cookie on every protected route. If the token is invalid or expired, a `401` is returned and the SvelteKit layout redirects to `/login`.

---

## Database Schema

`schema.surql` defines four tables:

- **`user`** — email, name, hashed password, role (`user` | `admin`), created_at
- **`item`** — demo CRUD table with row-level ownership (replace with your domain)
- **`password_reset_tokens`** — short-lived tokens for password reset emails
- **`file`** — metadata for uploaded files (owner, filename, content type, size, public flag); binary content lives on disk under `UPLOAD_DIR` (`./uploads` locally, `/uploads` in Docker via the `uploads-volume`)


SurrealDB GraphQL is auto-generated from these tables via `DEFINE CONFIG GRAPHQL AUTO`.

---

## Making It Your Own

1. **Replace `item`** — edit `schema.surql`, `seed.surql`, `src/lib/api.ts` (Item section), and `src/routes/items/` with your domain model
2. **Add roles** — extend `UserRole` in `src/lib/auth/guards.ts` and add role-gating in `isRoleAllowedForPath`
3. **Add REST routes** — add a new `.rs` module in `server/src/`, `mod` it in `main.rs`, and add to `routes![]`
4. **Style** — swap the DaisyUI theme in `src/app.css` (`data-theme` attribute)

---

## Deployment

The project ships with a complete Docker Compose setup.

```bash
# On your server (Caddy handles TLS)
./deploy.sh
```

`deploy.sh` builds the SvelteKit static bundle, builds the Rust Docker image, pushes to your registry, and restarts the Compose stack. `nginx.conf` serves `build/` and reverse-proxies `/api` to the Rocket container.

---

## Development Commands

```bash
npm run dev          # SvelteKit dev server
npm run build        # Build static frontend
npm run check        # Svelte type check
npm run test         # Vitest unit tests

cd server
cargo run            # Start Rocket API
cargo check          # Type-check without building
cargo test           # Run backend tests
```

---

## License

MIT
