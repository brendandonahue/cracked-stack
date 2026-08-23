<script lang="ts">
  const year = new Date().getFullYear();
</script>

<footer class="bg-base-200 border-t border-base-content/5">
  <div class="container mx-auto px-6 py-12">
    <div class="grid grid-cols-1 md:grid-cols-3 gap-10">
      <div>
        <a href="/" class="inline-flex items-center gap-3 group">
          <img
            src="/images/logo.jpg"
            alt="crackedstack"
            class="w-10 h-10 rounded-lg object-cover"
            width="40"
            height="40"
          />
          <span class="font-display font-bold text-lg tracking-tight group-hover:text-primary transition-colors">
            cracked<span class="text-primary">stack</span>
          </span>
        </a>
        <p class="mt-3 text-sm text-base-content/60 leading-relaxed">
          Production-ready full-stack starter — SvelteKit, Rocket, SurrealDB, and Caddy.
        </p>
      </div>

      <div>
        <h3 class="font-display font-semibold text-sm uppercase tracking-wide mb-4">Product</h3>
        <ul class="space-y-2 text-sm">
          <li><a href="/" class="link link-hover text-base-content/70">Home</a></li>
          <li><a href="/signup" class="link link-hover text-base-content/70">Get Started</a></li>
          <li><a href="/login" class="link link-hover text-base-content/70">Login</a></li>
          <li>
            <a
              href="https://github.com/brendandonahue/cracked-stack"
              target="_blank"
              rel="noopener noreferrer"
              class="link link-hover text-base-content/70"
            >
              GitHub
            </a>
          </li>
        </ul>
      </div>

      <div>
        <h3 class="font-display font-semibold text-sm uppercase tracking-wide mb-4">Stack</h3>
        <ul class="space-y-2 text-sm text-base-content/70">
          <li>SvelteKit · Tailwind · DaisyUI</li>
          <li>Rocket.rs · SurrealDB</li>
          <li>Docker · nginx · Caddy</li>
        </ul>
      </div>
    </div>

    <div class="divider my-6"></div>

    <p class="text-center text-xs text-base-content/50 font-mono">
      &copy; {year} crackedstack. MIT License.
    </p>
  </div>
</footer>
