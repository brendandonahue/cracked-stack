<script lang="ts">
  const year = new Date().getFullYear();

  const product = [
    { href: '/#why', label: 'Why this stack' },
    { href: '/#stack', label: 'The three layers' },
    { href: '/#path', label: 'Request path' },
    { href: '/#security', label: 'Security' },
    { href: '/#box', label: 'What ships' },
    { href: '/signup', label: 'Create an account' },
    { href: '/login', label: 'Login' },
  ];

  const stack = [
    'SvelteKit 2 · Svelte 5',
    'Tailwind CSS · DaisyUI',
    'Rocket.rs · Rust',
    'SurrealDB · GraphQL',
    'Docker · nginx · Caddy',
    'Tauri desktop and mobile',
  ];
</script>

<footer class="bg-base-200 border-t border-base-content/10">
  <div class="max-w-6xl mx-auto px-6 py-14">
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-10">
      <div class="sm:col-span-2 lg:col-span-1">
        <a href="/" class="inline-flex items-center gap-3 group">
          <img
            src="/images/logo.jpg"
            alt=""
            class="w-10 h-10 rounded-xl object-cover ring-1 ring-base-content/10"
            width="40"
            height="40"
          />
          <span class="font-display font-bold text-lg tracking-tight">
            cracked<span class="text-primary">stack</span>
          </span>
        </a>
        <p class="mt-4 text-sm text-base-content/65 leading-relaxed max-w-xs">
          A full application you can run today. Replace the demo model. Keep the sessions, the permissions, and the deploy.
        </p>
      </div>

      <div>
        <h2 class="font-mono text-xs uppercase tracking-[0.18em] text-primary mb-4">Product</h2>
        <ul class="space-y-2 text-sm">
          {#each product as link (link.href)}
            <li>
              <a href={link.href} class="link link-hover text-base-content/70">{link.label}</a>
            </li>
          {/each}
        </ul>
      </div>

      <div>
        <h2 class="font-mono text-xs uppercase tracking-[0.18em] text-primary mb-4">Stack</h2>
        <ul class="space-y-2 text-sm text-base-content/70">
          {#each stack as line (line)}
            <li>{line}</li>
          {/each}
        </ul>
      </div>

      <div>
        <h2 class="font-mono text-xs uppercase tracking-[0.18em] text-primary mb-4">Source</h2>
        <p class="text-sm text-base-content/65 leading-relaxed mb-4">
          MIT licensed. The schema, the API, and the deploy script are in the repository.
        </p>
        <a
          href="https://github.com/brendandonahue/cracked-stack"
          target="_blank"
          rel="noopener noreferrer"
          class="btn btn-outline btn-sm"
        >
          View on GitHub
        </a>
      </div>
    </div>

    <div class="mt-12 pt-6 border-t border-base-content/10 flex flex-col sm:flex-row items-center justify-between gap-3">
      <p class="text-xs text-base-content/50 font-mono">
        &copy; {year} crackedstack · MIT License
      </p>
      <p class="text-xs text-base-content/45 font-mono">
        Theme <span class="text-primary">cracked</span>
      </p>
    </div>
  </div>
</footer>
