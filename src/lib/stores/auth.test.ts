// src/lib/stores/auth.test.ts
//
// Unit tests for the auth store (src/lib/stores/auth.ts).
// fetch is stubbed globally — no live backend required.
//
// Because $app/environment mock sets browser = true, the store fires an initial
// refreshSession() call on module load.  importAuthStore() pre-queues a 401
// for that call so every test starts from a clean, unauthenticated state.
//
// IMPORTANT: Always add test-specific fetch mocks AFTER importAuthStore()
// completes; mocks added before are consumed by the initial refreshSession().

import { vi, describe, it, expect, beforeEach, afterEach } from 'vitest';
import { get } from 'svelte/store';

// ─── helpers ────────────────────────────────────────────────────────────────

function mockJsonResponse(body: unknown, status = 200): Response {
  return new Response(JSON.stringify(body), {
    status,
    headers: { 'Content-Type': 'application/json' },
  });
}

function mockTextResponse(text: string, status = 400): Response {
  return new Response(text, { status });
}

// ─── setup / teardown ────────────────────────────────────────────────────────

beforeEach(() => {
  vi.stubGlobal('fetch', vi.fn());
});

afterEach(() => {
  vi.unstubAllGlobals();
  vi.clearAllMocks();
});

// ─── module loader ────────────────────────────────────────────────────────────

// Fresh store instance per test.  Pre-queues a 401 for the initial
// refreshSession() that fires when browser = true so call[0] is always
// that bootstrap call and tests can reliably address subsequent calls by index.
async function importAuthStore() {
  vi.resetModules();
  vi.mocked(fetch).mockResolvedValueOnce(new Response(null, { status: 401 }));
  const mod = await import('$lib/stores/auth');
  await new Promise((resolve) => setTimeout(resolve, 10));
  return mod;
}

// ─── derived stores ──────────────────────────────────────────────────────────

describe('derived stores', () => {
  it('isAuthenticated is false when no user is set', async () => {
    const { isAuthenticated } = await importAuthStore();
    expect(get(isAuthenticated)).toBe(false);
  });

  it('currentUser is null when no user is set', async () => {
    const { currentUser } = await importAuthStore();
    expect(get(currentUser)).toBeNull();
  });
});

// ─── login ───────────────────────────────────────────────────────────────────

describe('auth.login()', () => {
  it('POSTs to /login with email and password', async () => {
    const { auth } = await importAuthStore();
    // Mocks added after importAuthStore so they are not consumed by the
    // initial refreshSession (call[0]).  POST /login = call[1].
    vi.mocked(fetch)
      .mockResolvedValueOnce(mockJsonResponse({ success: true }))
      .mockResolvedValueOnce(mockJsonResponse({ id: 'user:1', email: 'a@b.com', role: 'company' }));

    await auth.login('a@b.com', 'secret');

    const loginCall = vi.mocked(fetch).mock.calls[1]; // call[0] = initial refresh
    expect(loginCall[0]).toMatch(/\/login$/);
    expect(loginCall[1]?.method).toBe('POST');

    const body = JSON.parse(loginCall[1]?.body as string);
    expect(body.email).toBe('a@b.com');
    expect(body.password).toBe('secret');
  });

  it('sends credentials: include on the login request', async () => {
    const { auth } = await importAuthStore();
    vi.mocked(fetch)
      .mockResolvedValueOnce(mockJsonResponse({ success: true }))
      .mockResolvedValueOnce(mockJsonResponse({ id: 'user:1', email: 'a@b.com', role: 'company' }));

    await auth.login('a@b.com', 'secret');

    const [, init] = vi.mocked(fetch).mock.calls[1]; // call[1] = POST /login
    expect(init?.credentials).toBe('include');
  });

  it('refreshes the session and populates currentUser after login', async () => {
    const userPayload = { id: 'user:1', email: 'a@b.com', role: 'company' };
    const { auth, currentUser } = await importAuthStore();
    vi.mocked(fetch)
      .mockResolvedValueOnce(mockJsonResponse({ success: true }))  // POST /login
      .mockResolvedValueOnce(mockJsonResponse(userPayload));         // GET /profile

    await auth.login('a@b.com', 'secret');

    expect(get(currentUser)).toEqual(userPayload);
  });

  it('throws when the server returns a non-ok response', async () => {
    const { auth } = await importAuthStore();
    vi.mocked(fetch).mockResolvedValueOnce(mockTextResponse('Invalid credentials', 401));

    await expect(auth.login('a@b.com', 'wrong')).rejects.toThrow('Invalid credentials');
  });

  it('returns true on success', async () => {
    const { auth } = await importAuthStore();
    vi.mocked(fetch)
      .mockResolvedValueOnce(mockJsonResponse({ success: true }))
      .mockResolvedValueOnce(mockJsonResponse({ id: 'user:1', email: 'a@b.com', role: 'company' }));

    const result = await auth.login('a@b.com', 'secret');
    expect(result).toBe(true);
  });
});

