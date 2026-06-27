<script lang="ts">
  import { auth } from '$lib/stores/auth';
  import { goto } from '$app/navigation';

  let email = $state('');
  let password = $state('');
  let name = $state('');
  let error = $state('');
  let loading = $state(false);

  async function handleSignup(e: Event) {
    e.preventDefault();
    error = '';
    loading = true;
    try {
      await auth.signup(email, password, name, 'user');
      await goto('/dashboard', { replaceState: true });
    } catch (err) {
      error = err instanceof Error ? err.message : 'Signup failed. Please try again.';
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
      <p class="text-base-content/70 mt-2">Create your account to get started</p>
    </div>

    <div class="card bg-base-100 shadow-xl">
      <div class="card-body p-8 md:p-10">
        {#if error}
          <div class="alert alert-error mb-6"><span>{error}</span></div>
        {/if}

        <form onsubmit={handleSignup} class="space-y-6">
          <div class="form-control">
            <label for="name" class="label">
              <span class="label-text font-medium">Full Name</span>
            </label>
            <input
              type="text" id="name" placeholder="Your name"
              class="input input-bordered w-full"
              bind:value={name} required
            />
          </div>

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

          <div class="pt-4">
            <button type="submit" class="btn btn-primary w-full h-12 text-base font-semibold" disabled={loading}>
              {#if loading}
                <span class="loading loading-spinner loading-md"></span> Creating account…
              {:else}
                Create Account
              {/if}
            </button>
          </div>
        </form>

        <div class="text-center mt-8">
          <p class="text-sm text-base-content/70">
            Already have an account?
            <a href="/login" class="link link-primary font-medium">Login</a>
          </p>
        </div>
      </div>
    </div>
  </div>
</div>
