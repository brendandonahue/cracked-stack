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
- **Stripe payments** — small custom integration (no SDK): hosted Checkout Sessions with server-owned prices, signed webhook (HMAC-SHA256) that records payments, demo at `/payments`
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
| `GET` | `/stripe/plans` | — | Demo plan catalogue (prices are owned by the server) and whether Stripe is configured |
| `POST` | `/stripe/checkout` | ✓ | Body `{ plan_id }`. Creates a Stripe Checkout Session and returns `{ url }` to redirect the browser to |
| `POST` | `/stripe/webhook` | Stripe signature | Called by Stripe. Verifies `Stripe-Signature` (HMAC-SHA256, 5 min tolerance) and records a paid session in the `payment` table. Never call it from the frontend |

---

## Stripe payments

A small custom integration in `server/src/stripe.rs` — plain REST calls, no Stripe SDK.

```
Browser → POST /stripe/checkout {plan_id} → Rocket → Stripe (creates Checkout Session)
Browser ← { url } ← Rocket           Browser → redirected to Stripe-hosted checkout
Stripe  → POST /stripe/webhook (signed) → Rocket verifies HMAC → UPSERT payment:<session id>
Browser → /payments → GraphQL `payment` (PERMISSIONS: owner or admin can read; nobody can write)
```

- **Prices live on the server** (`PLANS` in `stripe.rs`); the client only sends a plan id. Replace the three demo plans with your own.
- **Payments are only written by the webhook**, on the root DB client. The `payment` table denies create/update/delete to users, so a payment can't be forged through GraphQL. The record id is the Checkout Session id, so repeated webhook deliveries are idempotent.
- **Card data never touches your server** — customers pay on Stripe-hosted Checkout.

Setup (test mode):

1. Set `STRIPE_SECRET_KEY`, `STRIPE_WEBHOOK_SECRET` and `APP_BASE_URL` (see `.env.example`; `docker-compose.yml` passes them to the Rust container).
2. Re-import `schema.surql` to create the `payment` table.
3. Forward webhooks locally: `stripe listen --forward-to localhost:8000/stripe/webhook` (it prints the `whsec_…` secret). In production, add an endpoint `https://<ROCKET_HOST>/stripe/webhook` in the Stripe Dashboard for `checkout.session.completed` and `checkout.session.async_payment_succeeded`.
4. Sign in, open **Payments**, and pay with test card `4242 4242 4242 4242`.

If `STRIPE_SECRET_KEY` is unset the server stays healthy; `/stripe/checkout` returns `503` and the Payments page shows a notice.

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
│   │   ├── payments/           # Demo Stripe checkout + payment history
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
│       ├── stripe.rs           # Stripe Checkout + signed webhook (GET /stripe/plans, POST /stripe/checkout, POST /stripe/webhook)
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
surreal start --bind 127.0.0.1:8001 --log info --user root --pass root rocksdb://C:/surrealdb/dev.db
```

### 4. Apply the schema and seed data

```bash
surreal import --endpoint http://127.0.0.1:8001 --user root --pass root \
  --ns crackedstack --db crackedstack schema.surql

surreal import --endpoint http://127.0.0.1:8001 --user root --pass root \
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

`schema.surql` defines five tables:

- **`user`** — email, name, hashed password, role (`user` | `admin`), created_at
- **`item`** — demo CRUD table with row-level ownership (replace with your domain)
- **`password_reset_tokens`** — short-lived tokens for password reset emails
- **`file`** — metadata for uploaded files (owner, filename, content type, size, public flag); binary content lives on disk under `UPLOAD_DIR` (`./uploads` locally, `/uploads` in Docker via the `uploads-volume`)
- **`payment`** — Stripe payments (owner, plan, amount in cents, currency, status). Readable by the owner or an admin; written only by the signed webhook on the root client


SurrealDB GraphQL is auto-generated from these tables via `DEFINE CONFIG GRAPHQL AUTO`.

