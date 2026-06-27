<!-- src/routes/+layout.svelte -->
<script lang="ts">
  import "../app.css";
  import Navbar from "$lib/components/Navbar.svelte";
  import Dock from "$lib/components/Dock.svelte";
  import ToastContainer from "$lib/components/ToastContainer.svelte";

  import { auth, isAuthenticated, currentUser } from "$lib/stores/auth";
  import { goto, afterNavigate } from "$app/navigation";
  import { page } from "$app/state";
  import { isPublicPage, getDefaultRedirectPath, isRoleAllowedForPath } from "$lib/auth/guards";
  import type { Snippet } from "svelte";

  let { children }: { children: Snippet } = $props();

  let isChecking = $state(true);

  async function checkAuth() {
    if (isPublicPage(page.url.pathname)) {
      isChecking = false;
      return;
    }

    await auth.refresh();

    if (!$isAuthenticated) {
      await goto("/login", { replaceState: true });
      isChecking = false;
      return;
    }

    const userRole = $currentUser?.role;
    if (!isRoleAllowedForPath(userRole, page.url.pathname)) {
      await goto(getDefaultRedirectPath(userRole), { replaceState: true });
      isChecking = false;
      return;
    }

    isChecking = false;
  }

  $effect(() => { checkAuth(); });
  afterNavigate(() => { isChecking = true; checkAuth(); });
</script>

{#if isChecking}
  <div class="flex items-center justify-center min-h-screen bg-base-200">
    <span class="loading loading-spinner loading-lg"></span>
  </div>
{:else}
  <div class="flex flex-col min-h-screen font-mono">
    {#if isPublicPage(page.url.pathname)}
      {@render children()}
    {:else if $isAuthenticated}
      <Navbar />
      <main class="flex-1 pb-16">
        {@render children()}
      </main>
      <Dock />
    {:else}
      <div class="flex items-center justify-center min-h-screen bg-base-200">
        <div class="alert alert-warning">Redirecting to login…</div>
      </div>
    {/if}
    <ToastContainer />
  </div>
{/if}
