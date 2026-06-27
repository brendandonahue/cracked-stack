<script lang="ts">
    import type { Snippet } from 'svelte';

    interface FabItem {
        label: string;
        icon: Snippet;
        onclick: () => void;
        class?: string;
    }

    interface Props {
        /** Items shown when the FAB is expanded */
        items?: FabItem[];
        /** Icon shown on the closed-state trigger button */
        triggerIcon?: Snippet;
        /** Tailwind/DaisyUI position class, e.g. "bottom-18" */
        position?: string;
        /** Additional class for the main FAB wrapper */
        class?: string;
    }

    let {
        items = [],
        triggerIcon,
        position = 'bottom-18',
        class: extraClass = '',
    }: Props = $props();
</script>

<div class="fab fab-flower {position} {extraClass}">
    <!-- Closed-state preview button (shown before FAB is opened) -->
    <div tabindex="0" role="button" class="btn btn-circle btn-lg btn-primary">
        {#if triggerIcon}
            {@render triggerIcon()}
        {:else}
            <!-- Default: sparkles icon -->
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6 animate-bounce">
                <path stroke-linecap="round" stroke-linejoin="round" d="M9.813 15.904 9 18.75l-.813-2.846a4.5 4.5 0 0 0-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 0 0 3.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 0 0 3.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 0 0-3.09 3.09ZM18.259 8.715 18 9.75l-.259-1.035a3.375 3.375 0 0 0-2.455-2.456L14.25 6l1.036-.259a3.375 3.375 0 0 0 2.455-2.456L18 2.25l.259 1.035a3.375 3.375 0 0 0 2.456 2.456L21.75 6l-1.035.259a3.375 3.375 0 0 0-2.456 2.456Z" />
            </svg>
        {/if}
    </div>

    <!-- Main action button (replaces the trigger when FAB is open) -->
    <button class="fab-main-action btn btn-circle btn-lg" aria-label="Actions">
        {#if triggerIcon}
            {@render triggerIcon()}
        {:else}
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                <path stroke-linecap="round" stroke-linejoin="round" d="M9.813 15.904 9 18.75l-.813-2.846a4.5 4.5 0 0 0-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 0 0 3.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 0 0 3.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 0 0-3.09 3.09ZM18.259 8.715 18 9.75l-.259-1.035a3.375 3.375 0 0 0-2.455-2.456L14.25 6l1.036-.259a3.375 3.375 0 0 0 2.455-2.456L18 2.25l.259 1.035a3.375 3.375 0 0 0 2.456 2.456L21.75 6l-1.035.259a3.375 3.375 0 0 0-2.456 2.456Z" />
            </svg>
        {/if}
    </button>

    <!-- Action item buttons -->
    {#each items as item}
        <button
            class="btn btn-circle btn-lg btn-primary group {item.class ?? ''}"
            aria-label={item.label}
            onclick={item.onclick}
        >
            {@render item.icon()}
        </button>
    {/each}
</div>
