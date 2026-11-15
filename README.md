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

## Local AI: use a Windows gaming PC as your Ollama server

`docker-compose.yml.example` shows a full deployment that runs Ollama in a container next to the app. That works, but a CPU-only web server is slow for LLMs. If you have a gaming PC on the same LAN, you can run Ollama there on the GPU instead and point every app at it. This guide uses an AMD Radeon RX 7900 XT (20 GB VRAM) as the example.

```
Web server (CPU only, Docker)                 Gaming PC (Windows, AMD GPU)
┌───────────────────────────┐   LAN :11434    ┌──────────────────────────┐
│ Rocket container          │ ──────────────▶ │ Ollama for Windows       │
│ OLLAMA_BASE_URL=http://   │                 │ models on the 7900 XT    │
│   192.168.1.50:11434      │                 │ (ROCm v7 or Vulkan)      │
└───────────────────────────┘                 └──────────────────────────┘
```

### Why it isn't a container on Windows

Containerizing this specific setup doesn't work today, so the tutorial runs Ollama natively on Windows:

- Docker Desktop's GPU passthrough on Windows (`--gpus`) is documented for **NVIDIA only**.
- The `ollama/ollama:rocm` image expects `/dev/kfd` and `/dev/dri`. Under WSL2 these don't exist. AMD's ROCm-on-WSL exposes the GPU as `/dev/dxg`, and Ollama's Linux installer and container image don't support that path out of the box.
- **Ollama for Windows** supports the 7900 XT natively (ROCm v7, with Vulkan as a fallback), runs as a background app, and serves the same HTTP API on port `11434`. To the apps it looks exactly like the container.

