<script lang="ts">
  let {
    accordions,
  }: {
    accordions?: [{summary: String, details: String}];
  } = $props();
</script>
{#each accordions as accordion}
<details class="collapse bg-base-100 border border-base-300" name="my-accordion-det-1" open>
  <summary class="collapse-title font-semibold">{accordion.summary}</summary>
  <div class="collapse-content text-sm">{accordion.details}</div>
</details>
{/each}
