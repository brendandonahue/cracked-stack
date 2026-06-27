import { createAuthClient } from "better-auth/svelte"
import { PUBLIC_API_URL } from '$env/static/public';

export const authClient = createAuthClient({
  baseURL: PUBLIC_API_URL, // or internal K8s service DNS, e.g. http://rocket-svc:8000
  // basePath: "/api/auth"  // only if you expose routes under this exact path on Rocket
  fetchOptions: {
    credentials: "include", // critical for cookies to be sent/received
  },
})