<script lang="ts">
  import { goto } from '$app/navigation';
  import { auth, isAuthenticated, currentUser } from '$lib/stores/auth';

  const roleConfig: Record<string, { badge: string; label: string }> = {
    admin: { badge: 'badge-error',   label: 'Admin' },
    user:  { badge: 'badge-primary', label: 'User' },
  };

  const initials = $derived(
    $currentUser?.name
      ? $currentUser.name.split(' ').map((n: string) => n[0]).slice(0, 2).join('').toUpperCase()
      : $currentUser?.email?.[0]?.toUpperCase() ?? '?'
  );

  const role = $derived($currentUser?.role ?? '');
  const roleInfo = $derived(roleConfig[role] ?? { badge: 'badge-ghost', label: role });

  async function handleLogout() {
    await auth.logout();
    goto('/');
  }
</script>

{#if $isAuthenticated}
  <div class="max-w-2xl mx-auto px-4 py-10">

    <div class="mb-8">
      <h1 class="text-4xl font-bold">Profile</h1>
      <p class="text-base-content/60 mt-1">Your account details</p>
    </div>

    <div class="card bg-base-100 shadow-xl mb-6">
      <div class="card-body items-center text-center gap-4">
        <div class="avatar placeholder">
          <div class="bg-primary text-primary-content rounded-full w-24 text-3xl font-bold flex items-center justify-center">
            <span>{initials}</span>
          </div>
        </div>

        <div>
          <h2 class="text-2xl font-bold">{$currentUser?.name ?? 'Unnamed User'}</h2>
          <p class="text-base-content/60 text-sm mt-1">{$currentUser?.email}</p>
        </div>

        <span class="badge {roleInfo.badge} badge-lg font-semibold">{roleInfo.label}</span>

        <button class="btn btn-error btn-outline w-full mt-2" onclick={handleLogout}>
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 9V5.25A2.25 2.25 0 0 0 13.5 3h-6a2.25 2.25 0 0 0-2.25 2.25v13.5A2.25 2.25 0 0 0 7.5 21h6a2.25 2.25 0 0 0 2.25-2.25V15m3 0 3-3m0 0-3-3m3 3H9" />
          </svg>
          Sign Out
        </button>
      </div>
    </div>

    <div class="card bg-base-100 shadow-xl">
      <div class="card-body">
        <h2 class="card-title mb-2">Account Details</h2>
        <div class="divide-y divide-base-200">

          <div class="flex items-center gap-3 py-3">
            <span class="text-xl">👤</span>
            <div>
              <p class="text-xs text-base-content/50 font-medium uppercase tracking-wide">Full Name</p>
              <p class="font-medium">{$currentUser?.name ?? '—'}</p>
            </div>
          </div>

          <div class="flex items-center gap-3 py-3">
            <span class="text-xl">✉️</span>
            <div>
              <p class="text-xs text-base-content/50 font-medium uppercase tracking-wide">Email</p>
              <p class="font-medium">{$currentUser?.email}</p>
            </div>
          </div>

          <div class="flex items-center gap-3 py-3">
            <span class="text-xl">🏷️</span>
            <div>
              <p class="text-xs text-base-content/50 font-medium uppercase tracking-wide">Role</p>
              <p class="font-medium capitalize">{role}</p>
            </div>
          </div>

          <div class="flex items-center gap-3 py-3">
            <span class="text-xl">🔑</span>
            <div>
              <p class="text-xs text-base-content/50 font-medium uppercase tracking-wide">Account ID</p>
              <p class="font-mono text-sm text-base-content/70">{$currentUser?.id ?? '—'}</p>
            </div>
          </div>

        </div>
      </div>
    </div>

  </div>
{:else}
  <div class="flex items-center justify-center min-h-screen">
    <div class="alert alert-warning max-w-sm">
      <span>You must be signed in to view this page.</span>
    </div>
  </div>
{/if}
