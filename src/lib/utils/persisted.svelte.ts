// src/lib/utils/persisted.ts
import { browser } from '$app/environment';

export function persisted<T>(
  key: string,
  initial: T,
  storage = localStorage // or sessionStorage
): { value: T } {
  let value = $state(initial);

  // Load once from storage (only in browser)
  if (browser) {
    const stored = storage.getItem(key);
    try {
      if (stored !== null) {
        console.log(`[persisted] Attempting parse for key: "${key}"`);
        console.log(`[persisted] Stored value preview:`, stored.substring(0, 200));
        value = JSON.parse(stored);
      }
    } catch (e) {
      console.error(`[persisted] Parse failed for key "${key}":`, e);
      console.error(`[persisted] Bad stored value was:`, stored?.substring(0, 500) || 'null');
      value = initial;
      storage.removeItem(key);
    }
  }

  // Auto-save on change
  const cleanup = $effect.root(() => {
    $effect(() => {
      if (!browser) return;

      try {
        if (value === undefined || value === null) {
          storage.removeItem(key);
        } else {
          const jsonString = JSON.stringify(value);
          // Quick safety check: if it looks like HTML, don't save
          if (jsonString.startsWith('<!') || jsonString.includes('<html')) {
            console.warn(`Refusing to save HTML-like value to "${key}"`);
            storage.removeItem(key);
            return;
          }
          storage.setItem(key, jsonString);
        }
      } catch (e) {
        console.warn(`Failed to save "${key}"`, e);
      }
    });

    // Cross-tab sync (also inside root so it can be cleaned up)
    if (browser) {
      const handleStorage = (e: StorageEvent) => {
        if (e.key !== key) return;

        if (e.newValue === null) {
          value = initial;
        } else {
          try {
            value = JSON.parse(e.newValue) as T;
          } catch {}
        }
      };

      window.addEventListener('storage', handleStorage);

      // Return cleanup for the listener
      return () => {
        window.removeEventListener('storage', handleStorage);
      };
    }
  });

   return { value };
}