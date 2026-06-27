<script lang="ts">
  import { auth } from '$lib/stores/auth';
  import { goto } from '$app/navigation';

  let email = $state('');
  let password = $state('');
  let error = $state('');
  let loading = $state(false);

  async function handleLogin(e: Event) {
    e.preventDefault();
    error = '';
    loading = true;
    try {
      await auth.login(email, password);
      await goto('/dashboard');
    } catch (err) {
      error = err instanceof Error ? err.message : 'Login failed';
    } finally {
      loading = false;
    }
  }
</script>

<div class="min-h-screen bg-base-200 flex items-center justify-center py-12 px-4">
  <div class="w-full max-w-md">
    <div class="text-center mb-8">
      <a href="/" class="text-3xl font-bold tracking-tight">
        cracked<span class="text-primary">stack</span>
      </a>
      <p class="text-base-content/70 mt-2">Sign in to your account</p>
    </div>

    <div class="card bg-base-100 shadow-xl">
      <div class="card-body p-8 md:p-10">
        {#if error}
          <div class="alert alert-error mb-6"><span>{error}</span></div>
        {/if}

        <form onsubmit={handleLogin} class="space-y-6">
          <div class="form-control">
            <label for="email" class="label">
              <span class="label-text font-medium">Email Address</span>
            </label>
            <input
              type="email" id="email" placeholder="you@example.com"
              class="input input-bordered w-full"
              bind:value={email} required
            />
          </div>

          <div class="form-control">
            <label for="password" class="label">
              <span class="label-text font-medium">Password</span>
            </label>
            <input
              type="password" id="password" placeholder="••••••••"
              class="input input-bordered w-full"
              bind:value={password} required
            />
          </div>

          <div class="pt-2">
            <button type="submit" class="btn btn-primary w-full h-12 text-base font-semibold" disabled={loading}>
              {#if loading}
                <span class="loading loading-spinner loading-md"></span> Logging in…
              {:else}
                Login
              {/if}
            </button>
          </div>
        </form>

        <div class="text-center mt-8 space-y-1">
          <a href="/forgot-password" class="link link-primary text-sm">Forgot password?</a>
          <p class="text-sm text-base-content/70">
            Don't have an account?
            <a href="/signup" class="link link-primary font-medium">Sign up</a>
          </p>
        </div>
      </div>
    </div>
  </div>
</div>
