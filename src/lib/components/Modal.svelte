<script lang="ts">
    import type { Snippet } from "svelte";

  let {
    open = $bindable(false),                  // two-way binding
    title = '',
    size = 'md',                               // sm, md, lg, xl, 2xl, 3xl, 5xl, full
    showCloseButton = true,
    closeOnBackdrop = true,
    closeOnEsc = true,                         // native dialog already supports Esc
    class: extraClass = '',
    onOpen,
    onClose,
    onCancel,
    onConfirm,
    children,
    actions
  }: {
    open?: boolean;
    title?: string;
    size?: 'sm' | 'md' | 'lg' | 'xl' | '2xl' | '3xl' | '5xl' | 'full';
    showCloseButton?: boolean;
    closeOnBackdrop?: boolean;
    closeOnEsc?: boolean;
    class?: string;
    onOpen?: () => void;
    onClose?: () => void;
    onCancel?: () => void;
    onConfirm?: (detail?: unknown) => void;
    children?: Snippet;
    actions?: Snippet;
  } = $props();

  // ────────────────────────────────────────────────
  // State & refs
  // ────────────────────────────────────────────────
  let dialog = $state<HTMLDialogElement | null>(null);

  // Sync prop → dialog state
  $effect(() => {
    if (!dialog) return;

    if (open) {
      dialog.showModal();
      onOpen?.();
    } else {
      dialog.close();
      onClose?.();
    }
  });

  // Handle backdrop click (when clicking the ::backdrop pseudo-element area)
  function handleBackdropClick(e: MouseEvent) {
    if (closeOnBackdrop && e.target === dialog) {
      dialog?.close();
      onClose?.();
      onCancel?.();
    }
  }

  // Optional: if someone calls dialog.cancel() programmatically
  function handleCancel(e: Event) {
    // You can e.preventDefault() here if you want to block closing
    onCancel?.();
  }
</script>
<!-- Open the modal using ID.showModal() method -->
<dialog
  bind:this={dialog}
  class="modal {extraClass}"
  oncancel={handleCancel}
  onclick={handleBackdropClick}
>
  <div
    class="modal-box relative"
    class:max-w-sm={size === 'sm'}
    class:max-w-md={size === 'md'}
    class:max-w-lg={size === 'lg'}
    class:max-w-xl={size === 'xl'}
    class:max-w-2xl={size === '2xl'}
    class:max-w-3xl={size === '3xl'}
    class:max-w-5xl={size === '5xl'}
    class:max-w-full={size === 'full'}
    class:h-full={size === 'full'}
  >
    {#if showCloseButton}
      <form method="dialog">
        <button
          class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
          aria-label="close"
          onclick={() => {
            open = false;
            onClose?.();
            onCancel?.();
          }}
        >
          ✕
        </button>
      </form>
    {/if}

    {#if title}
      <h3 class="font-bold text-lg mb-4 pr-10">{title}</h3>
    {/if}

    <!-- Main content slot -->
    {@render children?.()}

    <!-- Actions / footer slot (named) -->
    {#if actions}
      {@render actions()}
    {:else}
      <div class="modal-action">
        <button class="btn" onclick={() => (open = false)}>Close</button>
      </div>
    {/if}
  </div>

  {#if closeOnBackdrop}
    <form method="dialog" class="modal-backdrop">
      <button>close</button>
    </form>
  {/if}
</dialog>