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

## Schema ([`schema.surql`](../schema.surql))

- Tables: `user`, `item` (demo), `password_reset_tokens`, `file`. Access method `account` (RECORD, token 1d, session 4w2d). Everything uses `OVERWRITE` so it's re-runnable. GraphQL via `DEFINE CONFIG GRAPHQL AUTO`.
- Password reset runs on root `client` (the `password_reset_tokens` PERMISSIONS only matter for GraphQL/user sessions).

## Environment variables

- Server reads `SURREALDB_HOST`, `SURREALDB_PORT` (default 8001), `SURREALDB_USER`, `SURREALDB_PASS`, `SURREALDB_NS`, `SURREALDB_DB`, `UPLOAD_DIR`, plus SMTP vars. These match `docker-compose.yml`.
- README's "Configure environment variables" block lists `SURREAL_URL`/`SURREAL_USER`/… — that does NOT match the code. Trust `SURREALDB_*`.
- `SURREALDB_NS`/`DB`/`USER`/`PASS` use `.expect()` — missing vars panic.
- CORS origins are hardcoded in `main.rs` (`http://localhost:1420`, `https://crackedstack.com`); update when changing domains.
- Auth cookie is `Secure` + `SameSite=Strict` → local HTTP dev may need localhost handling (browsers treat localhost as secure context).

## Rate limiting

- `RateLimitGuard` in `main.rs` = 1 req/second per client for all routes that use it (login, signup, validate, health, reset flows). `/graphql` and uploads are not rate limited.

## Deployment

- `docker-compose.yml`: nginx (static `build/`, SPA fallback to `200.html`) + Rust server; both on the **external** docker network `app-net` (must exist: `docker network create app-net`). Caddy on the host does TLS/proxying.
- `nginx.conf` only serves static files; it does NOT reverse-proxy `/api` despite what README says — routing to the Rocket container is Caddy's job.
- `Dockerfile.rust` uses nightly Rust (edition 2024 crate), build context is `./server`; runtime image installs ffmpeg + curl (healthcheck uses curl on `/health`).
- `docker-compose.yml` passes OLLAMA_* vars but the AI module is unmounted.

## Tooling gotchas

- Shell is Windows PowerShell 7: no `&&` between commands in old syntax-safe usage — prefer `cargo test --manifest-path server/Cargo.toml` from the repo root.
- Backend baseline: `cargo test` → 25 passing tests (`auth`, `error`, `uploads`); ~8 dead-code warnings from unmounted modules (`notifications::send_sms`, etc.) are expected.
- Frontend: `npm run test` (vitest + happy-dom), `npm run check`.
- Tauri is optional (`src-tauri/`); `@stripe/stripe-js`, `better-auth`, `@zxing/*` appear in `package.json` but are not part of the documented template.
