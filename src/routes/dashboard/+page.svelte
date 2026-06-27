<script lang="ts">
  import { currentUser } from '$lib/stores/auth';
  import { getItems } from '$lib/api';
  import { onMount } from 'svelte';

  let itemCount = $state(0);
  let loading = $state(true);

  onMount(async () => {
    try {
      const items = await getItems();
      itemCount = items.length;
    } catch (e) {
      // If GraphQL isn't seeded yet, ignore
    } finally {
      loading = false;
    }
  });

  const initials = $derived(
    $currentUser?.name
      ? $currentUser.name.split(' ').map((n: string) => n[0]).slice(0, 2).join('').toUpperCase()
      : $currentUser?.email?.[0]?.toUpperCase() ?? '?'
  );
</script>

<div class="max-w-4xl mx-auto px-4 py-10">

  <!-- Welcome header -->
  <div class="mb-10">
    <h1 class="text-4xl font-bold">
      Welcome back{$currentUser?.name ? `, ${$currentUser.name.split(' ')[0]}` : ''}! 👋
    </h1>
    <p class="text-base-content/60 mt-1">Here's what's going on in your account.</p>
  </div>

  <!-- Stats row -->
  <div class="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-10">
    <div class="stat bg-base-100 rounded-2xl shadow">
      <div class="stat-figure text-primary">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-8">
          <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 6.75h12M8.25 12h12m-12 5.25h12M3.75 6.75h.007v.008H3.75V6.75Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0ZM3.75 12h.007v.008H3.75V12Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm-.375 5.25h.007v.008H3.75v-.008Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Z" />
        </svg>
      </div>
      <div class="stat-title">Your Items</div>
      <div class="stat-value text-primary">
        {#if loading}
          <span class="loading loading-dots loading-sm"></span>
        {:else}
          {itemCount}
        {/if}
      </div>
      <div class="stat-desc">
        <a href="/items" class="link link-primary">Manage items →</a>
      </div>
    </div>

    <div class="stat bg-base-100 rounded-2xl shadow">
      <div class="stat-figure text-secondary">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-8">
          <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0ZM4.501 20.118a7.5 7.5 0 0 1 14.998 0A17.933 17.933 0 0 1 12 21.75c-2.676 0-5.216-.584-7.499-1.632Z" />
        </svg>
      </div>
      <div class="stat-title">Role</div>
      <div class="stat-value text-secondary capitalize">{$currentUser?.role ?? '—'}</div>
      <div class="stat-desc">
        <a href="/profile" class="link link-secondary">View profile →</a>
      </div>
    </div>

    <div class="stat bg-base-100 rounded-2xl shadow">
      <div class="stat-figure text-accent">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-8">
          <path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75 11.25 15 15 9.75m-3-7.036A11.959 11.959 0 0 1 3.598 6 11.99 11.99 0 0 0 3 9.749c0 5.592 3.824 10.29 9 11.623 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.285Z" />
        </svg>
      </div>
      <div class="stat-title">Auth Status</div>
      <div class="stat-value text-accent text-2xl">Active</div>
      <div class="stat-desc">Cookie session · 24h TTL</div>
    </div>
  </div>

  <!-- Account details card -->
  <div class="card bg-base-100 shadow-xl mb-6">
    <div class="card-body">
      <h2 class="card-title mb-4">Account Details</h2>
      <div class="flex items-center gap-4">
        <div class="avatar placeholder">
          <div class="bg-primary text-primary-content rounded-full w-14 text-xl font-bold flex items-center justify-center">
            <span>{initials}</span>
          </div>
        </div>
        <div>
          <p class="font-semibold text-lg">{$currentUser?.name ?? 'Unnamed User'}</p>
          <p class="text-base-content/60 text-sm">{$currentUser?.email}</p>
          <span class="badge badge-primary badge-sm mt-1 capitalize">{$currentUser?.role}</span>
        </div>
      </div>
    </div>
  </div>

  <!-- Quick actions -->
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <h2 class="card-title mb-4">Quick Actions</h2>
      <div class="grid grid-cols-2 sm:grid-cols-3 gap-3">
        <a href="/items" class="btn btn-outline justify-start gap-2">
          <span>📋</span> Items
        </a>
        <a href="/items" class="btn btn-outline justify-start gap-2">
          <span>➕</span> New Item
        </a>
        <a href="/profile" class="btn btn-outline justify-start gap-2">
          <span>👤</span> Profile
        </a>
      </div>
    </div>
  </div>

</div>
