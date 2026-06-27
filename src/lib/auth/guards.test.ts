// src/lib/auth/guards.test.ts
// Unit tests for the route-permission guard helpers.
// All functions are pure — no mocks required.

import { describe, it, expect } from 'vitest';
import {
  isRoleAllowedForPath,
  getDefaultRedirectPath,
  isPublicPage,
  isProtectedPath,
  isLandingFooterPage,
  ROUTE_PERMISSIONS,
} from './guards';

// ─── ROUTE_PERMISSIONS sanity ────────────────────────────────────────────────

describe('ROUTE_PERMISSIONS', () => {
  it('is a non-empty array', () => {
    expect(ROUTE_PERMISSIONS.length).toBeGreaterThan(0);
  });

  it('every entry has a non-empty path string', () => {
    for (const perm of ROUTE_PERMISSIONS) {
      expect(typeof perm.path).toBe('string');
      expect(perm.path.startsWith('/')).toBe(true);
    }
  });

  it('every entry has at least one allowed role', () => {
    for (const perm of ROUTE_PERMISSIONS) {
      expect(Array.isArray(perm.allowedRoles)).toBe(true);
      expect(perm.allowedRoles.length).toBeGreaterThan(0);
    }
  });
});

// ─── isRoleAllowedForPath() ──────────────────────────────────────────────────

describe('isRoleAllowedForPath()', () => {
  // ── missing role ────────────────────────────────────────────────────────────

  it('returns false when userRole is undefined', () => {
    expect(isRoleAllowedForPath(undefined, '/jobs')).toBe(false);
  });

  it('returns false when userRole is an empty string', () => {
    expect(isRoleAllowedForPath('', '/jobs')).toBe(false);
  });

  // ── customer routes ─────────────────────────────────────────────────────────

  it('allows a customer to access /start', () => {
    expect(isRoleAllowedForPath('customer', '/start')).toBe(true);
  });

  it('denies admin access to the customer-only /start route', () => {
    expect(isRoleAllowedForPath('admin', '/start')).toBe(false);
  });

  it('allows a customer to access /trackStatus', () => {
    expect(isRoleAllowedForPath('customer', '/trackStatus')).toBe(true);
  });

  it('allows a customer to access /checkout', () => {
    expect(isRoleAllowedForPath('customer', '/checkout')).toBe(true);
  });

  // ── admin-only routes ───────────────────────────────────────────────────────

  it('allows admin to access /adminDashboard', () => {
    expect(isRoleAllowedForPath('admin', '/adminDashboard')).toBe(true);
  });

  it('denies company access to /adminDashboard', () => {
    expect(isRoleAllowedForPath('company', '/adminDashboard')).toBe(false);
  });

  it('allows admin to access /companies', () => {
    expect(isRoleAllowedForPath('admin', '/companies')).toBe(true);
  });

  it('allows admin to access /leads', () => {
    expect(isRoleAllowedForPath('admin', '/leads')).toBe(true);
  });

  // ── admin + company routes ──────────────────────────────────────────────────

  it('allows admin to access /jobs', () => {
    expect(isRoleAllowedForPath('admin', '/jobs')).toBe(true);
  });

  it('allows company to access /jobs', () => {
    expect(isRoleAllowedForPath('company', '/jobs')).toBe(true);
  });

  it('allows employee to access /jobs', () => {
    expect(isRoleAllowedForPath('employee', '/jobs')).toBe(true);
  });

  it('denies distributor access to /jobs', () => {
    expect(isRoleAllowedForPath('distributor', '/jobs')).toBe(false);
  });

  it('allows admin and company to access /employees', () => {
    expect(isRoleAllowedForPath('admin', '/employees')).toBe(true);
    expect(isRoleAllowedForPath('company', '/employees')).toBe(true);
  });

  it('denies employee access to /employees', () => {
    expect(isRoleAllowedForPath('employee', '/employees')).toBe(false);
  });

  it('allows admin and company to access /crews', () => {
    expect(isRoleAllowedForPath('admin', '/crews')).toBe(true);
    expect(isRoleAllowedForPath('company', '/crews')).toBe(true);
  });

  // ── employee routes ─────────────────────────────────────────────────────────

  it('allows employee to access /job (with nested id)', () => {
    expect(isRoleAllowedForPath('employee', '/job/abc123')).toBe(true);
  });

  it('allows admin and company to access /job/:id', () => {
    expect(isRoleAllowedForPath('admin', '/job/abc')).toBe(true);
    expect(isRoleAllowedForPath('company', '/job/abc')).toBe(true);
  });

  it('allows employee to access /training', () => {
    expect(isRoleAllowedForPath('employee', '/training')).toBe(true);
  });

  it('denies company access to /training (employee-only)', () => {
    expect(isRoleAllowedForPath('company', '/training')).toBe(false);
  });

  it('allows employee to access /setSchedule', () => {
    expect(isRoleAllowedForPath('employee', '/setSchedule')).toBe(true);
  });

  // ── distributor routes ──────────────────────────────────────────────────────

  it('allows distributor to access /distributorDashboard', () => {
    expect(isRoleAllowedForPath('distributor', '/distributorDashboard')).toBe(true);
  });

  it('allows distributor to access /trackDelivery', () => {
    expect(isRoleAllowedForPath('distributor', '/trackDelivery')).toBe(true);
  });

  it('denies customer access to /distributorDashboard', () => {
    expect(isRoleAllowedForPath('customer', '/distributorDashboard')).toBe(false);
  });

  // ── unlisted / public paths ─────────────────────────────────────────────────

  it('returns true for any role on an unlisted path (open by default)', () => {
    expect(isRoleAllowedForPath('customer', '/some-unknown-page')).toBe(true);
    expect(isRoleAllowedForPath('admin', '/some-unknown-page')).toBe(true);
  });

  // ── nested path matching ────────────────────────────────────────────────────

  it('allows a customer to access /tileJob/:id (startsWith match)', () => {
    expect(isRoleAllowedForPath('customer', '/tileJob/abc123')).toBe(true);
  });

  it('denies an employee access to /tileJob/:id (customer-only)', () => {
    expect(isRoleAllowedForPath('employee', '/tileJob/abc123')).toBe(false);
  });
});

