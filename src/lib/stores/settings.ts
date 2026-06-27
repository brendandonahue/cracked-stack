// src/lib/stores/settings.ts
import { persisted } from '$lib/utils/persisted.svelte';

export const settings = persisted('app:settings', {
  name: "",
  role: "",
});