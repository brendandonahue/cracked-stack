// src/lib/auth/guards.test.ts
// Unit tests for the route-permission guard helpers.
// All functions are pure — no mocks required.

import { describe, it, expect } from 'vitest';
import {
  isRoleAllowedForPath,
  getDefaultRedirectPath,
  isPublicPage,
  isProtectedPath,
} from './guards';

// ─── isPublicPage() ───────────────────────────────────────────────────────────

describe('isPublicPage()', () => {
  it('returns false for an empty string', () => {
    expect(isPublicPage('')).toBe(false);
  });

  it('recognises / as public', () => {
    expect(isPublicPage('/')).toBe(true);
  });

  it('recognises /login as public', () => {
    expect(isPublicPage('/login')).toBe(true);
  });

  it('recognises /signup as public', () => {
    expect(isPublicPage('/signup')).toBe(true);
  });

  it('recognises /forgot-password as public', () => {
    expect(isPublicPage('/forgot-password')).toBe(true);
  });

  it('recognises /reset-password/:token as public', () => {
    expect(isPublicPage('/reset-password/sometoken')).toBe(true);
  });

  it('is case-insensitive', () => {
    expect(isPublicPage('/LOGIN')).toBe(true);
    expect(isPublicPage('/Signup')).toBe(true);
    expect(isPublicPage('/RESET-PASSWORD/TOKEN')).toBe(true);
  });

  it('trims whitespace before comparing', () => {
    expect(isPublicPage('  /login  ')).toBe(true);
  });

  it('returns false for protected app routes', () => {
    expect(isPublicPage('/dashboard')).toBe(false);
    expect(isPublicPage('/items')).toBe(false);
    expect(isPublicPage('/files')).toBe(false);
    expect(isPublicPage('/profile')).toBe(false);
  });
});

// ─── isProtectedPath() ───────────────────────────────────────────────────────

describe('isProtectedPath()', () => {
  it('returns true for a protected path like /dashboard', () => {
    expect(isProtectedPath('/dashboard')).toBe(true);
  });

  it('returns false for a public path like /login', () => {
    expect(isProtectedPath('/login')).toBe(false);
  });

  it('is the exact logical inverse of isPublicPage for every tested path', () => {
    const paths = [
      '/',
      '/login',
      '/signup',
      '/forgot-password',
      '/reset-password/tok',
      '/dashboard',
      '/items',
      '/files',
      '/profile',
    ];
    for (const path of paths) {
      expect(isProtectedPath(path)).toBe(!isPublicPage(path));
    }
  });
});

// ─── getDefaultRedirectPath() ────────────────────────────────────────────────

describe('getDefaultRedirectPath()', () => {
  it('redirects any authenticated role to /dashboard', () => {
    expect(getDefaultRedirectPath('user')).toBe('/dashboard');
    expect(getDefaultRedirectPath('admin')).toBe('/dashboard');
  });

  it('redirects undefined role to /dashboard', () => {
    expect(getDefaultRedirectPath(undefined)).toBe('/dashboard');
  });
});

// ─── isRoleAllowedForPath() ──────────────────────────────────────────────────

describe('isRoleAllowedForPath()', () => {
  it('allows any role on protected routes (no role gating in the template)', () => {
    expect(isRoleAllowedForPath('user', '/dashboard')).toBe(true);
    expect(isRoleAllowedForPath('admin', '/items')).toBe(true);
    expect(isRoleAllowedForPath('user', '/profile')).toBe(true);
  });

  it('allows undefined role (layout handles auth separately)', () => {
    expect(isRoleAllowedForPath(undefined, '/dashboard')).toBe(true);
  });

  it('returns true for unknown paths (open by default)', () => {
    expect(isRoleAllowedForPath('user', '/some-unknown-page')).toBe(true);
  });
});
