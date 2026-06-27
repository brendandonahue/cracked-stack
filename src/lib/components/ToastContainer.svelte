<script lang="ts">
  import { toastStore } from '$lib/stores/toast';
  import { onMount } from 'svelte';

  let toasts: any[] = [];

  const unsubscribe = toastStore.subscribe(value => {
    toasts = value;
  });

  onMount(() => {
    return unsubscribe;
  });
</script>

{#each toasts as toast (toast.id)}
  <div class="toast toast-top toast-center z-50">
    <div class="alert {toast.type === 'success' ? 'alert-success' : toast.type === 'error' ? 'alert-error' : toast.type === 'warning' ? 'alert-warning' : 'alert-info'}">
      <span>{toast.message}</span>
      <button
        class="btn btn-sm btn-circle btn-ghost"
        onclick={() => toastStore.removeToast(toast.id)}
      >
        ✕
      </button>
    </div>
  </div>
{/each}