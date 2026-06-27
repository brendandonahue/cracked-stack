<script lang="ts">
  import { isAuthenticated } from '$lib/stores/auth';
</script>

<!-- ── Hero ─────────────────────────────────────────────────── -->
<section class="min-h-screen bg-base-100 flex flex-col items-center justify-center px-6 text-center">
  <div class="max-w-3xl mx-auto">
    <div class="badge badge-primary badge-outline mb-6 font-mono text-sm px-4 py-3">
      production-ready starter
    </div>

    <h1 class="text-5xl md:text-7xl font-black tracking-tight leading-tight mb-6">
      cracked<span class="text-primary">stack</span>
    </h1>

    <p class="text-xl md:text-2xl text-base-content/70 max-w-2xl mx-auto mb-10 leading-relaxed">
      A full-stack application template built with <strong>SvelteKit</strong>,
      <strong>Rocket.rs</strong>, and <strong>SurrealDB</strong> — with auth, GraphQL,
      and Docker deployment baked in. Skip the boilerplate. Ship faster.
    </p>

    <div class="flex flex-wrap gap-4 justify-center">
      {#if $isAuthenticated}
        <a href="/dashboard" class="btn btn-primary btn-lg">Go to Dashboard</a>
      {:else}
        <a href="/signup" class="btn btn-primary btn-lg">Get Started Free</a>
        <a href="/login" class="btn btn-ghost btn-lg">Login</a>
      {/if}
      <a
        href="https://github.com/brendandonahue/cracked-stack"
        target="_blank"
        rel="noopener noreferrer"
        class="btn btn-outline btn-lg gap-2"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="size-5" viewBox="0 0 24 24" fill="currentColor">
          <path d="M12 0C5.374 0 0 5.373 0 12c0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23A11.509 11.509 0 0 1 12 5.803c1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576C20.566 21.797 24 17.3 24 12c0-6.627-5.373-12-12-12z"/>
        </svg>
        View on GitHub
      </a>
    </div>
  </div>
</section>

<!-- ── Stack Overview ─────────────────────────────────────────── -->
<section class="bg-base-200 py-24 px-6">
  <div class="max-w-5xl mx-auto">
    <h2 class="text-3xl md:text-4xl font-bold text-center mb-4">The Stack</h2>
    <p class="text-base-content/60 text-center mb-16 max-w-xl mx-auto">
      Every piece chosen for performance, developer experience, and production reliability.
    </p>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">

      <div class="card bg-base-100 shadow-lg hover:shadow-xl transition-shadow">
        <div class="card-body">
          <div class="text-4xl mb-3">⚡</div>
          <h3 class="card-title">SvelteKit</h3>
          <p class="text-base-content/70 text-sm">
            Reactive UI with Svelte 5 runes, Tailwind CSS, and DaisyUI components.
            SSR optional — ships as a static SPA for Tauri compatibility.
          </p>
          <div class="card-actions mt-2">
            <span class="badge badge-ghost font-mono text-xs">SvelteKit</span>
            <span class="badge badge-ghost font-mono text-xs">Tailwind</span>
            <span class="badge badge-ghost font-mono text-xs">DaisyUI</span>
          </div>
        </div>
      </div>

      <div class="card bg-base-100 shadow-lg hover:shadow-xl transition-shadow">
        <div class="card-body">
          <div class="text-4xl mb-3">🦀</div>
          <h3 class="card-title">Rocket.rs</h3>
          <p class="text-base-content/70 text-sm">
            Type-safe async HTTP API in Rust. Cookie-based auth, GraphQL proxy,
            rate limiting, and CORS — all production-hardened.
          </p>
          <div class="card-actions mt-2">
            <span class="badge badge-ghost font-mono text-xs">Rust</span>
            <span class="badge badge-ghost font-mono text-xs">Rocket</span>
            <span class="badge badge-ghost font-mono text-xs">Async</span>
          </div>
        </div>
      </div>

      <div class="card bg-base-100 shadow-lg hover:shadow-xl transition-shadow">
        <div class="card-body">
          <div class="text-4xl mb-3">🗄️</div>
          <h3 class="card-title">SurrealDB</h3>
          <p class="text-base-content/70 text-sm">
            Multi-model database with row-level permissions baked into the schema.
            Auto-generates a GraphQL API from your table definitions.
          </p>
          <div class="card-actions mt-2">
            <span class="badge badge-ghost font-mono text-xs">SurrealDB</span>
            <span class="badge badge-ghost font-mono text-xs">GraphQL</span>
            <span class="badge badge-ghost font-mono text-xs">RBAC</span>
          </div>
        </div>
      </div>

      <div class="card bg-base-100 shadow-lg hover:shadow-xl transition-shadow">
        <div class="card-body">
          <div class="text-4xl mb-3">🔐</div>
          <h3 class="card-title">Auth Built In</h3>
          <p class="text-base-content/70 text-sm">
            Secure httpOnly cookie sessions, signup/login/logout, password reset
            with email tokens, and route guards — all wired up from day one.
          </p>
          <div class="card-actions mt-2">
            <span class="badge badge-ghost font-mono text-xs">Argon2</span>
            <span class="badge badge-ghost font-mono text-xs">httpOnly</span>
            <span class="badge badge-ghost font-mono text-xs">SameSite</span>
          </div>
        </div>
      </div>

      <div class="card bg-base-100 shadow-lg hover:shadow-xl transition-shadow">
        <div class="card-body">
          <div class="text-4xl mb-3">🐳</div>
          <h3 class="card-title">Docker Deploy</h3>
          <p class="text-base-content/70 text-sm">
            Docker Compose orchestrates SurrealDB, the Rust API, and nginx serving
            the SvelteKit build. Caddy handles TLS on the host. One script to deploy.
          </p>
          <div class="card-actions mt-2">
            <span class="badge badge-ghost font-mono text-xs">Docker</span>
            <span class="badge badge-ghost font-mono text-xs">nginx</span>
            <span class="badge badge-ghost font-mono text-xs">Caddy</span>
          </div>
        </div>
      </div>

      <div class="card bg-base-100 shadow-lg hover:shadow-xl transition-shadow">
        <div class="card-body">
          <div class="text-4xl mb-3">📱</div>
          <h3 class="card-title">Tauri Ready</h3>
          <p class="text-base-content/70 text-sm">
            The SvelteKit frontend runs as a native desktop/mobile app via Tauri
            with zero changes — adapter-static + <code class="font-mono text-xs">ssr=false</code> makes it work.
          </p>
          <div class="card-actions mt-2">
            <span class="badge badge-ghost font-mono text-xs">Tauri</span>
            <span class="badge badge-ghost font-mono text-xs">macOS</span>
            <span class="badge badge-ghost font-mono text-xs">Windows</span>
          </div>
        </div>
      </div>

    </div>
  </div>
</section>

<!-- ── Auth Flow ──────────────────────────────────────────────── -->
<section class="bg-base-100 py-24 px-6">
  <div class="max-w-4xl mx-auto">
    <h2 class="text-3xl md:text-4xl font-bold text-center mb-4">How Auth Works</h2>
    <p class="text-base-content/60 text-center mb-16 max-w-xl mx-auto">
      Zero third-party auth services. Cookie-based sessions using SurrealDB's native argon2 hashing.
    </p>

    <div class="steps steps-vertical lg:steps-horizontal w-full">
      <div class="step step-primary">
        <div class="step-content p-4 text-left">
          <h4 class="font-bold">Signup / Login</h4>
          <p class="text-sm text-base-content/60">SvelteKit calls <code class="font-mono text-xs">POST /signup</code> or <code class="font-mono text-xs">POST /login</code></p>
        </div>
      </div>
      <div class="step step-primary">
        <div class="step-content p-4 text-left">
          <h4 class="font-bold">Token Set</h4>
          <p class="text-sm text-base-content/60">Rocket issues a SurrealDB token as an httpOnly Secure cookie</p>
        </div>
      </div>
      <div class="step step-primary">
        <div class="step-content p-4 text-left">
          <h4 class="font-bold">Authenticated Requests</h4>
          <p class="text-sm text-base-content/60">Every <code class="font-mono text-xs">apiFetch()</code> call sends the cookie automatically</p>
        </div>
      </div>
      <div class="step step-primary">
        <div class="step-content p-4 text-left">
          <h4 class="font-bold">DB Permissions</h4>
          <p class="text-sm text-base-content/60">SurrealDB enforces row-level access via <code class="font-mono text-xs">$auth.id</code> in the schema</p>
        </div>
      </div>
    </div>
  </div>
</section>

<!-- ── CTA ───────────────────────────────────────────────────── -->
<section class="bg-primary text-primary-content py-20 px-6 text-center">
  <h2 class="text-3xl md:text-4xl font-bold mb-4">Ready to build?</h2>
  <p class="text-primary-content/80 max-w-md mx-auto mb-8">
    Clone the repo, run the seed script, and have a working full-stack app in minutes.
  </p>
  <div class="flex flex-wrap gap-4 justify-center">
    <a href="/signup" class="btn btn-lg bg-white text-primary hover:bg-white/90 border-0 font-bold">
      Create an Account
    </a>
    <a
      href="https://github.com/brendandonahue/cracked-stack"
      target="_blank"
      rel="noopener noreferrer"
      class="btn btn-lg btn-outline border-white text-white hover:bg-white hover:text-primary"
    >
      Read the Docs
    </a>
  </div>
</section>

<!-- ── Footer ────────────────────────────────────────────────── -->
<footer class="bg-base-200 py-8 px-6 text-center text-base-content/50 text-sm font-mono">
  cracked<span class="text-primary font-bold">stack</span> — MIT License
</footer>
