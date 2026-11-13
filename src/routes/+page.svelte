<script lang="ts">
  import { isAuthenticated } from '$lib/stores/auth';
  import LandingFooter from '$lib/components/LandingFooter.svelte';
  import Hover3D from '$lib/components/Hover3D.svelte';
  import type { Component } from 'svelte';

  import ArrowRight from '@lucide/svelte/icons/arrow-right';
  import BadgeCheck from '@lucide/svelte/icons/badge-check';
  import Boxes from '@lucide/svelte/icons/boxes';
  import Braces from '@lucide/svelte/icons/braces';
  import Cookie from '@lucide/svelte/icons/cookie';
  import CreditCard from '@lucide/svelte/icons/credit-card';
  import Database from '@lucide/svelte/icons/database';
  import Fingerprint from '@lucide/svelte/icons/fingerprint';
  import FolderLock from '@lucide/svelte/icons/folder-lock';
  import Gauge from '@lucide/svelte/icons/gauge';
  import Globe from '@lucide/svelte/icons/globe';
  import KeyRound from '@lucide/svelte/icons/key-round';
  import Layers from '@lucide/svelte/icons/layers';
  import Lock from '@lucide/svelte/icons/lock';
  import LockKeyhole from '@lucide/svelte/icons/lock-keyhole';
  import Mail from '@lucide/svelte/icons/mail';
  import Menu from '@lucide/svelte/icons/menu';
  import MonitorSmartphone from '@lucide/svelte/icons/monitor-smartphone';
  import Receipt from '@lucide/svelte/icons/receipt';
  import ScrollText from '@lucide/svelte/icons/scroll-text';
  import Server from '@lucide/svelte/icons/server';
  import Shield from '@lucide/svelte/icons/shield';
  import ShieldAlert from '@lucide/svelte/icons/shield-alert';
  import ShieldCheck from '@lucide/svelte/icons/shield-check';
  import Terminal from '@lucide/svelte/icons/terminal';
  import Users from '@lucide/svelte/icons/users';
  import Waypoints from '@lucide/svelte/icons/waypoints';
  import Webhook from '@lucide/svelte/icons/webhook';
  import Zap from '@lucide/svelte/icons/zap';

  const repo = 'https://github.com/brendandonahue/cracked-stack';

  const nav = [
    { href: '#why', label: 'Why' },
    { href: '#stack', label: 'Stack' },
    { href: '#path', label: 'Path' },
    { href: '#security', label: 'Security' },
    { href: '#box', label: 'In the box' },
  ];

  const pills: { icon: Component; label: string }[] = [
    { icon: Layers, label: 'SvelteKit' },
    { icon: Zap, label: 'Svelte 5' },
    { icon: Boxes, label: 'Tailwind' },
    { icon: BadgeCheck, label: 'DaisyUI' },
    { icon: Terminal, label: 'Rust' },
    { icon: Server, label: 'Rocket' },
    { icon: Database, label: 'SurrealDB' },
    { icon: Braces, label: 'GraphQL' },
    { icon: KeyRound, label: 'Argon2' },
    { icon: CreditCard, label: 'Stripe' },
    { icon: Globe, label: 'Caddy' },
    { icon: MonitorSmartphone, label: 'Tauri' },
  ];

  const outcomes = [
    {
      icon: Fingerprint,
      tone: 'text-primary bg-primary/15',
      title: 'People can sign in, and stay signed in',
      plain: 'An account is created in the database. The browser holds a cookie it cannot read. Tomorrow they are still signed in, until that cookie expires.',
      detail: 'Signup and login call SurrealDB’s record access. The token comes back as an httpOnly, Secure, SameSite=Strict cookie with a 24-hour lifetime.',
    },
    {
      icon: ShieldCheck,
      tone: 'text-secondary bg-secondary/15',
      title: 'A person only sees their own records',
      plain: 'The rule is not a filter someone remembered to add in the UI. The database refuses the row.',
      detail: 'Tables are SCHEMAFULL. Item reads and writes require created_by = $auth.id, unless the session role is admin. Files add a public flag.',
    },
    {
      icon: Lock,
      tone: 'text-accent bg-accent/15',
      title: 'The public site has a real lock',
      plain: 'Visitors arrive on HTTPS. Plain HTTP is sent there. The certificate renews without a weekend of paperwork.',
      detail: 'Caddy terminates TLS with ACME, sets HSTS, nosniff, and X-Frame-Options DENY, then reverse-proxies to nginx and Rocket.',
    },
    {
      icon: Server,
      tone: 'text-success bg-success/15',
      title: 'Putting it on a server is a script',
      plain: 'One command builds the interface, builds the API, and restarts the containers. The database is not exposed on the public internet.',
      detail: 'deploy.sh rolls the Compose stack. nginx serves the static build. Rocket listens for the API. SurrealDB stays on the private network.',
    },
    {
      icon: CreditCard,
      tone: 'text-warning bg-warning/15',
      title: 'It is ready to take a payment',
      plain: 'A customer picks a plan, pays on a page Stripe hosts, and lands back in the app with a receipt. Card numbers never touch your server. The receipt only appears once Stripe says the money arrived.',
      detail: 'A small custom Stripe integration, no SDK: Rocket creates the Checkout Session with prices it owns, and a webhook verified with HMAC-SHA256 writes the payment row. The payment table is read-only to users, so a receipt cannot be forged.',
    },
  ];

  const shipped: { icon: Component; title: string; detail: string }[] = [
    { icon: Cookie, title: 'Cookie sessions', detail: 'httpOnly, Secure, SameSite=Strict. JavaScript on the page never sees the token.' },
    { icon: KeyRound, title: 'Argon2 passwords', detail: 'Hashed with crypto::argon2 inside SurrealDB on signup, and again when a password is reset.' },
    { icon: Shield, title: 'Row-level permissions', detail: 'Ownership is a PERMISSIONS clause. Admins are the exception, written in the same clause.' },
    { icon: Mail, title: 'Password reset', detail: 'A short-lived token is stored, emailed over SMTP, then marked used so it cannot be replayed.' },
    { icon: Braces, title: 'Generated GraphQL', detail: 'DEFINE CONFIG GRAPHQL AUTO. Rocket proxies the query with the caller’s session attached.' },
    { icon: FolderLock, title: 'Uploads', detail: 'Multipart upload. Private files are served only to the owner or an admin. Public files are explicit.' },
    { icon: CreditCard, title: 'Stripe Checkout', detail: 'A custom integration with no SDK. Rocket opens a hosted Checkout Session, so card details never reach your server.' },
    { icon: Webhook, title: 'Signed webhooks', detail: 'Stripe events are verified with HMAC-SHA256 and a five-minute window. A repeated delivery updates one payment, not two.' },
    { icon: Receipt, title: 'Payments you cannot forge', detail: 'Prices live on the server. The payment table is written only by the webhook, and users can read just their own rows.' },
    { icon: Gauge, title: 'Rate limits', detail: 'Rocket Governor sits on the auth routes, so a script cannot hammer login or signup.' },
    { icon: Waypoints, title: 'Route guards', detail: 'The SvelteKit layout checks the session on every navigation and sends strangers to login.' },
    { icon: ShieldAlert, title: 'No self-serve admin', detail: 'Public signup cannot grant the admin role. That role is set in the database.' },
    { icon: ScrollText, title: 'Toasts and forms', detail: 'A small toast store, login, signup, forgot-password, and reset-password screens are already there.' },
    { icon: Globe, title: 'Caddy on the edge', detail: 'Automatic certificates, HTTP to HTTPS, compression, and security headers in Caddyfile.example.' },
    { icon: MonitorSmartphone, title: 'One UI, more shells', detail: 'The frontend builds as a static SPA. Tauri can wrap the same build for desktop and mobile.' },
  ];

  const stations = [
    { k: '01', title: 'Browser', text: 'The SvelteKit app calls the API with credentials included. The cookie rides along. The page never handles the token.' },
    { k: '02', title: 'Caddy', text: 'TLS ends here. Certificates come from Let’s Encrypt. HTTP is redirected. HSTS is set. The containers are not on the open internet.' },
    { k: '03', title: 'nginx + Rocket', text: 'nginx serves the static build. Rocket serves login, GraphQL, uploads, Stripe checkout and webhooks, and the rest of the API, with CORS set for the environment.' },
    { k: '04', title: 'SurrealDB', text: 'Queries run as the signed-in record. Row permissions apply. GraphQL is generated from the tables.' },
  ];

  const steps = [
    { n: '1', title: 'Clone and configure', text: 'Node 20, Rust, and SurrealDB 2. Point the frontend at the API and the API at the database.' },
    { n: '2', title: 'Import the schema', text: 'schema.surql creates user, item, password reset, file, and payment. seed.surql adds a demo admin and a demo user.' },
    { n: '3', title: 'Run both sides', text: 'cargo run serves Rocket on port 8000. npm run dev serves SvelteKit on port 5173. Sign in and open the items screen.' },
    { n: '4', title: 'Replace item, then ship', text: 'The demo resource is the only thing that should become your domain. Then Caddyfile.example and deploy.sh put it up.' },
  ];

  const faqs = [
    {
      q: 'Do I have to understand every layer before I can change the product?',
      a: 'No. The demo resource is one table, item, plus the screens and helpers that talk to it. Replace that with your domain. Sessions, permissions, uploads, and the deploy path stay.',
    },
    {
      q: 'Where does a password go?',
      a: 'Into SurrealDB, hashed with Argon2 by the database itself. The API never stores the raw password. The session that comes back is a cookie the page’s JavaScript cannot read.',
    },
    {
      q: 'Can one account read another account’s records?',
      a: 'Not by default. An item is visible and editable only by the user who created it, or by an admin. That check runs in the database, using the signed-in record, on every query.',
    },
    {
      q: 'How does the site get HTTPS?',
      a: 'Caddy asks Let’s Encrypt for a certificate, redirects HTTP to HTTPS, and renews on its own. You set an email and two hostnames. There is no separate certbot step.',
    },
    {
      q: 'Can this become a desktop or phone app?',
      a: 'The interface is built as a static single-page app, which is what Tauri wraps. The same screens run in the browser, and in the desktop and mobile shells, without a second frontend.',
    },
    {
      q: 'Can it take payments?',
      a: 'Yes. It ships with a small custom Stripe integration. Rocket creates a hosted Checkout Session from plans defined on the server, and a signed webhook records the payment once Stripe confirms it. Add your Stripe keys and open the Payments screen to try it in test mode. Replace the three demo plans with what you sell.',
    },
    {
      q: 'What is deliberately left for me?',
      a: 'Your tables, your screens, and your rules beyond the two roles already here: user and admin. The starter is opinionated about the platform and quiet about your domain.',
    },
  ];
