import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import tailwindcss from "@tailwindcss/vite";
import { mdsvex } from "mdsvex";

// Tauri doesn't have a Node.js server to do proper SSR
// so we use adapter-static with a fallback to 200.html to put the site in SPA mode
// See: https://svelte.dev/docs/kit/single-page-apps
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import adapter from "@sveltejs/adapter-static";

const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [
    tailwindcss(),
    // SvelteKit 3 reads its config here; svelte.config.js is no longer supported.
    sveltekit({
      preprocess: [vitePreprocess(), mdsvex()],
      adapter: adapter({ fallback: "200.html" }),
      extensions: [".svelte", ".svx"],
      // SvelteKit 3 removed the built-in $lib alias (it is now #lib); keep $lib so existing imports work.
      alias: { $lib: "src/lib" }
    })
  ],

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
      // SvelteKit 3: $env/static/public is replaced by $app/env/public.
      // Must stay before "$app/env" because Vite string aliases match by prefix.
      "$app/env/public": new URL(
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
      // SvelteKit 3 renamed $app/environment to $app/env
      "$app/env": new URL(
        "./src/__mocks__/app-environment.ts",
        import.meta.url
      ).pathname,
      // Map $lib to the real source
      "$lib": new URL("./src/lib", import.meta.url).pathname,
    },
  },
}));
