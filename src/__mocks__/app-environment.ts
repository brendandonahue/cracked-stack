// src/__mocks__/app-environment.ts
// Stub for $app/environment used during tests.
// happy-dom provides a browser-like environment, so browser = true here so that
// auth-store logic that guards on `if (browser)` behaves the same as in-app.
export const browser = true;
export const building = false;
export const dev = true;
export const version = 'test';