</script>

<svelte:head>
  <title>crackedstack — the full stack, already finished</title>
  <meta
    name="description"
    content="SvelteKit, Rocket, and SurrealDB with cookie sessions, Argon2, row-level permissions, file uploads, a custom Stripe payments integration, and Caddy automatic HTTPS. Clone it and replace the demo model."
  />
</svelte:head>

<div class="bg-base-100 text-base-content overflow-x-clip">
  <header class="sticky top-0 z-50 border-b border-base-content/10 bg-base-100/80 backdrop-blur-md">
    <div class="max-w-6xl mx-auto px-4 sm:px-6 h-16 flex items-center gap-4">
      <a href="/" class="flex items-center gap-2.5 font-display font-bold tracking-tight text-lg shrink-0">
        <img src="/images/logo.jpg" alt="" width="32" height="32" class="w-8 h-8 rounded-lg object-cover ring-1 ring-base-content/10" />
        cracked<span class="text-primary">stack</span>
      </a>

      <nav class="hidden md:flex items-center gap-1 ml-4" aria-label="Page">
        {#each nav as link (link.href)}
          <a href={link.href} class="btn btn-ghost btn-sm font-medium text-base-content/80">{link.label}</a>
        {/each}
      </nav>

      <div class="ml-auto flex items-center gap-2">
        <a href={repo} target="_blank" rel="noopener noreferrer" class="btn btn-ghost btn-sm btn-square hidden md:inline-flex" aria-label="GitHub repository">
          <svg xmlns="http://www.w3.org/2000/svg" class="size-5" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
            <path d="M12 0C5.374 0 0 5.373 0 12c0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23A11.509 11.509 0 0 1 12 5.803c1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576C20.566 21.797 24 17.3 24 12c0-6.627-5.373-12-12-12z"/>
          </svg>
        </a>
        {#if $isAuthenticated}
          <a href="/dashboard" class="btn btn-primary btn-sm font-semibold hidden sm:inline-flex">Dashboard</a>
        {:else}
          <a href="/login" class="btn btn-ghost btn-sm hidden md:inline-flex">Login</a>
          <a href="/signup" class="btn btn-primary btn-sm font-semibold cs-sheen hidden sm:inline-flex">Get started</a>
        {/if}

        <div class="dropdown dropdown-end md:hidden">
          <button type="button" tabindex="0" class="btn btn-ghost btn-sm btn-square" aria-label="Open menu">
            <Menu size={20} />
          </button>
          <ul class="dropdown-content menu bg-base-200 rounded-box w-52 mt-2 p-2 shadow-xl border border-base-content/10 z-50">
            {#each nav as link (link.href)}
              <li><a href={link.href}>{link.label}</a></li>
            {/each}
            {#if $isAuthenticated}
              <li><a href="/dashboard">Dashboard</a></li>
            {:else}
              <li><a href="/signup">Get started</a></li>
              <li><a href="/login">Login</a></li>
            {/if}
            <li><a href={repo} target="_blank" rel="noopener noreferrer">GitHub</a></li>
          </ul>
        </div>
      </div>
    </div>
  </header>

  <section class="relative overflow-hidden">
    <div class="cs-aurora absolute inset-0 pointer-events-none" aria-hidden="true"></div>
    <div class="cs-grid absolute inset-0 pointer-events-none" aria-hidden="true"></div>

    <div class="relative max-w-6xl mx-auto px-6 pt-16 pb-20 md:pt-24 md:pb-28">
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 lg:gap-16 items-center min-w-0">
        <div class="min-w-0">
          <p class="cs-enter inline-flex items-center gap-2 badge badge-primary badge-outline font-mono text-xs tracking-wide px-3 py-3 mb-6">
            <span class="size-1.5 rounded-full bg-primary animate-pulse"></span>
            production-shaped starter
          </p>

          <h1 class="cs-enter font-display text-5xl sm:text-6xl xl:text-7xl font-extrabold tracking-tight leading-[1.02] mb-6" style="animation-delay: 70ms">
            The full stack,<br />
            <span class="text-primary">already finished.</span>
          </h1>

          <p class="cs-enter text-lg md:text-xl text-base-content/75 max-w-xl leading-relaxed mb-4" style="animation-delay: 140ms">
            Sign-in, permissions, files, payments, and HTTPS are the starting point — not the weekend you still owe yourself.
            <strong class="font-semibold text-base-content">SvelteKit</strong> on the screen,
            <strong class="font-semibold text-base-content">Rocket</strong> in the middle,
            <strong class="font-semibold text-base-content">SurrealDB</strong> underneath,
            <strong class="font-semibold text-base-content">Caddy</strong> on the public edge.
          </p>
          <p class="cs-enter text-base text-base-content/60 max-w-lg leading-relaxed mb-8" style="animation-delay: 200ms">
            Replace the demo model. Keep the parts that are expensive to get right, and easy to explain to someone who will never open the repository.
          </p>

          <div class="cs-enter flex flex-wrap gap-3" style="animation-delay: 260ms">
            {#if $isAuthenticated}
              <a href="/dashboard" class="btn btn-primary btn-lg font-semibold cs-sheen">
                Go to dashboard
                <ArrowRight size={18} />
              </a>
            {:else}
              <a href="/signup" class="btn btn-primary btn-lg font-semibold cs-sheen">
                Create an account
                <ArrowRight size={18} />
              </a>
              <a href="/login" class="btn btn-ghost btn-lg">Login</a>
            {/if}
            <a href="#path" class="btn btn-outline btn-lg">See the request path</a>
          </div>
        </div>

        <div class="cs-enter cs-stage min-w-0 w-full" style="animation-delay: 180ms">
          <Hover3D>
            <img
              src="/images/hero-stack.jpg"
              alt="Three stacked layers cracked by a bolt of light: the interface, the API, and the database"
              class="w-full max-w-full rounded-2xl border border-base-content/10 shadow-2xl object-cover aspect-video bg-base-300"
              width="1280"
              height="720"
            />
          </Hover3D>

          <ul class="grid grid-cols-2 gap-2 -mt-6 mx-3 sm:mx-6 relative z-10">
            {#each [
              { icon: Cookie, label: 'httpOnly cookie' },
              { icon: KeyRound, label: 'Argon2 in the DB' },
              { icon: Shield, label: 'Row-level rules' },
              { icon: LockKeyhole, label: 'Automatic TLS' },
            ] as chip (chip.label)}
              {@const Icon = chip.icon}
              <li class="flex items-center gap-2 rounded-xl border border-base-content/10 bg-base-100/90 backdrop-blur px-3 py-2 text-sm shadow-lg">
                <Icon size={16} class="text-primary shrink-0" />
                <span class="font-medium">{chip.label}</span>
              </li>
            {/each}
          </ul>
        </div>
      </div>
    </div>
  </section>

  <section class="border-y border-base-content/10 bg-base-200/60" aria-label="Technologies">
    <div class="max-w-6xl mx-auto px-6 py-5 flex flex-wrap justify-center gap-2">
      {#each pills as pill (pill.label)}
        {@const Icon = pill.icon}
        <span class="inline-flex items-center gap-1.5 rounded-full border border-base-content/10 bg-base-100 px-3 py-1.5 text-xs font-mono text-base-content/70">
          <Icon size={14} class="text-primary" />
          {pill.label}
        </span>
      {/each}
    </div>
  </section>

  <section id="why" class="py-20 md:py-28 px-6">
    <div class="max-w-6xl mx-auto">
      <div class="reveal max-w-2xl mb-12">
        <p class="font-mono text-xs uppercase tracking-[0.2em] text-primary mb-3">Two readers, one repository</p>
        <h2 class="font-display text-3xl md:text-5xl font-bold tracking-tight mb-4">Built so a developer wants to run it, and a non-developer can trust it.</h2>
        <p class="text-lg text-base-content/65 leading-relaxed">
          The interesting claims are checkable. Each one below is a file in this repository, not a slogan on top of a blank app.
        </p>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-5">
        <article class="reveal card bg-base-200 border border-base-content/10 shadow-sm">
          <div class="card-body p-7 md:p-8">
            <div class="flex items-center gap-3 mb-4">
              <span class="cs-icon w-11 h-11 rounded-xl bg-primary/15 text-primary flex items-center justify-center">
                <Terminal size={22} />
              </span>
              <h3 class="font-display text-2xl font-bold">If you write the code</h3>
            </div>
            <ul class="space-y-3 text-base-content/75">
              <li class="flex gap-3"><BadgeCheck size={18} class="text-primary mt-0.5 shrink-0" /> Svelte 5 runes, Tailwind, and this app’s DaisyUI theme. The build is a static SPA, so Tauri can wrap it.</li>
              <li class="flex gap-3"><BadgeCheck size={18} class="text-primary mt-0.5 shrink-0" /> A typed Rocket API: cookie auth, a GraphQL proxy, uploads, a custom Stripe checkout with signed webhooks, CORS, and a governor on the auth routes.</li>
              <li class="flex gap-3"><BadgeCheck size={18} class="text-primary mt-0.5 shrink-0" /> SurrealDB with a schema, record access, and GraphQL generated from the tables. Permissions live next to the fields.</li>
              <li class="flex gap-3"><BadgeCheck size={18} class="text-primary mt-0.5 shrink-0" /> Compose for nginx and Rocket, a Caddyfile for the host, and a deploy script that rolls them forward.</li>
            </ul>
          </div>
        </article>

        <article class="reveal card bg-base-200 border border-primary/25 shadow-sm">
          <div class="card-body p-7 md:p-8">
            <div class="flex items-center gap-3 mb-4">
              <span class="cs-icon w-11 h-11 rounded-xl bg-secondary/15 text-secondary flex items-center justify-center">
                <Users size={22} />
              </span>
              <h3 class="font-display text-2xl font-bold">If you have to stand behind it</h3>
            </div>
            <ul class="space-y-3 text-base-content/75">
              <li class="flex gap-3"><BadgeCheck size={18} class="text-secondary mt-0.5 shrink-0" /> Customers get an account. Their password is not left sitting in the browser as something a script can copy.</li>
              <li class="flex gap-3"><BadgeCheck size={18} class="text-secondary mt-0.5 shrink-0" /> One customer does not see another customer’s records. An administrator can. That split is enforced in the database.</li>
              <li class="flex gap-3"><BadgeCheck size={18} class="text-secondary mt-0.5 shrink-0" /> The public address is HTTPS, with the certificate handled for you, and ordinary HTTP bounced over to it.</li>
              <li class="flex gap-3"><BadgeCheck size={18} class="text-secondary mt-0.5 shrink-0" /> Shipping is a script you can hand to someone. The database stays off the public network.</li>
              <li class="flex gap-3"><BadgeCheck size={18} class="text-secondary mt-0.5 shrink-0" /> Customers can pay by card through Stripe. The card number never reaches your server, and a payment is recorded only after Stripe confirms it.</li>
            </ul>
          </div>
        </article>
      </div>
    </div>
  </section>

  <section class="bg-base-200 py-20 md:py-28 px-6">
    <div class="max-w-6xl mx-auto">
      <div class="reveal max-w-2xl mb-12">
        <p class="font-mono text-xs uppercase tracking-[0.2em] text-primary mb-3">Day one</p>
        <h2 class="font-display text-3xl md:text-5xl font-bold tracking-tight">What is already true when you clone it</h2>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
        {#each outcomes as item, i (item.title)}
          {@const Icon = item.icon}
          <article class="reveal group card bg-base-100 border border-base-content/10 shadow-sm hover:border-primary/30 transition-colors {outcomes.length % 2 === 1 && i === outcomes.length - 1 ? 'md:col-span-2' : ''}">
            <div class="card-body p-7">
              <span class="cs-icon w-11 h-11 rounded-xl {item.tone} flex items-center justify-center mb-3">
                <Icon size={22} />
              </span>
              <h3 class="font-display text-xl font-bold mb-2">{item.title}</h3>
              <p class="text-base-content/80 leading-relaxed mb-3">{item.plain}</p>
              <p class="text-sm text-base-content/55 leading-relaxed border-t border-base-content/10 pt-3">{item.detail}</p>
            </div>
          </article>
        {/each}
      </div>
    </div>
  </section>

  <section id="stack" class="py-20 md:py-28 px-6">
    <div class="max-w-6xl mx-auto">
      <div class="reveal max-w-2xl mb-16">
        <p class="font-mono text-xs uppercase tracking-[0.2em] text-primary mb-3">The stack</p>
        <h2 class="font-display text-3xl md:text-5xl font-bold tracking-tight mb-4">Three layers, chosen for a reason</h2>
        <p class="text-lg text-base-content/65 leading-relaxed">
          Each one earns its place. Fast to work in, strict where mistakes get expensive, and boring to operate once it is up.
        </p>
      </div>

      <div class="space-y-20">
        <article class="reveal grid grid-cols-1 lg:grid-cols-2 gap-10 items-center">
          <div>
            <p class="font-mono text-sm text-primary mb-2">01 — Interface</p>
            <h3 class="font-display text-3xl font-bold tracking-tight mb-4 flex items-center gap-3">
              <Layers size={28} class="text-primary" /> SvelteKit
            </h3>
            <p class="text-base-content/75 leading-relaxed mb-4">
              Svelte 5 runes, Tailwind, and DaisyUI. The theme on this page — <span class="font-mono text-primary">cracked</span> — is the same theme the signed-in app uses, so a login screen and a dashboard do not look like different products.
            </p>
            <p class="text-base-content/75 leading-relaxed mb-5">
              The production build is static. That is what nginx serves, and what Tauri wraps for desktop and mobile, without a second interface to maintain.
            </p>
            <div class="flex flex-wrap gap-2">
              <span class="badge badge-outline font-mono">SvelteKit 2</span>
              <span class="badge badge-outline font-mono">Svelte 5</span>
              <span class="badge badge-outline font-mono">Tailwind</span>
              <span class="badge badge-outline font-mono">DaisyUI</span>
            </div>
          </div>
          <img
            src="/images/devices.jpg"
            alt="A laptop and a phone showing the same abstract interface, the one frontend this stack ships"
            class="w-full rounded-2xl border border-base-content/10 shadow-xl object-cover aspect-3/2 bg-base-300"
            width="1200"
            height="800"
          />
        </article>

        <article class="reveal grid grid-cols-1 lg:grid-cols-2 gap-10 items-center">
          <div class="lg:order-2">
            <p class="font-mono text-sm text-secondary mb-2">02 — API</p>
            <h3 class="font-display text-3xl font-bold tracking-tight mb-4 flex items-center gap-3">
              <Terminal size={28} class="text-secondary" /> Rocket.rs
            </h3>
            <p class="text-base-content/75 leading-relaxed mb-4">
              The HTTP boundary is Rust. Routes are explicit: login, signup, logout, profile, password reset, a GraphQL proxy, and file upload, download, and delete.
            </p>
            <p class="text-base-content/75 leading-relaxed mb-5">
              An auth guard reads the cookie and checks it with SurrealDB. If the token is missing or expired, the API answers 401 and the interface sends the person to login. Rate limits sit on the routes people abuse first.
            </p>
            <div class="flex flex-wrap gap-2">
              <span class="badge badge-outline font-mono">Rust</span>
              <span class="badge badge-outline font-mono">Rocket</span>
              <span class="badge badge-outline font-mono">Cookie guard</span>
              <span class="badge badge-outline font-mono">CORS</span>
            </div>
          </div>
          <div class="lg:order-1 rounded-2xl border border-base-content/10 bg-base-200 shadow-xl overflow-hidden">
            <div class="flex items-center gap-2 px-4 py-3 border-b border-base-content/10 bg-base-300/50">
              <span class="size-2.5 rounded-full bg-error/80"></span>
              <span class="size-2.5 rounded-full bg-warning/80"></span>
              <span class="size-2.5 rounded-full bg-success/80"></span>
              <span class="ml-2 font-mono text-xs text-base-content/50">server/src/main.rs</span>
            </div>
            <pre class="p-5 text-sm leading-relaxed overflow-x-auto"><code><span class="text-base-content/40">// the session the browser is allowed to hold</span>
Cookie::build(("auth_token", token))
  .<span class="text-primary">http_only</span>(true)
  .<span class="text-primary">secure</span>(true)
  .<span class="text-secondary">same_site</span>(SameSite::Strict)
  .path("/")
  .<span class="text-accent">max_age</span>(Duration::hours(24))</code></pre>
          </div>
        </article>

        <article class="reveal grid grid-cols-1 lg:grid-cols-2 gap-10 items-center">
          <div>
            <p class="font-mono text-sm text-accent mb-2">03 — Data</p>
            <h3 class="font-display text-3xl font-bold tracking-tight mb-4 flex items-center gap-3">
              <Database size={28} class="text-accent" /> SurrealDB
            </h3>
            <p class="text-base-content/75 leading-relaxed mb-4">
              One database covers the shapes you actually use: records, relationships, and a schema that rejects junk. GraphQL is generated from those tables, so the API surface does not drift away from the data.
            </p>
            <p class="text-base-content/75 leading-relaxed mb-5">
              Passwords are hashed in the database. Permissions are clauses on the table. A query runs as the signed-in person, which is the whole point of putting the rule there instead of in a handler someone might skip.
            </p>
            <div class="flex flex-wrap gap-2">
              <span class="badge badge-outline font-mono">Multi-model</span>
              <span class="badge badge-outline font-mono">SCHEMAFULL</span>
              <span class="badge badge-outline font-mono">GraphQL AUTO</span>
              <span class="badge badge-outline font-mono">$auth.id</span>
            </div>
          </div>
          <img
            src="/images/lattice.jpg"
            alt="Documents, connected nodes, and database discs joined into one lattice"
            class="w-full rounded-2xl border border-base-content/10 shadow-xl object-cover aspect-video bg-base-300"
            width="1280"
            height="720"
          />
        </article>
      </div>
    </div>
  </section>

  <section id="path" class="relative bg-base-200 py-20 md:py-28 px-6 overflow-hidden">
    <div class="max-w-6xl mx-auto">
      <div class="reveal grid grid-cols-1 lg:grid-cols-2 gap-10 items-end mb-12">
        <div>
          <p class="font-mono text-xs uppercase tracking-[0.2em] text-primary mb-3">Production path</p>
          <h2 class="font-display text-3xl md:text-5xl font-bold tracking-tight mb-4">A request has somewhere honest to go</h2>
          <p class="text-lg text-base-content/65 leading-relaxed">
            Traffic hits Caddy first. The interface and the API are separate hosts behind it. The database is not one of those hosts.
          </p>
        </div>
        <img
          src="/images/edge.jpg"
          alt="A lit gateway opening onto quiet servers, the public edge in front of the private machines"
          class="w-full rounded-2xl border border-base-content/10 shadow-xl object-cover aspect-video bg-base-300"
          width="1280"
          height="720"
        />
      </div>

      <ol class="relative grid grid-cols-1 md:grid-cols-4 gap-4">
        <div class="hidden md:block absolute top-9 left-[8%] right-[8%] h-px bg-linear-to-r from-primary via-accent to-secondary" aria-hidden="true">
          <span class="cs-comet absolute top-1/2 -translate-y-1/2 size-2 rounded-full bg-primary shadow-[0_0_12px_var(--color-primary)]"></span>
        </div>
        {#each stations as station (station.k)}
          <li class="reveal relative rounded-2xl border border-base-content/10 bg-base-100 p-5 shadow-sm">
            <p class="font-mono text-xs text-primary mb-2">{station.k}</p>
            <h3 class="font-display text-lg font-bold mb-2">{station.title}</h3>
            <p class="text-sm text-base-content/65 leading-relaxed">{station.text}</p>
          </li>
        {/each}
      </ol>

      <div class="reveal mt-8 rounded-2xl border border-base-content/10 bg-base-100 p-5 md:p-6 font-mono text-sm overflow-x-auto">
        <p class="text-base-content/40 mb-2"># the shape of a first deploy</p>
        <p><span class="text-primary">$</span> cp Caddyfile.example /path/to/Caddyfile</p>
        <p class="text-base-content/50"># CADDY_EMAIL, SVELTE_HOST, ROCKET_HOST</p>
        <p><span class="text-primary">$</span> ./deploy.sh</p>
      </div>
    </div>
  </section>

  <section id="security" class="py-20 md:py-28 px-6">
    <div class="max-w-6xl mx-auto grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
      <div class="reveal">
        <img
          src="/images/security.jpg"
          alt="A sealed capsule holding a glowing token, with a shield of light behind it"
          class="w-full rounded-2xl border border-base-content/10 shadow-xl object-cover aspect-video bg-base-300"
          width="1280"
          height="720"
        />
      </div>
      <div class="reveal">
        <p class="font-mono text-xs uppercase tracking-[0.2em] text-primary mb-3">Security</p>
        <h2 class="font-display text-3xl md:text-5xl font-bold tracking-tight mb-4">The session is a cookie. The rule is a clause.</h2>
        <p class="text-base-content/70 leading-relaxed mb-6">
          There is no third-party login service in the middle. SurrealDB issues the token, Rocket sets the cookie, and the database applies the permission when the query runs. Password reset uses its own short-lived table, emailed through SMTP.
        </p>

        <ol class="steps steps-vertical mb-6">
          <li class="step step-primary">
            <div class="text-left pb-4">
              <p class="font-display font-bold">Signup or login</p>
              <p class="text-sm text-base-content/60">The page posts to Rocket. SurrealDB’s record access hashes with Argon2 and signs the session.</p>
            </div>
          </li>
          <li class="step step-primary">
            <div class="text-left pb-4">
              <p class="font-display font-bold">Cookie, not local storage</p>
              <p class="text-sm text-base-content/60">httpOnly, Secure, SameSite=Strict, 24 hours. A script on the page cannot read it, and another site cannot send it.</p>
            </div>
          </li>
          <li class="step step-primary">
            <div class="text-left pb-4">
              <p class="font-display font-bold">Every later request</p>
              <p class="text-sm text-base-content/60">apiFetch sends the cookie. The AuthUser guard checks it. A bad token is a 401, and the layout redirects.</p>
            </div>
          </li>
          <li class="step step-primary">
            <div class="text-left">
              <p class="font-display font-bold">The row decides</p>
              <p class="text-sm text-base-content/60">SurrealDB evaluates $auth.id and $auth.role. The UI does not get a vote.</p>
            </div>
          </li>
        </ol>

        <pre class="rounded-2xl bg-base-200 border border-base-content/10 p-5 text-sm overflow-x-auto leading-relaxed"><code>DEFINE TABLE item SCHEMAFULL
  PERMISSIONS
    FOR select
      WHERE created_by = $auth.id
         OR $auth.role = "admin"
    FOR update, delete
      WHERE created_by = $auth.id
         OR $auth.role = "admin";</code></pre>
      </div>
    </div>
  </section>

  <section id="box" class="bg-base-200 py-20 md:py-28 px-6">
    <div class="max-w-6xl mx-auto">
      <div class="reveal max-w-2xl mb-12">
        <p class="font-mono text-xs uppercase tracking-[0.2em] text-primary mb-3">In the box</p>
        <h2 class="font-display text-3xl md:text-5xl font-bold tracking-tight mb-4">The list you would otherwise build first</h2>
        <p class="text-lg text-base-content/65 leading-relaxed">
          Twelve pieces that are already wired. None of them are a sample button that console.logs.
        </p>
      </div>

      <ul class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each shipped as item (item.title)}
          {@const Icon = item.icon}
          <li class="reveal group flex gap-4 rounded-2xl border border-base-content/10 bg-base-100 p-5 hover:border-primary/30 transition-colors">
            <span class="cs-icon w-10 h-10 shrink-0 rounded-xl bg-primary/15 text-primary flex items-center justify-center">
              <Icon size={20} />
            </span>
            <div>
              <h3 class="font-display font-bold mb-1">{item.title}</h3>
              <p class="text-sm text-base-content/65 leading-relaxed">{item.detail}</p>
            </div>
          </li>
        {/each}
      </ul>
    </div>
  </section>

  <section id="start" class="py-20 md:py-28 px-6">
    <div class="max-w-6xl mx-auto">
      <div class="reveal max-w-2xl mb-12">
        <p class="font-mono text-xs uppercase tracking-[0.2em] text-primary mb-3">First hour</p>
        <h2 class="font-display text-3xl md:text-5xl font-bold tracking-tight">From clone to a signed-in app</h2>
      </div>
      <ol class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {#each steps as step (step.n)}
          <li class="reveal rounded-2xl border border-base-content/10 bg-base-200 p-6">
            <span class="font-display text-4xl font-extrabold text-primary/80">{step.n}</span>
            <h3 class="font-display text-lg font-bold mt-3 mb-2">{step.title}</h3>
            <p class="text-sm text-base-content/65 leading-relaxed">{step.text}</p>
          </li>
        {/each}
      </ol>
    </div>
  </section>

  <section id="faq" class="bg-base-200 py-20 md:py-28 px-6">
    <div class="max-w-3xl mx-auto">
      <div class="reveal text-center mb-10">
        <p class="font-mono text-xs uppercase tracking-[0.2em] text-primary mb-3">Questions worth asking</p>
        <h2 class="font-display text-3xl md:text-5xl font-bold tracking-tight">Before you clone it</h2>
      </div>
      <div class="reveal space-y-3">
        {#each faqs as faq, i (faq.q)}
          <div class="collapse collapse-arrow bg-base-100 border border-base-content/10">
            <input type="radio" name="landing-faq" checked={i === 0} aria-label={faq.q} />
            <div class="collapse-title font-display font-semibold text-lg pr-12">{faq.q}</div>
            <div class="collapse-content text-base-content/70 leading-relaxed">{faq.a}</div>
          </div>
        {/each}
      </div>
    </div>
  </section>

  <section class="relative overflow-hidden">
    <div class="absolute inset-0 bg-primary"></div>
    <div class="cs-grid absolute inset-0 opacity-40 pointer-events-none" aria-hidden="true"></div>
    <div class="relative max-w-3xl mx-auto px-6 py-20 md:py-24 text-center text-primary-content">
      <img
        src="/images/logo.jpg"
        alt=""
        width="72"
        height="72"
        class="w-16 h-16 rounded-2xl mx-auto mb-6 shadow-lg ring-1 ring-primary-content/20 object-cover"
      />
      <h2 class="font-display text-4xl md:text-5xl font-extrabold tracking-tight mb-4">Bring your domain. The platform is here.</h2>
      <p class="text-lg text-primary-content/85 max-w-xl mx-auto mb-8 leading-relaxed">
        Clone the repository, import the schema, and sign in. The next edit that matters is replacing <span class="font-mono">item</span> with the thing you actually sell.
      </p>
      <div class="flex flex-wrap gap-3 justify-center">
        {#if $isAuthenticated}
          <a href="/dashboard" class="btn btn-lg bg-base-100 text-base-content border-0 font-bold hover:bg-base-100/90">Open the dashboard</a>
        {:else}
          <a href="/signup" class="btn btn-lg bg-base-100 text-base-content border-0 font-bold hover:bg-base-100/90">Create an account</a>
        {/if}
        <a href={repo} target="_blank" rel="noopener noreferrer" class="btn btn-lg btn-outline border-primary-content text-primary-content hover:bg-primary-content hover:text-primary font-semibold">
          Read the source
        </a>
      </div>
    </div>
  </section>

  <LandingFooter />
</div>
