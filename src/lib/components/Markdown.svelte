<script lang="ts">
  import { compile } from 'mdsvex';

  export let source: string = '';

  let html = '';
  let error: string | null = null;

  $: if (source) {
    (async () => {
      try {
        // Compile markdown → HTML
        const result: any = await compile(source);

        html = result.code;
      } catch (err) {
        console.error('mdsvex compile failed:', err);
        error = 'Failed to render markdown';
        html = '';
      }
    })();
  }
</script>

{#if error}
  <div class="alert alert-error">{error}</div>
{:else if html}
  <div class="prose prose-invert max-w-none">
    {@html html}
  </div>
{:else}
  <div class="skeleton h-32 w-full rounded-lg"></div>
{/if}