If you want a container, run `ollama/ollama:rocm` on a Linux host with the AMD ROCm v7 driver (see [Ollama's Docker docs](https://docs.ollama.com/docker)). The rest of this guide (LAN access, `.env`, tuning) applies unchanged.

### 1. Install and verify the GPU

1. Install the latest AMD Adrenalin driver from amd.com. Ollama's Windows ROCm backend needs a ROCm v7 / HIP7-capable driver stack. Vulkan ships with the regular driver and is enabled by default, so Ollama can use the card either way.
2. Install Ollama from [ollama.com/download](https://ollama.com/download) (`OllamaSetup.exe`, no Administrator needed). It starts in the background and serves `http://localhost:11434`.
3. Pull the model the template defaults to and check it landed on the GPU:

```powershell
ollama pull qwen3:14b
ollama run qwen3:14b "say hi"
ollama ps
```

`ollama ps` must show `100% GPU` in the PROCESSOR column. `100% CPU` or a CPU/GPU split means the GPU isn't being used fully. Check `%LOCALAPPDATA%\Ollama\server.log` for the detected library (ROCm or Vulkan).

### 2. Expose it on your LAN

Ollama binds to `127.0.0.1` by default. Examples below use `192.168.1.50` for the gaming PC and `192.168.1.10` for the web server. Substitute your own addresses.

1. **Give both machines fixed addresses.** Add DHCP reservations in your router (or set static IPs). The gaming PC's address is baked into `OLLAMA_BASE_URL` and into the bind address below, and the web server's address is baked into the firewall rule.
2. **Bind Ollama to the PC's LAN address**, not `0.0.0.0`, so it doesn't listen on VPN or virtual adapters. Quit Ollama from the tray icon, then set a user environment variable (Settings → search "environment variables" → *Edit environment variables for your account*, or use `setx`):

```powershell
setx OLLAMA_HOST "192.168.1.50:11434"
```

   Start Ollama again from the Start menu. `http://localhost:11434` stops working once it is bound to the LAN address. The `ollama` CLI follows `OLLAMA_HOST`, so `ollama ps` still works. If the PC's address ever changes, Ollama won't start listening, so keep the reservation.
3. **Set the network profile to Private** (Settings → Network & internet → your connection) and allow the port **from the web server only** (admin PowerShell):

```powershell
New-NetFirewallRule -DisplayName "Ollama from web server" -Direction Inbound -Protocol TCP -LocalPort 11434 -Action Allow -Profile Private -RemoteAddress 192.168.1.10/32
```

4. **From the web server**, confirm it is reachable:

```bash
curl http://192.168.1.50:11434/api/version
curl http://192.168.1.50:11434/api/tags
```

> **Security:** Ollama has no authentication. Anyone who can reach the port can run inference on your GPU and call its management endpoints (pull, create and delete models). Never port-forward `11434` or add it to the Caddyfile. See [Security](#5-security-for-the-gaming-pc-ollama-server) below for the full threat model and hardening.

### 3. Point the apps at it (`.env`)

Edit the `.env` on the web server (the one `docker-compose.yml` reads). Nothing else changes per app:

```bash
OLLAMA_BASE_URL=http://192.168.1.50:11434
OLLAMA_CS_MODEL=qwen3:14b
```

`docker-compose.yml` already passes both to the Rust container and defaults to `http://ollama:11434` when `OLLAMA_BASE_URL` is unset. Restart with `./deploy.sh` (or `docker compose up -d`). To switch back to an on-server Ollama, change the URL back (or remove the line).

If you started from `docker-compose.yml.example`, drop the local Ollama container so it doesn't sit idle or block startup:

1. Delete the `ollama:` service.
2. Delete `ollama-data:` from `volumes:`.
3. Delete the `ollama: condition: service_healthy` entry under the Rust server's `depends_on:`. Otherwise the server waits for a container that no longer exists.
4. Add `- OLLAMA_BASE_URL=${OLLAMA_BASE_URL}` and `- OLLAMA_CS_MODEL=${OLLAMA_CS_MODEL}` to the Rust server's `environment:`. The example doesn't pass them today.
5. Remove the now-unused `OLLAMA_HOST`, `OLLAMA_ORIGINS`, `OLLAMA_DEBUG`, `OLLAMA_KEEP_ALIVE`, `OLLAMA_MAX_LOADED_MODELS` and `OLLAMA_NUM_PARALLEL` from the web server's `.env`. Those are server-side settings and now live on the gaming PC (next section).

> **Current limitation:** the Ollama client code in `server/src/ai.rs` is not mounted in `main.rs`, and nothing in the mounted routes reads `OLLAMA_BASE_URL` yet. The variable is wired through Docker so it is ready when you mount an AI route. The unmounted `ai.rs` reads `OLLAMA_CHAT_URL` and `OLLAMA_CHAT_COMPLETIONS_URL` (default `http://ollama:11434/...`) rather than `OLLAMA_BASE_URL`. When you mount it, derive those URLs from `OLLAMA_BASE_URL` (or set them to `http://192.168.1.50:11434/api/chat` and `http://192.168.1.50:11434/v1/chat/completions`).

### 4. Getting the most out of the GPU

**Windows host**

- Plug the gaming PC into **Ethernet**, not Wi-Fi.
- Set the power plan to *High performance* / *Best performance* and stop the PC from sleeping, or the API goes offline when it's idle: `powercfg /change standby-timeout-ac 0`.
- Ollama starts when you **log in**. Enable automatic sign-in (or leave the PC logged in) so it comes back after a reboot or power cut.
- Keep the Adrenalin driver current, and enable **Resizable BAR / Smart Access Memory** in the BIOS if it isn't already.
- Put models on a fast NVMe drive: set a user variable `OLLAMA_MODELS` (for example `D:\ollama\models`). This mostly affects model load time.
- Don't game while the apps need the GPU. A running game and a loaded model compete for the same 20 GB of VRAM, and Ollama will spill to the CPU.

**Ollama settings** (user environment variables on the gaming PC; quit and restart Ollama after changing them)

| Variable | Suggested | Why |
|---|---|---|
| `OLLAMA_FLASH_ATTENTION` | `1` | Lower memory use at long context, and usually faster |
| `OLLAMA_KV_CACHE_TYPE` | `q8_0` | Roughly halves KV-cache memory vs. the `f16` default, so more context fits. Requires flash attention. Go back to `f16` if you notice quality loss |
| `OLLAMA_KEEP_ALIVE` | `-1` (or `24h`) | Keep the model resident. The default unloads after 5 minutes, so the next request pays a cold-load penalty |
| `OLLAMA_CONTEXT_LENGTH` | `8192`–`32768` | Ollama chooses a default from your VRAM. Set it explicitly and confirm with `ollama ps` that the model still shows `100% GPU` |
| `OLLAMA_NUM_PARALLEL` | `2`–`4` | Concurrent requests per model (default 1). Each slot adds context memory, so raise it only while VRAM allows |
| `OLLAMA_MAX_LOADED_MODELS` | `1` (or `2` for small models) | Stops several models from fighting for VRAM |
| `OLLAMA_GPU_OVERHEAD` | optional, bytes | Reserves VRAM for Windows and other apps if you hit out-of-memory errors |

```powershell
setx OLLAMA_FLASH_ATTENTION "1"
setx OLLAMA_KV_CACHE_TYPE "q8_0"
setx OLLAMA_KEEP_ALIVE "-1"
setx OLLAMA_CONTEXT_LENGTH "16384"
setx OLLAMA_NUM_PARALLEL "2"
setx OLLAMA_MAX_LOADED_MODELS "1"
```

These are starting points, not benchmarked values. Change one at a time and re-check `ollama ps`.

**Pick models that fit in 20 GB.** Full speed needs the whole model plus its context cache in VRAM. As a rough guide, 7B–14B models at 4-bit (like the default `qwen3:14b`) leave plenty of room for context. ~30B-class 4-bit models are around 18-20 GB by themselves and will likely spill to the CPU once you add context. Any CPU share in the PROCESSOR column of `ollama ps` means the model or context is too big.

**ROCm vs Vulkan.** On Windows, Ollama can use either ROCm (when the driver exposes ROCm v7) or Vulkan, and Vulkan is on by default. Which is faster depends on the model and driver, so measure with `ollama run <model> --verbose` and compare `eval rate` (tokens/s). Set `OLLAMA_VULKAN=0` to turn Vulkan off and compare. If the PC also has an integrated GPU that Ollama picks by mistake, set `GGML_VK_VISIBLE_DEVICES` to the discrete GPU's index.

**Optional:** Adrenalin's tuning panel can adjust the power limit or undervolt the card. Test stability thoroughly before relying on it for a server.

### 5. Security for the gaming PC (Ollama server)

**Native vs. Docker.** Running Ollama natively on Windows is the only setup that works with an AMD GPU (see above), and Docker wouldn't change the biggest risks. Native Ollama runs as your Windows user, so a compromised process can read that user's files (browser profiles, game launchers, documents). A container would limit that, but passing a GPU into it already weakens isolation, and bugs in Ollama's HTTP API are exposed the same way in both. On Linux, `ollama/ollama:rocm` in Docker is the better option and the hardening below still applies.

**What can go wrong**

| Risk | Why it matters |
|---|---|
| **No authentication** | Anyone who can reach `:11434` can use your GPU and call management endpoints (pull, create, delete models) |
| **The web server is the internet-facing machine** | If it is compromised, an attacker can pivot to Ollama on the gaming PC, which also holds your personal data. This is the main threat |
| **Wide firewall rules** | Allowing a whole subnet exposes Ollama to phones, TVs, IoT and guest devices |
| **Binding to `0.0.0.0`** | Listens on every adapter, including VPN and virtual ones |
| **Plain HTTP** | Prompts and responses cross the LAN unencrypted. For something like a customer-service agent, that may include customer data |

**Hardening checklist** (the first four are already in the steps above)

1. Allow the port from the **web server's IP only** (`-RemoteAddress <ip>/32`), not the whole subnet. This is the biggest improvement.
2. Bind `OLLAMA_HOST` to the PC's LAN IP, not `0.0.0.0`.
3. Use the **Private** network profile for the firewall rule.
4. Never port-forward `11434` and never add it to the Caddyfile. Only the Rocket container should talk to it.
5. **Remove any other Ollama firewall rules.** Windows may prompt you to allow `ollama.exe` on first run, which can create a broader rule that includes the Public profile. List them, and delete or disable any that are wider than your rule:

```powershell
Get-NetFirewallRule -DisplayName "*ollama*" | Format-Table DisplayName, Enabled, Profile, Direction, Action
```

6. Set `OLLAMA_NO_CLOUD` to `1` so Ollama's cloud features (remote inference and web search) stay off and the server is local-only: `setx OLLAMA_NO_CLOUD "1"`, then restart Ollama.
7. **Keep Ollama updated.** The Windows app updates itself (tray icon → *Restart to update*). Past security issues have been in its HTTP API, so updates matter.
8. Don't run anything else sensitive in the same Windows account if you can avoid it. Consider a separate, non-admin Windows user just for the Ollama server.

**Stronger options**

- **Separate VLAN or network segment** for the web server and gaming PC, if your router supports it, so a compromised web server can't reach the rest of your LAN.
- **WireGuard or Tailscale** between the two machines. This encrypts the traffic and means no LAN port needs to be exposed. Bind Ollama to the VPN address and point `OLLAMA_BASE_URL` at it. More setup, but it is the proper fix for plaintext HTTP.
- **Authenticating reverse proxy** on the PC (Caddy or nginx checking a bearer token) in front of Ollama, which then listens only on localhost. Probably overkill on a home LAN with the rules above, and the Rust code would need to send the token.

**A note on the firewall rule.** The Rocket container reaches the PC through its Docker host, so the PC sees the **web server's LAN address** as the source, which is what the `/32` rule matches. If you use Docker's host networking or a different path, check the source address with `Get-NetTCPConnection -LocalPort 11434` while a request is running.

### 6. Troubleshooting

| Symptom | Check |
|---|---|
| `curl` from the web server times out | The firewall rule only applies to the **Private** profile and to the single address in `-RemoteAddress`. Check the network profile, that the web server's IP matches the rule, and that Ollama was restarted after `setx OLLAMA_HOST` |
| Connection refused | Ollama is still bound to localhost (`OLLAMA_HOST` not applied), the PC's IP changed so the bind failed, or Ollama isn't running. `Get-NetTCPConnection -LocalPort 11434` should show the PC's LAN IP |
| `localhost:11434` stopped working on the PC | Expected once `OLLAMA_HOST` is bound to the LAN IP. Use `http://192.168.1.50:11434` (the `ollama` CLI already follows `OLLAMA_HOST`) |
| Works until you reboot | Ollama starts at login (see the Windows host tips), and the PC may have gone to sleep |
| `ollama ps` shows CPU | Model or context too large for VRAM, a game is using the GPU, or the driver is outdated. Read `%LOCALAPPDATA%\Ollama\server.log` |
| Rocket container can't reach the PC | Use the PC's LAN IP, not `localhost` or `host.docker.internal`. The container runs on a different machine |

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
