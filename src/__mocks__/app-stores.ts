// src/__mocks__/app-stores.ts
// Stub for $app/stores used during tests
import { vi } from 'vitest';
import { writable, readable } from 'svelte/store';

export const page = readable({ url: new URL('http://localhost/'), params: {}, data: {} });
export const navigating = readable(null);
export const updated = readable(false);
