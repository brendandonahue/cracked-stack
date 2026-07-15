<script lang="ts">
  import { onMount } from 'svelte';
  import { uploadFile, getFileUrl, deleteFile, graphqlQuery, type UploadedFile } from '$lib/api';
  import { toastStore } from '$lib/stores/toast';


  interface FileMeta {
    id: string;
    filename: string;
    original_name: string;
    content_type: string;
    size: number;
    public: boolean;
    created_at: string;
  }

  let files = $state<FileMeta[]>([]);
  let loading = $state(true);
  let uploading = $state(false);
  let makePublic = $state(false);
  let fileInput: HTMLInputElement | undefined = $state();

  onMount(async () => {
    await loadFiles();
  });

  async function loadFiles() {
    loading = true;
    try {
      // `file` is exposed automatically via SurrealDB's GRAPHQL AUTO config,
      // same pattern as the `item` demo resource.
      const query = `query getFiles { file { id filename original_name content_type size public created_at } }`;
      const result = await graphqlQuery(query);
      files = (result.data?.file ?? []).slice().sort((a: FileMeta, b: FileMeta) =>
        new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
      );
    } catch (e) {
      toastStore.error('Failed to load files');
    } finally {
      loading = false;
    }
  }

  async function handleUpload(e: Event) {
    e.preventDefault();
    const chosen = fileInput?.files?.[0];
    if (!chosen) return;

    uploading = true;
    try {
      const uploaded: UploadedFile = await uploadFile(chosen, { public: makePublic });
      toastStore.success(`Uploaded "${chosen.name}"!`);
      if (fileInput) fileInput.value = '';
      makePublic = false;
      await loadFiles();
      void uploaded; // response already reflected via reload
    } catch (e) {
      toastStore.error('Failed to upload file');
    } finally {
      uploading = false;
    }
  }

  async function handleDelete(id: string) {
    if (!confirm('Delete this file?')) return;
    try {
      await deleteFile(id);
      files = files.filter(f => f.id !== id);
      toastStore.success('File deleted');
    } catch (e) {
      toastStore.error('Failed to delete file');
    }
  }

  function formatSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }

  function isImage(contentType: string): boolean {
    return contentType.startsWith('image/');
  }
</script>

<div class="max-w-3xl mx-auto px-4 py-10">

  <div class="mb-8">
    <h1 class="text-4xl font-bold">Files</h1>
    <p class="text-base-content/60 mt-1">
      Demo file upload + serving via the Rust API (<code>POST /upload</code>, <code>GET /files/&lt;id&gt;</code>).
    </p>
  </div>

  <!-- Upload form -->
  <div class="card bg-base-100 shadow-xl mb-8">
    <div class="card-body">
      <h2 class="card-title text-lg mb-2">Upload a file</h2>
      <form onsubmit={handleUpload} class="space-y-3">
        <input
          type="file"
          class="file-input file-input-bordered w-full"
          bind:this={fileInput}
          required
        />
        <label class="label cursor-pointer justify-start gap-3">
          <input type="checkbox" class="checkbox checkbox-sm" bind:checked={makePublic} />
          <span class="label-text">Make public (viewable without login)</span>
        </label>
        <button type="submit" class="btn btn-primary w-full" disabled={uploading}>
          {#if uploading}
            <span class="loading loading-spinner loading-sm"></span> Uploading…
          {:else}
            ⬆️ Upload
          {/if}
        </button>
      </form>
    </div>
  </div>

  <!-- Files list -->
  {#if loading}
    <div class="flex justify-center py-16">
      <span class="loading loading-spinner loading-lg"></span>
    </div>
  {:else if files.length === 0}
    <div class="text-center py-20 text-base-content/40">
      <div class="text-6xl mb-4">🗂️</div>
      <p class="text-lg">No files yet. Upload your first one above.</p>
    </div>
  {:else}
    <div class="space-y-4">
      {#each files as file (file.id)}
        <div class="card bg-base-100 shadow-md">
          <div class="card-body py-4">
            <div class="flex items-start justify-between gap-4">
              <div class="flex items-center gap-3 flex-1 min-w-0">
                {#if isImage(file.content_type)}
                  <a href={getFileUrl(file.id)} target="_blank" rel="noopener noreferrer" class="shrink-0">
                    <img src={getFileUrl(file.id)} alt={file.original_name} class="size-14 object-cover rounded-lg" />
                  </a>
                {:else}
                  <div class="size-14 rounded-lg bg-base-200 flex items-center justify-center text-2xl shrink-0">
                    📄
                  </div>
                {/if}
                <div class="flex-1 min-w-0">
                  <a
                    href={getFileUrl(file.id)}
                    target="_blank"
                    rel="noopener noreferrer"
                    class="font-semibold leading-tight link link-hover truncate block"
                  >
                    {file.original_name}
                  </a>
                  <p class="text-base-content/60 text-sm mt-1">
                    {formatSize(file.size)} · {file.content_type}
                  </p>
                  <div class="flex items-center gap-2 mt-1">
                    {#if file.public}
                      <span class="badge badge-success badge-sm">Public</span>
                    {:else}
                      <span class="badge badge-ghost badge-sm">Private</span>
                    {/if}
                    <span class="text-base-content/40 text-xs">
                      {new Date(file.created_at).toLocaleDateString()}
                    </span>
                  </div>
                </div>
              </div>
              <button class="btn btn-ghost btn-sm btn-square text-error shrink-0" onclick={() => handleDelete(file.id)} title="Delete">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                  <path stroke-linecap="round" stroke-linejoin="round" d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0" />
                </svg>
              </button>
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}

</div>