// ─── signup ──────────────────────────────────────────────────────────────────

describe('auth.signup()', () => {
  it('POSTs to /signup with required fields', async () => {
    const { auth } = await importAuthStore();
    vi.mocked(fetch)
      .mockResolvedValueOnce(mockJsonResponse({ success: true }))
      .mockResolvedValueOnce(mockJsonResponse({ id: 'user:2', email: 'new@b.com', role: 'user' }));

    await auth.signup('new@b.com', 'pass123', 'New User');

    const signupCall = vi.mocked(fetch).mock.calls[1]; // call[0] = initial refresh
    expect(signupCall[0]).toMatch(/\/signup$/);
    const body = JSON.parse(signupCall[1]?.body as string);
    expect(body.email).toBe('new@b.com');
    expect(body.password).toBe('pass123');
    expect(body.name).toBe('New User');
  });

  it('defaults role to "user" when not specified', async () => {
    const { auth } = await importAuthStore();
    vi.mocked(fetch)
      .mockResolvedValueOnce(mockJsonResponse({ success: true }))
      .mockResolvedValueOnce(mockJsonResponse({ id: 'user:2', email: 'x@x.com', role: 'user' }));

    await auth.signup('x@x.com', 'pw');

    const body = JSON.parse(vi.mocked(fetch).mock.calls[1][1]?.body as string);
    expect(body.role).toBe('user');
  });

  it('throws when signup fails', async () => {
    const { auth } = await importAuthStore();
    vi.mocked(fetch).mockResolvedValueOnce(mockTextResponse('Email already in use', 409));

    await expect(auth.signup('dup@b.com', 'pw')).rejects.toThrow('Email already in use');
  });

  it('refreshes the session and returns true on success', async () => {
    const userPayload = { id: 'user:3', email: 'new@b.com', role: 'user' };
    const { auth, currentUser } = await importAuthStore();
    vi.mocked(fetch)
      .mockResolvedValueOnce(mockJsonResponse({ success: true }))  // POST /signup
      .mockResolvedValueOnce(mockJsonResponse(userPayload));         // GET /profile

    const result = await auth.signup('new@b.com', 'pw', 'Alice');

    expect(result).toBe(true);
    expect(get(currentUser)).toEqual(userPayload);
  });
});

// ─── logout ──────────────────────────────────────────────────────────────────

describe('auth.logout()', () => {
  it('clears the user from the store', async () => {
    const userPayload = { id: 'user:1', email: 'a@b.com', role: 'company' };
    const { auth, currentUser } = await importAuthStore();
    // Login first to seed the store
    vi.mocked(fetch)
      .mockResolvedValueOnce(mockJsonResponse({ success: true }))
      .mockResolvedValueOnce(mockJsonResponse(userPayload));
    await auth.login('a@b.com', 'secret');
    expect(get(currentUser)).toEqual(userPayload);

    // Logout — browser=true means POST /logout is called
    vi.mocked(fetch).mockResolvedValueOnce(mockJsonResponse({ success: true }));
    await auth.logout();

    expect(get(currentUser)).toBeNull();
  });

  it('isAuthenticated becomes false after logout', async () => {
    const userPayload = { id: 'user:1', email: 'a@b.com', role: 'company' };
    const { auth, isAuthenticated } = await importAuthStore();
    vi.mocked(fetch)
      .mockResolvedValueOnce(mockJsonResponse({ success: true }))
      .mockResolvedValueOnce(mockJsonResponse(userPayload));
    await auth.login('a@b.com', 'secret');
    expect(get(isAuthenticated)).toBe(true);

    vi.mocked(fetch).mockResolvedValueOnce(mockJsonResponse({ success: true }));
    await auth.logout();

    expect(get(isAuthenticated)).toBe(false);
  });
});
