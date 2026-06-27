<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
    import { apiFetch } from '$lib/api';

	let password = '';
	let confirmPassword = '';
	let error = '';
	let success = '';
	let loading = false;

	$: token = $page.params.token;

	async function handleResetPassword(e: Event) {
		e.preventDefault();
		error = '';
		success = '';
		loading = true;

		if (password !== confirmPassword) {
			error = 'Passwords do not match';
			loading = false;
			return;
		}

		if (password.length < 8) {
			error = 'Password must be at least 8 characters long';
			loading = false;
			return;
		}

		try {
			const response = await apiFetch('/reset-password', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					token,
					new_password: password,
				}),
			});

			if (!response.ok) {
				const errorData = await response.json().catch(() => ({ error: 'Request failed' }));
				throw new Error(errorData.error || 'Request failed');
			}

			success = 'Password reset successfully! Redirecting to login...';
			setTimeout(() => {
				goto('/login');
			}, 2000);
		} catch (err) {
			error = err instanceof Error ? err.message : 'An error occurred';
		} finally {
			loading = false;
		}
	}
</script>

<div class="min-h-screen bg-base-200 flex items-center justify-center py-12 px-4">
  <div class="w-full max-w-md">
    <!-- Header -->
    <div class="text-center mb-8">
      <h1 class="text-3xl font-bold tracking-tight">Set New Password</h1>
      <p class="text-base-content/70 mt-2">Enter your new password</p>
    </div>

    <div class="card bg-base-100 shadow-xl">
      <div class="card-body p-8 md:p-10">
        {#if error}
          <div class="alert alert-error mb-6">
            <span>{error}</span>
          </div>
        {/if}

        {#if success}
          <div class="alert alert-success mb-6">
            <span>{success}</span>
          </div>
        {/if}

        <form on:submit|preventDefault={handleResetPassword} class="space-y-6">
          <!-- New Password -->
          <div class="form-control">
            <label for="password" class="label">
              <span class="label-text font-medium">New Password</span>
            </label>
            <input
              type="password"
              id="password"
              name="password"
              placeholder="••••••••"
              class="input input-bordered w-full focus:input-primary"
              bind:value={password}
              required
              minlength="8"
            />
          </div>

          <!-- Confirm Password -->
          <div class="form-control">
            <label for="confirmPassword" class="label">
              <span class="label-text font-medium">Confirm New Password</span>
            </label>
            <input
              type="password"
              id="confirmPassword"
              name="confirmPassword"
              placeholder="••••••••"
              class="input input-bordered w-full focus:input-primary"
              bind:value={confirmPassword}
              required
              minlength="8"
            />
          </div>

          <!-- Submit Button -->
          <div class="pt-2">
            <button
              type="submit"
              class="btn btn-primary w-full h-12 text-base font-semibold"
              disabled={loading}
            >
              {#if loading}
                <span class="loading loading-spinner loading-md"></span>
                Resetting...
              {:else}
                Reset Password
              {/if}
            </button>
          </div>
        </form>

        <!-- Back to login link -->
        <div class="text-center mt-8">
          <p class="text-sm text-base-content/70">
            Remember your password?
            <a href="/login" class="link link-primary font-medium hover:text-primary">
              Back to login
            </a>
          </p>
        </div>
      </div>
    </div>

    <!-- Optional footer note -->
    <p class="text-center text-xs text-base-content/50 mt-8">
      Secure password reset • Protected by industry-best encryption
    </p>
  </div>
</div>