<!-- Navbar.svelte -->
<script lang="ts">
    import { currentUser, auth } from '$lib/stores/auth';
    import { goto } from '$app/navigation';

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

    let drawerOpen = $state(false);

    function nav(path: string) {
        goto(path);
        drawerOpen = false;
    }
</script>

<div class="drawer">
    <input id="nav-drawer" type="checkbox" class="drawer-toggle" bind:checked={drawerOpen} />

    <!-- Drawer side -->
    <div class="drawer-side z-60">
        <label for="nav-drawer" aria-label="close sidebar" class="drawer-overlay"></label>
        <ul class="menu bg-base-200 min-h-full w-80 p-4 text-lg gap-1">
            <li class="mb-4">
                <label for="nav-drawer" class="btn btn-ghost w-full justify-start text-base">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                    Close
                </label>
            </li>
            {#if $currentUser}
                <li><a onclick={() => nav('/dashboard')}>🏠 Dashboard</a></li>
                <li><a onclick={() => nav('/items')}>📋 Items</a></li>
                <li><a onclick={() => nav('/files')}>🗂️ Files</a></li>
                <li><a onclick={() => nav('/profile')}>👤 Profile</a></li>

                <li><a onclick={() => { auth.logout(); goto('/'); drawerOpen = false; }}>🚪 Logout</a></li>

                <!-- User info card -->
                <li class="mt-4">
                    <div class="card bg-base-300 rounded-xl p-3 flex flex-row items-center gap-3 cursor-default hover:bg-base-300 active:bg-base-300">
                        <div class="avatar placeholder shrink-0">
                            <div class="bg-primary text-primary-content rounded-full w-10 font-bold flex items-center justify-center text-sm">
                                <span>{initials}</span>
                            </div>
                        </div>
                        <div class="min-w-0 flex-1">
                            <p class="font-semibold text-sm truncate leading-tight">{$currentUser.name ?? $currentUser.email}</p>
                            {#if $currentUser.name}
                                <p class="text-xs text-base-content/60 truncate">{$currentUser.email}</p>
                            {/if}
                            <span class="badge {roleInfo.badge} badge-sm mt-1">{roleInfo.label}</span>
                        </div>
                    </div>
                </li>
            {/if}
        </ul>
    </div>

    <!-- Drawer content -->
    <div class="drawer-content">
        <div class="navbar bg-base-100 shadow-sm sticky top-0 z-50">
            <div class="navbar-start">
                {#if $currentUser}
                    <label for="nav-drawer" class="btn btn-ghost drawer-button">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                        </svg>
                    </label>
                {/if}
            </div>
            <div class="navbar-center">
                <a href="/" class="btn btn-ghost text-xl font-bold tracking-tight">
                    cracked<span class="text-primary">stack</span>
                </a>
            </div>
            <div class="navbar-end">
                {#if !$currentUser}
                    <a href="/login" class="btn btn-primary btn-sm">Login</a>
                {:else}
                    <a href="/dashboard" class="btn btn-ghost btn-sm">Dashboard</a>
                {/if}
            </div>
        </div>
    </div>
</div>
