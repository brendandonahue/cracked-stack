// src/lib/stores/auth.ts
import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';
import { PUBLIC_API_URL } from '$env/static/public';

export interface User {
  id: string;
  name?: string;
  email: string;
  role: string;  // "user" | "admin"
}

interface Session {
  user: User | null;
}

const API_BASE = PUBLIC_API_URL;

function createAuthStore() {
  const { subscribe, set } = writable<Session>({ user: null });

  async function refreshSession(): Promise<void> {
    if (!browser) return;
    try {
      const res = await fetch(`${API_BASE}/profile`, { credentials: 'include' });
      if (res.ok) {
        const user: User = await res.json();
        set({ user });
      } else {
        set({ user: null });
      }
    } catch (err) {
      console.error('Session refresh failed:', err);
      set({ user: null });
    }
  }

  if (browser) {
    refreshSession();
  }

  return {
    subscribe,

    login: async (email: string, password: string): Promise<boolean> => {
      const res = await fetch(`${API_BASE}/login`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email, password }),
        credentials: 'include',
      });
      if (!res.ok) {
        const errorText = await res.text().catch(() => 'Login failed');
        throw new Error(errorText);
      }
      await refreshSession();
      return true;
    },

    signup: async (email: string, password: string, name?: string, role: string = 'user'): Promise<boolean> => {
      const res = await fetch(`${API_BASE}/signup`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email, password, name, role }),
        credentials: 'include',
      });
      if (!res.ok) {
        const errorText = await res.text().catch(() => 'Signup failed');
        throw new Error(errorText);
      }
      await refreshSession();
      return true;
    },

    logout: async (): Promise<void> => {
      if (browser) {
        await fetch(`${API_BASE}/logout`, {
          method: 'POST',
          credentials: 'include',
        }).catch(() => {});
      }
      set({ user: null });
    },

    refresh: refreshSession,
  };
}

export const auth = createAuthStore();
export const isAuthenticated = derived(auth, ($auth) => !!$auth.user);
export const currentUser = derived(auth, ($auth) => $auth.user);
