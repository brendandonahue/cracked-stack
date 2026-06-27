# Coding

Established conventions in this codebase:

## Fetching from the Rust API

Use `apiFetch` from [`src/lib/api.ts`](../src/lib/api.ts) for all backend HTTP calls (sends credentials / httpOnly `auth_token` cookie). Do not use raw `fetch` against the API except third-party URLs (e.g. geocoding).

For CRUD, prefer `graphqlQuery` and the typed helpers in `api.ts` over new REST endpoints.

**Mounted REST paths only** — source of truth is `server/src/main.rs` `routes![]`. See README section "HTTP API (mounted routes)". Do not call paths like `/cancelTileJob`, `/adminMetrics`, `/viewSchedule`; they are not registered.

**Unmounted in codebase (do not call from frontend until added to `main.rs`):**

- `server/src/crud.rs`: `/applicants`, duplicate `/recruits`, `/upload`, `/editApplicant`
- `server/src/ai.rs`: `/generate`, `/image` (OllamaChat — disabled until mounted)
- `server/src/notifications.rs`: `/send_email`, `/send_sms` (password reset uses internal helpers)

## Backend errors

Prefer new variants in [`server/src/error.rs`](../server/src/error.rs) and the shared `Error` type.

Read existing modules before adding patterns; ask clarifying questions when requirements are unclear.

## GraphQL

Prefer `POST /graphql` for queries and mutations when possible.

GRAPHQL QUERIES: single-line strings only. Newlines break mutations/queries.

## Testing

Ddd test coverage for each new feature added to the app, ensuring the app has updated test coverage at all times. Frontend tests are run with `npm run test` and backend uses `cargo test`. Comprehensive test coverage already exists for existing features.

## Workflow

Prefer not starting long-running dev servers in agent sessions. Running `npm run check` or `cargo check` in `server/` is OK to verify changes. The human runs `npm run dev`, SurrealDB, and the Rocket server for manual testing.