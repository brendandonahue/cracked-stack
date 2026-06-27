<script lang="ts">
  import logo from '$lib/public/cracked-stack.jpg';
  import { COMPANY_NAME, CONTACT_EMAIL } from '$lib/legal/constants';

  const year = new Date().getFullYear();
</script>

<footer class="bg-base-200 border-t border-base-300">
  <div class="container mx-auto px-6 py-12">
    <div class="grid grid-cols-1 md:grid-cols-4 gap-10">
      <div class="md:col-span-1">
        <a href="/" class="inline-flex items-center gap-3 group">
          <img
            src={logo}
            alt={COMPANY_NAME}
            class="w-10 h-10 rounded-lg object-cover"
          />
          <span class="font-bold text-lg group-hover:text-primary transition-colors">
            {COMPANY_NAME}
          </span>
        </a>
        <p class="mt-3 text-sm text-base-content/60 leading-relaxed">
          Premium tile craftsmanship, powered by technology.
        </p>
      </div>

      <div>
        <h3 class="font-semibold text-sm uppercase tracking-wide mb-4">For You</h3>
        <ul class="space-y-2 text-sm">
          <li><a href="/get-tile" class="link link-hover text-base-content/70">Get Tile</a></li>
          <li><a href="/sell-tile" class="link link-hover text-base-content/70">Sell Tile</a></li>
          <li><a href="/apply" class="link link-hover text-base-content/70">Apply to Work</a></li>
          <li><a href="/" class="link link-hover text-base-content/70">For Contractors</a></li>
        </ul>
      </div>

      <div>
        <h3 class="font-semibold text-sm uppercase tracking-wide mb-4">Legal</h3>
        <ul class="space-y-2 text-sm">
          <li><a href="/terms" class="link link-hover text-base-content/70">Terms of Service</a></li>
          <li><a href="/privacy" class="link link-hover text-base-content/70">Privacy Policy</a></li>
        </ul>
      </div>

      <div>
        <h3 class="font-semibold text-sm uppercase tracking-wide mb-4">Contact</h3>
        <p class="text-sm text-base-content/70">
          <a href="mailto:{CONTACT_EMAIL}" class="link link-hover">{CONTACT_EMAIL}</a>
        </p>
      </div>
    </div>

    <div class="divider my-6"></div>

    <p class="text-center text-xs text-base-content/50">
      &copy; {year} {COMPANY_NAME}. All rights reserved.
    </p>
  </div>
</footer>