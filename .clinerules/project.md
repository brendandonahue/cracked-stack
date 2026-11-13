# Project-specific findings & caveats

This file accumulates things learned the hard way across sessions so they
don't have to be re-researched (and re-burn tokens) every time. See also
`architecture.md` (stack/routes) and `coding.md` (established conventions).
Keep entries terse and link to the authoritative source file rather than
duplicating long explanations.

## Auth / SurrealDB connections ([`server/src/auth.rs`](../server/src/auth.rs))

- `Db` holds TWO separate SurrealDB WS connections:
  - `client` — root-authenticated ONCE at startup (`ensure_connected`). Every route handler (`uploads.rs`, `main.rs` password reset) queries on it **outside** `auth_guard`. **Never** `authenticate()`/`signin()`/`invalidate()` it after startup.
  - `record_client` — ns/db selected, no root auth. Used for `login`, `signup` and for validating JWTs in the `AuthUser` guard.
- `auth_guard` (tokio `Mutex`) serializes every use of `record_client`. Any new code that changes `record_client`'s session identity must hold it and call `reset_record_client()` (invalidate + re-select ns/db) before releasing, on success AND failure.
- **Fixed race (don't reintroduce):** the guard used to `authenticate()` the shared root `client` as the user, query `$auth`, then re-sign-in as root. That only serialized the swap; a concurrent request's handler could run queries on `client` while another request's guard had it authenticated as a different user → wrong permissions or intermittent 500 "Database error". Found in the `hall-link` repo (cloned from this template) and fixed there first, then ported here.
- `reauth_root_inner` is only used at startup now.
- Token extraction (`extract_token`): `auth_token` cookie wins, else `Authorization: Bearer`. Unit-tested in `auth.rs`.
- `AuthUser` still selects `company_id` (optional) from `$auth` — leftover from the app this template came from; `schema.surql` `user` table has no such field (returns NONE). Remove/rename if you touch it.
- Signup role: `main.rs` downgrades `admin` to `user`; admins are set manually in the DB.

## GraphQL proxy ([`server/src/graphql.rs`](../server/src/graphql.rs))

- Forwards the user's own `auth_token` cookie as `Authorization: Bearer` to SurrealDB `/graphql` with `Surreal-NS`/`Surreal-DB` headers, so table PERMISSIONS in `schema.surql` are what enforce access. Only the cookie is forwarded (a Bearer-header-only caller passes the guard but gets "No auth token found").
- Note: it logs the full response at `info` level.

## Uploads ([`server/src/uploads.rs`](../server/src/uploads.rs))

- Binary on disk under `UPLOAD_DIR` (default `uploads`; `/uploads` in Docker via `uploads-volume`), metadata in the `file` table. Stored name = uuid + extension derived from Content-Type.
- `GET /files/<id>` uses `Option<AuthUser>`; private files return 404 (not 403) to non-owners. Authorization is enforced in the handler using root `client`, not table permissions.
- Rocket limits (`main.rs`): `data` 2 MiB, `file` 10 MiB.

## Stripe ([`server/src/stripe.rs`](../server/src/stripe.rs))

- Custom, SDK-free: `reqwest` form POST to `https://api.stripe.com/v1/checkout/sessions` (inline `price_data`, so no Dashboard products needed). Routes: `GET /stripe/plans`, `POST /stripe/checkout` (AuthUser + RateLimitGuard), `POST /stripe/webhook` (no auth — Stripe calls it; protected by `Stripe-Signature` HMAC-SHA256 over `<t>.<raw body>`, 5 min tolerance, constant-time compare via `hmac::verify_slice`).
- Prices are in `PLANS` (server-side only). The client sends just `plan_id`; never accept amounts from the client.
- `payment` rows are written ONLY by the webhook on the root `db.client` via `UPSERT payment:<checkout session id>` (idempotent for re-delivered events). `schema.surql` gives users `select` on own rows and `NONE` for create/update/delete — a Rust test (`users_can_read_only_their_own_payments_and_never_write_them`) enforces this against an in-memory SurrealDB; keep it passing.
- The user is linked via Checkout `client_reference_id` = user record key (`user:abc` → `abc`); the webhook rebuilds `RecordId::new("user", key)`.
- Env: `STRIPE_SECRET_KEY`, `STRIPE_WEBHOOK_SECRET`, `APP_BASE_URL` (falls back to `FRONTEND_URL`). Missing keys → `Error::StripeNotConfigured` (503), NOT a startup panic. Never log them. Container gets them via `docker-compose.yml`.
- Webhook is not behind `AuthUser`, so Caddy must forward `/stripe/webhook` as-is (it does; ROCKET_HOST proxies everything). The `.env.example` `PUBLIC_STRIPE_PUBLISHABLE_KEY` / `STRIPE_PRICE_*` entries were removed: hosted Checkout needs neither.
- Frontend: `getStripePlans`, `createCheckoutSession`, `getPayments`, `formatMoney` in `src/lib/api.ts`; demo page `src/routes/payments/`. `@stripe/stripe-js` / `svelte-stripe` in `package.json` are still unused (redirect flow, no Stripe.js).
- Tests: `cargo test` now 53 passing; `npm run test` 108.

## Schema ([`schema.surql`](../schema.surql))

- Tables: `user`, `item` (demo), `password_reset_tokens`, `file`, `payment`. Access method `account` (RECORD, token 1d, session 4w2d). Everything uses `OVERWRITE` so it's re-runnable. GraphQL via `DEFINE CONFIG GRAPHQL AUTO`.
- Password reset runs on root `client` (the `password_reset_tokens` PERMISSIONS only matter for GraphQL/user sessions).

## Environment variables

- Server reads `SURREALDB_HOST`, `SURREALDB_PORT` (default 8001), `SURREALDB_USER`, `SURREALDB_PASS`, `SURREALDB_NS`, `SURREALDB_DB`, `UPLOAD_DIR`, plus SMTP vars and the optional Stripe vars (`STRIPE_SECRET_KEY`, `STRIPE_WEBHOOK_SECRET`, `APP_BASE_URL`). These match `docker-compose.yml`.
- README's "Configure environment variables" block lists `SURREAL_URL`/`SURREAL_USER`/… — that does NOT match the code. Trust `SURREALDB_*`.
- `SURREALDB_NS`/`DB`/`USER`/`PASS` use `.expect()` — missing vars panic.
- Email From: `SMTP_FROM` (falls back to `SMTP_USER`) + optional `SMTP_FROM_NAME`; empty = unset. See `build_from_mailbox` in [`server/src/notifications.rs`](../server/src/notifications.rs). Never log SMTP credentials.
- CORS origins are hardcoded in `main.rs` (`http://localhost:1420`, `https://crackedstack.com`); update when changing domains.
- Auth cookie is `Secure` + `SameSite=Strict` → local HTTP dev may need localhost handling (browsers treat localhost as secure context).

## Rate limiting

- `RateLimitGuard` in `main.rs` = 1 req/second per client for all routes that use it (login, signup, validate, health, reset flows, `/stripe/checkout`). `/graphql`, uploads and `/stripe/webhook` (Stripe's servers; signature-verified instead) are not rate limited.

## Deployment

- `docker-compose.yml`: nginx (static `build/`, SPA fallback to `200.html`) + Rust server; both on the **external** docker network `app-net` (must exist: `docker network create app-net`). Caddy on the host does TLS/proxying.
- `nginx.conf` only serves static files; it does NOT reverse-proxy `/api` despite what README says — routing to the Rocket container is Caddy's job.
- `Dockerfile.rust` uses nightly Rust (edition 2024 crate), build context is `./server`; runtime image installs ffmpeg + curl (healthcheck uses curl on `/health`).
- `docker-compose.yml` passes OLLAMA_* vars but the AI module is unmounted.

## Tooling gotchas

- Shell is Windows PowerShell 7: no `&&` between commands in old syntax-safe usage — prefer `cargo test --manifest-path server/Cargo.toml` from the repo root.
- Backend baseline: `cargo test` → 53 passing tests (`auth`, `error`, `notifications`, `stripe`, `uploads`); ~8 dead-code warnings from unmounted modules (`notifications::send_sms`, etc.) are expected.
- Frontend: `npm run test` (vitest + happy-dom), `npm run check`.
- Tauri is optional (`src-tauri/`); `@stripe/stripe-js`, `better-auth`, `@zxing/*` appear in `package.json` but are not part of the documented template.
