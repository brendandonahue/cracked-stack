// SvelteKit 3 environment variables (replaces the deprecated $env/static/public module).
// Values come from `.env` / the process environment at build time.
import { defineEnvVars } from "@sveltejs/kit/env";

export const variables = defineEnvVars({
  PUBLIC_API_URL: {
    public: true,
    static: true,
    description: "Base URL of the Rocket API, e.g. http://127.0.0.1:8000"
  }
});
