<script lang="ts">
    import type { Snippet } from "svelte";

    let {
        children,
        link
    }: {
        children: Snippet;
        link?: string;
    } = $props();
</script>
{#if link}
<a href={link}>
    <div class="hover-3d">
        {@render children()}
        <!-- 8 empty divs needed for the 3D effect -->
        <div></div>
        <div></div>
        <div></div>
        <div></div>
        <div></div>
        <div></div>
        <div></div>
        <div></div>
    </div>
</a>
  {:else}
<div class="hover-3d">
    {@render children()}
    <!-- 8 empty divs needed for the 3D effect -->
    <div></div>
    <div></div>
    <div></div>
    <div></div>
    <div></div>
    <div></div>
    <div></div>
    <div></div>
</div>
{/if}
