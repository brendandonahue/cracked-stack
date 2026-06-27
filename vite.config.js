import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import tailwindcss from "@tailwindcss/vite";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [tailwindcss(), sveltekit()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,

  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: "ws", host, port: 1421 } : undefined,

    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"]
    }
  },

  // Vitest configuration
  test: {
    include: ["src/**/*.test.ts", "src/**/*.test.js"],
    globals: true,
    environment: "happy-dom",
    setupFiles: ["src/test-setup.ts"],
    alias: {
      // Map SvelteKit virtual modules to test doubles
      "$env/static/public": new URL(
        "./src/__mocks__/env-public.ts",
        import.meta.url
      ).pathname,
      "$app/stores": new URL(
        "./src/__mocks__/app-stores.ts",
        import.meta.url
      ).pathname,
      "$app/navigation": new URL(
        "./src/__mocks__/app-navigation.ts",
        import.meta.url
      ).pathname,
      "$app/environment": new URL(
        "./src/__mocks__/app-environment.ts",
        import.meta.url
      ).pathname,
      // Map $lib to the real source
      "$lib": new URL("./src/lib", import.meta.url).pathname,
    },
  },
}));
