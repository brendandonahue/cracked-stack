// src/lib/stores/toast.test.ts
// Unit tests for the toast store.
// vi.useFakeTimers() is used to control auto-dismiss behaviour.

import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { get } from 'svelte/store';
import { toastStore } from './toast';

// ─── setup / teardown ────────────────────────────────────────────────────────

beforeEach(() => {
  vi.useFakeTimers();
  // Always start each test with an empty store
  toastStore.clearAll();
});

afterEach(() => {
  toastStore.clearAll();
  vi.useRealTimers();
  vi.clearAllMocks();
});

// ─── addToast ────────────────────────────────────────────────────────────────

describe('toastStore.addToast()', () => {
  it('adds a toast to the store', () => {
    toastStore.addToast({ message: 'Hello', type: 'success' });
    expect(get(toastStore)).toHaveLength(1);
  });

  it('stores the correct message and type', () => {
    toastStore.addToast({ message: 'Hello', type: 'error' });
    const [toast] = get(toastStore);
    expect(toast.message).toBe('Hello');
    expect(toast.type).toBe('error');
  });

  it('assigns a string id to each toast', () => {
    toastStore.addToast({ message: 'Test', type: 'info' });
    const [toast] = get(toastStore);
    expect(typeof toast.id).toBe('string');
    expect(toast.id.length).toBeGreaterThan(0);
  });

  it('assigns unique ids to each toast', () => {
    toastStore.addToast({ message: 'First', type: 'success' });
    toastStore.addToast({ message: 'Second', type: 'info' });
    const [a, b] = get(toastStore);
    expect(a.id).not.toBe(b.id);
  });

  it('uses a default duration of 5000 ms when none is provided', () => {
    toastStore.addToast({ message: 'Test', type: 'success' });
    const [toast] = get(toastStore);
    expect(toast.duration).toBe(5000);
  });

  it('respects a custom duration', () => {
    toastStore.addToast({ message: 'Quick', type: 'info', duration: 1000 });
    const [toast] = get(toastStore);
    expect(toast.duration).toBe(1000);
  });

  it('returns the id of the new toast', () => {
    const id = toastStore.addToast({ message: 'Test', type: 'warning' });
    expect(typeof id).toBe('string');
    expect(id.length).toBeGreaterThan(0);
  });

  it('appends toasts in insertion order', () => {
    toastStore.addToast({ message: 'First', type: 'success' });
    toastStore.addToast({ message: 'Second', type: 'error' });
    const toasts = get(toastStore);
    expect(toasts[0].message).toBe('First');
    expect(toasts[1].message).toBe('Second');
  });

  it('auto-removes a toast after its duration elapses', () => {
    toastStore.addToast({ message: 'Fleeting', type: 'info', duration: 500 });
    expect(get(toastStore)).toHaveLength(1);

    vi.advanceTimersByTime(500);

    expect(get(toastStore)).toHaveLength(0);
  });

  it('does not remove a toast before its duration elapses', () => {
    toastStore.addToast({ message: 'Still here', type: 'info', duration: 1000 });
    vi.advanceTimersByTime(999);
    expect(get(toastStore)).toHaveLength(1);
  });

  it('keeps a toast indefinitely when duration is 0', () => {
    toastStore.addToast({ message: 'Sticky', type: 'warning', duration: 0 });
    vi.advanceTimersByTime(60_000);
    expect(get(toastStore)).toHaveLength(1);
  });
});

// ─── removeToast ─────────────────────────────────────────────────────────────

describe('toastStore.removeToast()', () => {
  it('removes the toast with the given id', () => {
    const id = toastStore.addToast({ message: 'Remove me', type: 'success' });
    toastStore.removeToast(id);
    expect(get(toastStore)).toHaveLength(0);
  });

  it('only removes the targeted toast, leaving others intact', () => {
    toastStore.addToast({ message: 'Keep me', type: 'info' });
    const id = toastStore.addToast({ message: 'Remove me', type: 'error' });
    toastStore.removeToast(id);

    const toasts = get(toastStore);
    expect(toasts).toHaveLength(1);
    expect(toasts[0].message).toBe('Keep me');
  });

  it('is a no-op when the id does not exist', () => {
    toastStore.addToast({ message: 'Test', type: 'info' });
    toastStore.removeToast('nonexistent-id');
    expect(get(toastStore)).toHaveLength(1);
  });
});

// ─── clearAll ────────────────────────────────────────────────────────────────

describe('toastStore.clearAll()', () => {
  it('removes all toasts at once', () => {
    toastStore.addToast({ message: 'A', type: 'success' });
    toastStore.addToast({ message: 'B', type: 'error' });
    toastStore.addToast({ message: 'C', type: 'warning' });

    toastStore.clearAll();

    expect(get(toastStore)).toHaveLength(0);
  });

  it('does not throw when the store is already empty', () => {
    expect(() => toastStore.clearAll()).not.toThrow();
    expect(get(toastStore)).toHaveLength(0);
  });
});

// ─── convenience methods ──────────────────────────────────────────────────────

describe('toastStore.success()', () => {
  it('adds a success toast with the given message', () => {
    toastStore.success('It worked!');
    const [toast] = get(toastStore);
    expect(toast.type).toBe('success');
    expect(toast.message).toBe('It worked!');
  });

  it('accepts an optional custom duration', () => {
    toastStore.success('Fast', 1000);
    const [toast] = get(toastStore);
    expect(toast.duration).toBe(1000);
  });
});

describe('toastStore.error()', () => {
  it('adds an error toast with the given message', () => {
    toastStore.error('Something broke');
    const [toast] = get(toastStore);
    expect(toast.type).toBe('error');
    expect(toast.message).toBe('Something broke');
  });
});

describe('toastStore.info()', () => {
  it('adds an info toast with the given message', () => {
    toastStore.info('Just so you know');
    const [toast] = get(toastStore);
    expect(toast.type).toBe('info');
    expect(toast.message).toBe('Just so you know');
  });
});

describe('toastStore.warning()', () => {
  it('adds a warning toast with the given message', () => {
    toastStore.warning('Watch out!');
    const [toast] = get(toastStore);
    expect(toast.type).toBe('warning');
    expect(toast.message).toBe('Watch out!');
  });
});