---

## Making It Your Own

1. **Replace `item`** — edit `schema.surql`, `seed.surql`, `src/lib/api.ts` (Item section), and `src/routes/items/` with your domain model
2. **Add roles** — extend `UserRole` in `src/lib/auth/guards.ts` and add role-gating in `isRoleAllowedForPath`
3. **Add REST routes** — add a new `.rs` module in `server/src/`, `mod` it in `main.rs`, and add to `routes![]`
4. **Style** — the app uses the custom DaisyUI theme `cracked` in `src/app.css`, set with `data-theme="cracked"` on `<html>`

---

## Deployment

The project ships with Docker Compose plus a host-level Caddy reverse proxy for automatic HTTPS.

```bash
# On your server (Caddy handles TLS)
cp Caddyfile.example /path/to/Caddyfile   # set CADDY_EMAIL, SVELTE_HOST, ROCKET_HOST
./deploy.sh
```

`deploy.sh` builds the SvelteKit static bundle, builds the Rust image, and restarts the Compose stack. `nginx.conf` serves `build/`. Caddy (see `Caddyfile.example`) terminates TLS and reverse-proxies:

| Host env | Proxies to |
|---|---|
| `SVELTE_HOST` | `cracked-stack-nginx:80` |
| `ROCKET_HOST` | `cracked-stack-rust-server:8000` |

Containers expect a shared external Docker network named `app-net` (create with `docker network create app-net`).

---

## iOS (Tauri)

The iOS project lives in `src-tauri/gen/apple/` (generated by `npx tauri ios init`, git-ignored). Requires macOS, Xcode, an Apple Development certificate, and the iOS Rust targets (`rustup target add aarch64-apple-ios`). The signing team is set in `src-tauri/tauri.conf.json` (`bundle.iOS.developmentTeam`).

Put your iPhone's id in `.env` (git-ignored) so it never lands in the repo. Find it with `xcrun devicectl list devices`:

```bash
IOS_DEVICE_ID=<your-device-udid>
```

```bash
npm run ios:build     # npx tauri ios build --export-method debugging
npm run ios:install   # xcrun devicectl device install app --device $IOS_DEVICE_ID src-tauri/gen/apple/build/arm64/cracked-stack.ipa
npm run ios:device    # build + install in one go
IOS_DEVICE_ID=<other-udid> npm run ios:install   # a shell value overrides .env
```

`ios:install` runs `scripts/ios-install.mjs`, which loads `.env` itself (npm scripts don't) and fails with a clear message if `IOS_DEVICE_ID` is missing or the IPA hasn't been built.

**Custom icons.** The app icon source is `src-tauri/app-icon.svg`, derived from the web `static/favicon.svg` (same artwork, but full-bleed with no rounded corners, since iOS applies its own mask and rejects transparency). `npm run ios:icons` regenerates the desktop icons in `src-tauri/icons/`, the Xcode `AppIcon.appiconset`, and the Android launcher icons. If you change the favicon artwork, update `app-icon.svg` to match and re-run it. After a fresh `npx tauri ios init`, re-run `npm run ios:icons` too, because init writes the default Tauri icons.

**Caveats.**
- The IPA name comes from `productName` (`cracked-stack`); the Xcode project and scheme are still named after the Cargo crate (`yap`).
- The bundled app loads `PUBLIC_API_URL` from `.env` at build time. `http://127.0.0.1:8000` points at the phone itself, so set it to a reachable HTTPS API (e.g. `https://api.example.com`) before building for a device.
- The API cookie is `Secure` + `SameSite=Strict` and CORS origins are hardcoded in `server/src/main.rs`. The iOS webview origin (`tauri://localhost`) is not in that list, so add it before expecting login to work from the app.
- First install: trust the developer profile on the phone (Settings → General → VPN & Device Management) and enable Developer Mode.

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
