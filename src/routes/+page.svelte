<script lang="ts">
  import { isAuthenticated } from '$lib/stores/auth';
</script>

<svelte:head>
  <title>crackedstack — production-ready full-stack starter</title>
  <meta
    name="description"
    content="SvelteKit, Rocket.rs, SurrealDB, Docker, and Caddy automatic HTTPS. Auth, GraphQL, and deploy scripts included."
  />
</svelte:head>

<!-- ── Hero ─────────────────────────────────────────────────── -->
<section class="relative overflow-hidden bg-base-100">
  <div class="absolute inset-0 pointer-events-none">
    <div class="absolute -top-32 right-0 w-[40rem] h-[40rem] rounded-full bg-primary/10 blur-3xl"></div>
    <div class="absolute bottom-0 left-0 w-[28rem] h-[28rem] rounded-full bg-accent/10 blur-3xl"></div>
  </div>

  <div class="relative max-w-6xl mx-auto px-6 pt-20 pb-16 md:pt-28 md:pb-24">
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 lg:gap-16 items-center">
      <div class="text-center lg:text-left">
        <div class="inline-flex items-center gap-2 badge badge-primary badge-outline mb-6 font-mono text-xs tracking-wide px-4 py-3">
          <span class="size-1.5 rounded-full bg-primary animate-pulse"></span>
          production-ready starter
        </div>

        <h1 class="font-display text-5xl md:text-6xl xl:text-7xl font-extrabold tracking-tight leading-[1.05] mb-6">
          cracked<span class="text-primary">stack</span>
        </h1>

        <p class="text-lg md:text-xl text-base-content/70 max-w-xl mx-auto lg:mx-0 mb-4 leading-relaxed font-normal">
          A full-stack application template with
          <strong class="font-semibold text-base-content">SvelteKit</strong>,
          <strong class="font-semibold text-base-content">Rocket.rs</strong>,
          <strong class="font-semibold text-base-content">SurrealDB</strong>,
          and <strong class="font-semibold text-base-content">Caddy</strong> for automatic HTTPS —
          auth, GraphQL, and Docker deploy included.
        </p>
        <p class="text-base text-base-content/55 max-w-lg mx-auto lg:mx-0 mb-10 leading-relaxed">
          Skip the boilerplate. Ship a real app with TLS, cookies, and row-level security from day one.
        </p>

        <div class="flex flex-wrap gap-3 justify-center lg:justify-start">
          {#if $isAuthenticated}
            <a href="/dashboard" class="btn btn-primary btn-lg font-semibold">Go to Dashboard</a>
          {:else}
            <a href="/signup" class="btn btn-primary btn-lg font-semibold">Get Started Free</a>
            <a href="/login" class="btn btn-ghost btn-lg font-medium">Login</a>
          {/if}
          <a
            href="https://github.com/brendandonahue/cracked-stack"
            target="_blank"
            rel="noopener noreferrer"
            class="btn btn-outline btn-lg gap-2 font-medium"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="size-5" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
              <path d="M12 0C5.374 0 0 5.373 0 12c0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23A11.509 11.509 0 0 1 12 5.803c1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576C20.566 21.797 24 17.3 24 12c0-6.627-5.373-12-12-12z"/>
            </svg>
            View on GitHub
          </a>
        </div>
      </div>

      <div class="relative mx-auto w-full max-w-lg lg:max-w-none">
        <div class="absolute -inset-4 rounded-3xl bg-gradient-to-br from-primary/20 via-transparent to-accent/20 blur-2xl"></div>
        <img
          src="/images/hero.jpg"
          alt="Full-stack architecture: frontend panels, Rust core, database lattice, and HTTPS edge"
          class="relative w-full rounded-2xl shadow-2xl border border-base-content/10 object-cover aspect-video bg-base-300"
          width="1280"
          height="720"
        />
      </div>
    </div>
  </div>
</section>

<!-- ── Deploy path ──────────────────────────────────────────── -->
<section class="bg-base-200 py-24 px-6">
  <div class="max-w-6xl mx-auto">
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 lg:gap-16 items-center">
      <div>
        <p class="font-mono text-xs uppercase tracking-[0.2em] text-primary mb-3">Production path</p>
        <h2 class="font-display text-3xl md:text-4xl font-bold tracking-tight mb-4">
          Caddy on the edge. Docker underneath.
        </h2>
        <p class="text-base-content/65 text-lg leading-relaxed mb-6">
          Traffic hits <strong class="text-base-content font-semibold">Caddy</strong> first —
          automatic HTTPS certificates, HTTP→HTTPS redirects, and reverse proxying to your
          containers. nginx serves the static SvelteKit build; Rocket handles the API.
          SurrealDB stays on the private network.
        </p>
        <ul class="space-y-3 text-base-content/70">
          <li class="flex gap-3 items-start">
            <span class="mt-1 size-5 shrink-0 rounded-full bg-success/20 text-success flex items-center justify-center text-xs font-bold">✓</span>
            <span><code class="font-mono text-sm text-primary">Caddyfile.example</code> — drop-in TLS reverse proxy for one app</span>
          </li>
          <li class="flex gap-3 items-start">
            <span class="mt-1 size-5 shrink-0 rounded-full bg-success/20 text-success flex items-center justify-center text-xs font-bold">✓</span>
            <span>Compose services for nginx frontend + Rocket API</span>
          </li>
          <li class="flex gap-3 items-start">
            <span class="mt-1 size-5 shrink-0 rounded-full bg-success/20 text-success flex items-center justify-center text-xs font-bold">✓</span>
            <span><code class="font-mono text-sm text-primary">deploy.sh</code> builds, rolls out, and applies schema</span>
          </li>
        </ul>
      </div>

      <div class="relative">
        <img
          src="/images/architecture.jpg"
          alt="Request path: Caddy TLS, SvelteKit frontend, Rocket API, SurrealDB"
          class="w-full rounded-2xl shadow-xl border border-base-content/10 object-cover aspect-video bg-base-300"
          width="1280"
          height="720"
        />
        <div class="mt-4 grid grid-cols-2 sm:grid-cols-4 gap-2 text-center font-mono text-[10px] sm:text-xs uppercase tracking-wider text-base-content/55">
          <span class="rounded-lg bg-base-100 px-2 py-2 border border-base-content/10">Caddy TLS</span>
          <span class="rounded-lg bg-base-100 px-2 py-2 border border-base-content/10">SvelteKit</span>
          <span class="rounded-lg bg-base-100 px-2 py-2 border border-base-content/10">Rocket</span>
          <span class="rounded-lg bg-base-100 px-2 py-2 border border-base-content/10">SurrealDB</span>
        </div>
      </div>
    </div>
  </div>
</section>

<!-- ── Stack Overview ─────────────────────────────────────────── -->
<section class="bg-base-100 py-24 px-6">
  <div class="max-w-5xl mx-auto">
    <div class="text-center mb-16">
      <p class="font-mono text-xs uppercase tracking-[0.2em] text-primary mb-3">The stack</p>
      <h2 class="font-display text-3xl md:text-4xl font-bold tracking-tight mb-4">Everything you need to ship</h2>
      <p class="text-base-content/60 max-w-xl mx-auto text-lg leading-relaxed">
        Each piece chosen for performance, developer experience, and production reliability.
      </p>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">

      <div class="card bg-base-200 border border-base-content/5 shadow-sm hover:shadow-lg hover:border-primary/20 transition-all">
        <div class="card-body">
          <div class="w-11 h-11 rounded-xl bg-primary/15 text-primary flex items-center justify-center mb-2">
            <svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.75" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
            </svg>
          </div>
          <h3 class="card-title font-display text-lg">SvelteKit</h3>
          <p class="text-base-content/70 text-sm leading-relaxed">
            Reactive UI with Svelte 5 runes, Tailwind CSS, and DaisyUI.
            SSR optional — ships as a static SPA for Tauri.
          </p>
          <div class="card-actions mt-2">
            <span class="badge badge-ghost font-mono text-xs">SvelteKit</span>
            <span class="badge badge-ghost font-mono text-xs">Tailwind</span>
            <span class="badge badge-ghost font-mono text-xs">DaisyUI</span>
          </div>
        </div>
      </div>

      <div class="card bg-base-200 border border-base-content/5 shadow-sm hover:shadow-lg hover:border-primary/20 transition-all">
        <div class="card-body">
          <div class="w-11 h-11 rounded-xl bg-primary/15 text-primary flex items-center justify-center mb-2">
            <svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.75" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" d="M17.25 6.75 22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3-4.5 16.5" />
            </svg>
          </div>
          <h3 class="card-title font-display text-lg">Rocket.rs</h3>
          <p class="text-base-content/70 text-sm leading-relaxed">
            Type-safe async HTTP in Rust. Cookie auth, GraphQL proxy,
            rate limiting, and CORS — production-hardened.
          </p>
          <div class="card-actions mt-2">
            <span class="badge badge-ghost font-mono text-xs">Rust</span>
            <span class="badge badge-ghost font-mono text-xs">Rocket</span>
            <span class="badge badge-ghost font-mono text-xs">Async</span>
          </div>
        </div>
      </div>

      <div class="card bg-base-200 border border-base-content/5 shadow-sm hover:shadow-lg hover:border-primary/20 transition-all">
        <div class="card-body">
          <div class="w-11 h-11 rounded-xl bg-primary/15 text-primary flex items-center justify-center mb-2">
            <svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.75" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" d="M20.25 6.375c0 2.278-3.694 4.125-8.25 4.125S3.75 8.653 3.75 6.375m16.5 0c0-2.278-3.694-4.125-8.25-4.125S3.75 4.097 3.75 6.375m16.5 0v11.25c0 2.278-3.694 4.125-8.25 4.125s-8.25-1.847-8.25-4.125V6.375" />
            </svg>
          </div>
          <h3 class="card-title font-display text-lg">SurrealDB</h3>
          <p class="text-base-content/70 text-sm leading-relaxed">
            Multi-model database with row-level permissions in the schema.
            Auto-generates GraphQL from your tables.
          </p>
          <div class="card-actions mt-2">
            <span class="badge badge-ghost font-mono text-xs">SurrealDB</span>
            <span class="badge badge-ghost font-mono text-xs">GraphQL</span>
            <span class="badge badge-ghost font-mono text-xs">RBAC</span>
          </div>
        </div>
      </div>

      <div class="card bg-base-200 border border-base-content/5 shadow-sm hover:shadow-lg hover:border-primary/20 transition-all">
        <div class="card-body">
          <div class="w-11 h-11 rounded-xl bg-success/15 text-success flex items-center justify-center mb-2">
            <svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.75" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75 11.25 15 15 9.75m-3-7.036A11.959 11.959 0 0 1 3.598 6 11.99 11.99 0 0 0 3 9.749c0 5.592 3.824 10.29 9 11.623 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.285Z" />
            </svg>
          </div>
          <h3 class="card-title font-display text-lg">Caddy TLS</h3>
          <p class="text-base-content/70 text-sm leading-relaxed">
            Automatic HTTPS on the host. Caddy terminates TLS and reverse-proxies
            to nginx (static) and Rocket (API) — no certbot dance.
          </p>
          <div class="card-actions mt-2">
            <span class="badge badge-ghost font-mono text-xs">Caddy</span>
            <span class="badge badge-ghost font-mono text-xs">ACME</span>
            <span class="badge badge-ghost font-mono text-xs">HTTPS</span>
          </div>
        </div>
      </div>

      <div class="card bg-base-200 border border-base-content/5 shadow-sm hover:shadow-lg hover:border-primary/20 transition-all">
        <div class="card-body">
          <div class="w-11 h-11 rounded-xl bg-primary/15 text-primary flex items-center justify-center mb-2">
            <svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.75" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" d="M16.5 10.5V6.75a4.5 4.5 0 1 0-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 0 0 2.25-2.25v-6.75a2.25 2.25 0 0 0-2.25-2.25H6.75a2.25 2.25 0 0 0-2.25 2.25v6.75a2.25 2.25 0 0 0 2.25 2.25Z" />
            </svg>
          </div>
          <h3 class="card-title font-display text-lg">Auth Built In</h3>
          <p class="text-base-content/70 text-sm leading-relaxed">
            httpOnly cookie sessions, signup/login/logout, password reset
            with email tokens, and route guards from day one.
          </p>
          <div class="card-actions mt-2">
            <span class="badge badge-ghost font-mono text-xs">Argon2</span>
            <span class="badge badge-ghost font-mono text-xs">httpOnly</span>
            <span class="badge badge-ghost font-mono text-xs">SameSite</span>
          </div>
        </div>
      </div>

      <div class="card bg-base-200 border border-base-content/5 shadow-sm hover:shadow-lg hover:border-primary/20 transition-all">
        <div class="card-body">
          <div class="w-11 h-11 rounded-xl bg-primary/15 text-primary flex items-center justify-center mb-2">
            <svg xmlns="http://www.w3.org/2000/svg" class="size-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.75" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" d="M21.75 17.25v-.228a4.5 4.5 0 0 0-.12-1.03l-2.268-9.64a3.375 3.375 0 0 0-3.285-2.602H7.923a3.375 3.375 0 0 0-3.285 2.602l-2.268 9.64a4.5 4.5 0 0 0-.12 1.03v.228m19.5 0a3 3 0 0 1-3 3H5.25a3 3 0 0 1-3-3m19.5 0a3 3 0 0 0-3-3H5.25a3 3 0 0 0-3 3m16.5 0h.008v.008h-.008v-.008Zm-3 0h.008v.008h-.008v-.008Z" />
            </svg>
          </div>
          <h3 class="card-title font-display text-lg">Docker + Tauri</h3>
          <p class="text-base-content/70 text-sm leading-relaxed">
            Compose for SurrealDB, Rocket, and nginx. Optional Tauri desktop/mobile
            with zero frontend changes.
          </p>
          <div class="card-actions mt-2">
            <span class="badge badge-ghost font-mono text-xs">Docker</span>
            <span class="badge badge-ghost font-mono text-xs">nginx</span>
            <span class="badge badge-ghost font-mono text-xs">Tauri</span>
          </div>
        </div>
      </div>

    </div>
  </div>
</section>

<!-- ── Auth Flow ──────────────────────────────────────────────── -->
<section class="bg-base-200 py-24 px-6">
  <div class="max-w-4xl mx-auto">
    <div class="text-center mb-16">
      <p class="font-mono text-xs uppercase tracking-[0.2em] text-primary mb-3">Security</p>
      <h2 class="font-display text-3xl md:text-4xl font-bold tracking-tight mb-4">How auth works</h2>
      <p class="text-base-content/60 max-w-xl mx-auto text-lg leading-relaxed">
        No third-party auth services. Cookie sessions with SurrealDB’s native Argon2 hashing.
      </p>
    </div>

    <div class="steps steps-vertical lg:steps-horizontal w-full">
      <div class="step step-primary">
        <div class="step-content p-4 text-left">
          <h4 class="font-display font-bold">Signup / Login</h4>
          <p class="text-sm text-base-content/60 mt-1">SvelteKit calls <code class="font-mono text-xs">POST /signup</code> or <code class="font-mono text-xs">POST /login</code></p>
        </div>
      </div>
      <div class="step step-primary">
        <div class="step-content p-4 text-left">
          <h4 class="font-display font-bold">Token Set</h4>
          <p class="text-sm text-base-content/60 mt-1">Rocket issues a SurrealDB token as an httpOnly Secure cookie</p>
        </div>
      </div>
      <div class="step step-primary">
        <div class="step-content p-4 text-left">
          <h4 class="font-display font-bold">Authenticated Requests</h4>
          <p class="text-sm text-base-content/60 mt-1">Every <code class="font-mono text-xs">apiFetch()</code> call sends the cookie automatically</p>
        </div>
      </div>
      <div class="step step-primary">
        <div class="step-content p-4 text-left">
          <h4 class="font-display font-bold">DB Permissions</h4>
          <p class="text-sm text-base-content/60 mt-1">SurrealDB enforces row-level access via <code class="font-mono text-xs">$auth.id</code></p>
        </div>
      </div>
    </div>
  </div>
</section>

<!-- ── CTA ───────────────────────────────────────────────────── -->
<section class="relative overflow-hidden bg-primary text-primary-content py-20 px-6 text-center">
  <div class="absolute inset-0 opacity-20 pointer-events-none">
    <img src="/images/logo.jpg" alt="" class="w-full h-full object-cover scale-150 blur-sm" aria-hidden="true" />
  </div>
  <div class="relative max-w-2xl mx-auto">
    <img
      src="/images/logo.jpg"
      alt=""
      class="w-16 h-16 rounded-2xl mx-auto mb-6 shadow-lg border border-primary-content/20 object-cover"
      width="128"
      height="128"
      aria-hidden="true"
    />
    <h2 class="font-display text-3xl md:text-4xl font-bold tracking-tight mb-4">Ready to build?</h2>
    <p class="text-primary-content/85 max-w-md mx-auto mb-8 text-lg leading-relaxed">
      Clone the repo, run the seed script, point Caddy at your hosts, and ship.
    </p>
    <div class="flex flex-wrap gap-4 justify-center">
      <a href="/signup" class="btn btn-lg bg-white text-primary hover:bg-white/90 border-0 font-bold">
        Create an Account
      </a>
      <a
        href="https://github.com/brendandonahue/cracked-stack"
        target="_blank"
        rel="noopener noreferrer"
        class="btn btn-lg btn-outline border-white text-white hover:bg-white hover:text-primary font-semibold"
      >
        Read the Docs
      </a>
    </div>
  </div>
</section>

<!-- ── Footer ────────────────────────────────────────────────── -->
<footer class="bg-base-200 border-t border-base-content/5 py-10 px-6">
  <div class="max-w-5xl mx-auto flex flex-col sm:flex-row items-center justify-between gap-4">
    <div class="flex items-center gap-3">
      <img src="/images/logo.jpg" alt="crackedstack" class="w-8 h-8 rounded-lg object-cover" width="32" height="32" />
      <span class="font-display font-semibold tracking-tight">
        cracked<span class="text-primary">stack</span>
      </span>
    </div>
    <p class="text-sm text-base-content/50 font-mono">
      MIT License · SvelteKit · Rocket · SurrealDB · Caddy
    </p>
  </div>
</footer>
