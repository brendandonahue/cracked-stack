// src/lib/auth/guards.ts
export type UserRole = 'user' | 'admin';

/**
 * Pages that don't require authentication.
 */
export function isPublicPage(path: string): boolean {
  if (!path) return false;
  const p = path.toLowerCase().trim();

  const publicPaths = ['/', '/login', '/signup', '/forgot-password'];
  if (publicPaths.includes(p)) return true;

  // /reset-password/<token>
  if (p.startsWith('/reset-password/')) return true;

  return false;
}

/**
 * All authenticated users land on /dashboard after login.
 */
export function getDefaultRedirectPath(_userRole: string | undefined): string {
  return '/dashboard';
}

/**
 * With only two roles (user / admin) there are no role-restricted pages —
 * every authenticated user can reach every protected route.
 * Extend this function if you add role-gated sections.
 */
export function isRoleAllowedForPath(_userRole: string | undefined, _path: string): boolean {
  return true;
}
