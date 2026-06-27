Software stack for tile-elite:

**Frontend:** SvelteKit, Tailwind, DaisyUI  
**Mobile (optional):** Tauri  
**Backend:** Rust, Rocket.rs, SurrealDB  

**Deployment (this repo):** Docker Compose (`docker-compose.yml`), nginx for static `build/`, Caddy as reverse proxy on the host, [`deploy.sh`](../deploy.sh) for releases. Kubernetes is not defined in this repository.

**HTTP API:** Mounted routes are listed in README (authoritative list: [`server/src/main.rs`](../server/src/main.rs) `routes![]`). New data access should use `POST /graphql` via the Rocket proxy.

Make no mistakes. Do not hallucinate endpoints that are not mounted.