// ─── getDefaultRedirectPath() ────────────────────────────────────────────────

describe('getDefaultRedirectPath()', () => {
  it('redirects customer to /start', () => {
    expect(getDefaultRedirectPath('customer')).toBe('/start');
  });

  it('redirects admin to /adminDashboard', () => {
    expect(getDefaultRedirectPath('admin')).toBe('/adminDashboard');
  });

  it('redirects company to /jobs', () => {
    expect(getDefaultRedirectPath('company')).toBe('/jobs');
  });

  it('redirects employee to /setSchedule', () => {
    expect(getDefaultRedirectPath('employee')).toBe('/setSchedule');
  });

  it('redirects distributor to /distributorDashboard', () => {
    expect(getDefaultRedirectPath('distributor')).toBe('/distributorDashboard');
  });

  it('redirects an unknown role to /login', () => {
    expect(getDefaultRedirectPath('hacker')).toBe('/login');
  });

  it('redirects undefined to /login', () => {
    expect(getDefaultRedirectPath(undefined)).toBe('/login');
  });
});

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

  it('recognises /apply as public', () => {
    expect(isPublicPage('/apply')).toBe(true);
  });

  it('recognises /forgot-password as public', () => {
    expect(isPublicPage('/forgot-password')).toBe(true);
  });

  it('recognises /get-tile as public', () => {
    expect(isPublicPage('/get-tile')).toBe(true);
  });

  it('recognises /sell-tile as public', () => {
    expect(isPublicPage('/sell-tile')).toBe(true);
  });

  it('recognises /terms as public', () => {
    expect(isPublicPage('/terms')).toBe(true);
  });

  it('recognises /privacy as public', () => {
    expect(isPublicPage('/privacy')).toBe(true);
  });

  it('recognises /invite/:token as public', () => {
    expect(isPublicPage('/invite/abc123token')).toBe(true);
  });

  it('recognises /reset-password/:token as public', () => {
    expect(isPublicPage('/reset-password/sometoken')).toBe(true);
  });

  it('is case-insensitive', () => {
    expect(isPublicPage('/LOGIN')).toBe(true);
    expect(isPublicPage('/Signup')).toBe(true);
    expect(isPublicPage('/INVITE/TOKEN')).toBe(true);
  });

  it('trims whitespace before comparing', () => {
    expect(isPublicPage('  /login  ')).toBe(true);
  });

  it('returns false for /jobs (protected)', () => {
    expect(isPublicPage('/jobs')).toBe(false);
  });

  it('returns false for /adminDashboard (protected)', () => {
    expect(isPublicPage('/adminDashboard')).toBe(false);
  });

  it('returns false for /companyDashboard (protected)', () => {
    expect(isPublicPage('/companyDashboard')).toBe(false);
  });
});

// ─── isLandingFooterPage() ───────────────────────────────────────────────────

describe('isLandingFooterPage()', () => {
  it('returns true for marketing landing pages', () => {
    expect(isLandingFooterPage('/')).toBe(true);
    expect(isLandingFooterPage('/get-tile')).toBe(true);
    expect(isLandingFooterPage('/sell-tile')).toBe(true);
    expect(isLandingFooterPage('/apply')).toBe(true);
  });

  it('returns true for legal pages', () => {
    expect(isLandingFooterPage('/terms')).toBe(true);
    expect(isLandingFooterPage('/privacy')).toBe(true);
  });

  it('returns false for auth pages', () => {
    expect(isLandingFooterPage('/login')).toBe(false);
    expect(isLandingFooterPage('/signup')).toBe(false);
  });

  it('returns false for protected app pages', () => {
    expect(isLandingFooterPage('/jobs')).toBe(false);
  });
});

// ─── isProtectedPath() ───────────────────────────────────────────────────────

describe('isProtectedPath()', () => {
  it('returns true for a protected path like /jobs', () => {
    expect(isProtectedPath('/jobs')).toBe(true);
  });

  it('returns false for a public path like /login', () => {
    expect(isProtectedPath('/login')).toBe(false);
  });

  it('is the exact logical inverse of isPublicPage for every tested path', () => {
    const paths = [
      '/',
      '/login',
      '/signup',
      '/apply',
      '/forgot-password',
      '/get-tile',
      '/sell-tile',
      '/invite/tok',
      '/reset-password/tok',
      '/jobs',
      '/adminDashboard',
      '/employees',
    ];
    for (const path of paths) {
      expect(isProtectedPath(path)).toBe(!isPublicPage(path));
    }
  });
});
