<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    children,
    size,
  }: {
    children: Snippet;
    size?: String;
  } = $props();
</script>
<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class={size || 'size-10'}>
  {@render children()}
</svg>
