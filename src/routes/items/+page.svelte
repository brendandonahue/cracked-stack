<script lang="ts">
  import { onMount } from 'svelte';
  import { getItems, createItem, updateItem, deleteItem, type Item } from '$lib/api';
  import { toastStore } from '$lib/stores/toast';

  let items = $state<Item[]>([]);
  let loading = $state(true);
  let saving = $state(false);

  // Create form
  let newTitle = $state('');
  let newDescription = $state('');

  // Edit state
  let editingId = $state<string | null>(null);
  let editTitle = $state('');
  let editDescription = $state('');

  onMount(async () => {
    await loadItems();
  });

  async function loadItems() {
    loading = true;
    try {
      items = await getItems();
    } catch (e) {
      toastStore.error('Failed to load items');
    } finally {
      loading = false;
    }
  }

  async function handleCreate(e: Event) {
    e.preventDefault();
    if (!newTitle.trim()) return;
    saving = true;
    try {
      const created = await createItem({ title: newTitle.trim(), description: newDescription.trim() || undefined });
      items = [created, ...items];
      newTitle = '';
      newDescription = '';
      toastStore.success('Item created!');
    } catch (e) {
      toastStore.error('Failed to create item');
    } finally {
      saving = false;
    }
  }

  function startEdit(item: Item) {
    editingId = item.id;
    editTitle = item.title;
    editDescription = item.description ?? '';
  }

  function cancelEdit() {
    editingId = null;
    editTitle = '';
    editDescription = '';
  }

  async function handleUpdate(id: string) {
    saving = true;
    try {
      const updated = await updateItem(id, { title: editTitle.trim(), description: editDescription.trim() || undefined });
      items = items.map(i => i.id === id ? { ...i, ...updated } : i);
      cancelEdit();
      toastStore.success('Item updated!');
    } catch (e) {
      toastStore.error('Failed to update item');
    } finally {
      saving = false;
    }
  }

  async function handleDelete(id: string) {
    if (!confirm('Delete this item?')) return;
    try {
      await deleteItem(id);
      items = items.filter(i => i.id !== id);
      toastStore.success('Item deleted');
    } catch (e) {
      toastStore.error('Failed to delete item');
    }
  }
</script>

<div class="max-w-3xl mx-auto px-4 py-10">

  <div class="mb-8">
    <h1 class="text-4xl font-bold">Items</h1>
    <p class="text-base-content/60 mt-1">
      A demo CRUD resource. Replace this with your own domain model.
    </p>
  </div>

  <!-- Create form -->
  <div class="card bg-base-100 shadow-xl mb-8">
    <div class="card-body">
      <h2 class="card-title text-lg mb-2">New Item</h2>
      <form onsubmit={handleCreate} class="space-y-3">
        <input
          type="text"
          placeholder="Title *"
          class="input input-bordered w-full"
          bind:value={newTitle}
          required
        />
        <textarea
          placeholder="Description (optional)"
          class="textarea textarea-bordered w-full"
          rows="2"
          bind:value={newDescription}
        ></textarea>
        <button type="submit" class="btn btn-primary w-full" disabled={saving || !newTitle.trim()}>
          {#if saving}
            <span class="loading loading-spinner loading-sm"></span> Saving…
          {:else}
            ➕ Create Item
          {/if}
        </button>
      </form>
    </div>
  </div>

  <!-- Items list -->
  {#if loading}
    <div class="flex justify-center py-16">
      <span class="loading loading-spinner loading-lg"></span>
    </div>
  {:else if items.length === 0}
    <div class="text-center py-20 text-base-content/40">
      <div class="text-6xl mb-4">📭</div>
      <p class="text-lg">No items yet. Create your first one above.</p>
    </div>
  {:else}
    <div class="space-y-4">
      {#each items as item (item.id)}
        <div class="card bg-base-100 shadow-md">
          <div class="card-body py-4">
            {#if editingId === item.id}
              <!-- Edit mode -->
              <div class="space-y-2">
                <input
                  type="text"
                  class="input input-bordered w-full"
                  bind:value={editTitle}
                  placeholder="Title"
                />
                <textarea
                  class="textarea textarea-bordered w-full"
                  rows="2"
                  bind:value={editDescription}
                  placeholder="Description"
                ></textarea>
                <div class="flex gap-2">
                  <button
                    class="btn btn-primary btn-sm"
                    onclick={() => handleUpdate(item.id)}
                    disabled={saving || !editTitle.trim()}
                  >
                    {#if saving}<span class="loading loading-spinner loading-xs"></span>{:else}Save{/if}
                  </button>
                  <button class="btn btn-ghost btn-sm" onclick={cancelEdit}>Cancel</button>
                </div>
              </div>
            {:else}
              <!-- View mode -->
              <div class="flex items-start justify-between gap-4">
                <div class="flex-1 min-w-0">
                  <h3 class="font-semibold text-lg leading-tight">{item.title}</h3>
                  {#if item.description}
                    <p class="text-base-content/60 text-sm mt-1">{item.description}</p>
                  {/if}
                  <p class="text-base-content/40 text-xs mt-2 font-mono">
                    {item.id} · {new Date(item.created_at).toLocaleDateString()}
                  </p>
                </div>
                <div class="flex gap-1 shrink-0">
                  <button class="btn btn-ghost btn-sm btn-square" onclick={() => startEdit(item)} title="Edit">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                      <path stroke-linecap="round" stroke-linejoin="round" d="m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10" />
                    </svg>
                  </button>
                  <button class="btn btn-ghost btn-sm btn-square text-error" onclick={() => handleDelete(item.id)} title="Delete">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                      <path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0" />
                    </svg>
                  </button>
                </div>
              </div>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {/if}

</div>
