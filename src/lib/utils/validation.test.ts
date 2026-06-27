// src/lib/utils/validation.test.ts
// Unit tests for validateEmail() and validatePhone().
// Pure functions — no mocks required.

import { describe, it, expect } from 'vitest';
import { validateEmail, validatePhone } from './validation';

// ─── validateEmail ────────────────────────────────────────────────────────────

describe('validateEmail()', () => {
  // ── valid inputs ────────────────────────────────────────────────────────────

  it('returns null for a standard valid email', () => {
    expect(validateEmail('user@example.com')).toBeNull();
  });

  it('returns null for an empty string (required check is handled elsewhere)', () => {
    expect(validateEmail('')).toBeNull();
  });

  it('accepts a subdomain', () => {
    expect(validateEmail('user@mail.example.com')).toBeNull();
  });

  it('accepts a plus-sign alias', () => {
    expect(validateEmail('user+tag@example.com')).toBeNull();
  });

  it('accepts dots in the local part', () => {
    expect(validateEmail('first.last@example.com')).toBeNull();
  });

  it('accepts underscores in the local part', () => {
    expect(validateEmail('first_last@example.com')).toBeNull();
  });

  it('accepts a two-character TLD', () => {
    expect(validateEmail('user@example.io')).toBeNull();
  });

  it('accepts a long TLD', () => {
    expect(validateEmail('user@example.travel')).toBeNull();
  });

  // ── invalid inputs ──────────────────────────────────────────────────────────

  it('returns an error message for a missing @', () => {
    expect(validateEmail('notanemail')).toBe('Please enter a valid email address');
  });

  it('returns an error for a missing domain after @', () => {
    expect(validateEmail('user@')).toBe('Please enter a valid email address');
  });

  it('returns an error for a missing TLD', () => {
    expect(validateEmail('user@domain')).toBe('Please enter a valid email address');
  });

  it('returns an error for a single-char TLD', () => {
    expect(validateEmail('user@example.c')).toBe('Please enter a valid email address');
  });

  it('returns an error for spaces in the address', () => {
    expect(validateEmail('user @example.com')).toBe('Please enter a valid email address');
  });

  it('returns an error for a bare @ sign', () => {
    expect(validateEmail('@')).toBe('Please enter a valid email address');
  });

  it('returns an error for double @', () => {
    expect(validateEmail('user@@example.com')).toBe('Please enter a valid email address');
  });
});

// ─── validatePhone ────────────────────────────────────────────────────────────

describe('validatePhone()', () => {
  // ── valid inputs ────────────────────────────────────────────────────────────

  it('returns null for an empty string', () => {
    expect(validatePhone('')).toBeNull();
  });

  it('accepts (xxx) xxx-xxxx format', () => {
    expect(validatePhone('(203) 555-1234')).toBeNull();
  });

  it('accepts xxx-xxx-xxxx dashed format', () => {
    expect(validatePhone('203-555-1234')).toBeNull();
  });

  it('accepts xxx.xxx.xxxx dot format', () => {
    expect(validatePhone('203.555.1234')).toBeNull();
  });

  it('accepts plain 10-digit number', () => {
    expect(validatePhone('2035551234')).toBeNull();
  });

  it('accepts +1 country code with area code in parens', () => {
    expect(validatePhone('+1 (203) 555-1234')).toBeNull();
  });

  it('accepts +1 followed by plain digits', () => {
    expect(validatePhone('+12035551234')).toBeNull();
  });

  it('trims leading and trailing whitespace before validating', () => {
    expect(validatePhone('  (203) 555-1234  ')).toBeNull();
  });

  // ── invalid inputs ──────────────────────────────────────────────────────────

  it('returns an error for a 3-digit number', () => {
    expect(validatePhone('123')).toBe(
      'Please enter a valid phone number (e.g., (203) 555-1234 or 203-555-1234)'
    );
  });

  it('returns an error for alphabetic input', () => {
    expect(validatePhone('abc-def-ghij')).toBe(
      'Please enter a valid phone number (e.g., (203) 555-1234 or 203-555-1234)'
    );
  });

  it('returns an error for a 9-digit number (one digit short)', () => {
    expect(validatePhone('203555123')).toBe(
      'Please enter a valid phone number (e.g., (203) 555-1234 or 203-555-1234)'
    );
  });

  it('returns an error for an 11-digit number without +1 prefix', () => {
    expect(validatePhone('12035551234')).toBe(
      'Please enter a valid phone number (e.g., (203) 555-1234 or 203-555-1234)'
    );
  });
});
