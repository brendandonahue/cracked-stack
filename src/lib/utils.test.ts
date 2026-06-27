// src/lib/utils.test.ts
// Unit tests for cn() — the clsx + tailwind-merge helper.
// No mocks required; this is a pure utility function.

import { describe, it, expect } from 'vitest';
import { cn } from './utils';

describe('cn()', () => {
  it('returns an empty string when called with no arguments', () => {
    expect(cn()).toBe('');
  });

  it('returns a single class name unchanged', () => {
    expect(cn('foo')).toBe('foo');
  });

  it('joins multiple class names with a space', () => {
    expect(cn('foo', 'bar', 'baz')).toBe('foo bar baz');
  });

  it('ignores undefined values', () => {
    expect(cn('foo', undefined, 'bar')).toBe('foo bar');
  });

  it('ignores null values', () => {
    expect(cn('foo', null, 'bar')).toBe('foo bar');
  });

  it('ignores false values', () => {
    expect(cn('foo', false, 'bar')).toBe('foo bar');
  });

  it('ignores 0 as a falsy value', () => {
    // clsx treats 0 as falsy
    expect(cn('foo', 0 as any, 'bar')).toBe('foo bar');
  });

  it('includes truthy string values from a conditional object', () => {
    expect(cn({ foo: true, bar: false, baz: true })).toBe('foo baz');
  });

  it('handles array inputs', () => {
    expect(cn(['foo', 'bar'])).toBe('foo bar');
  });

  it('handles nested arrays', () => {
    expect(cn(['foo', ['bar', 'baz']])).toBe('foo bar baz');
  });

  // tailwind-merge specific: last conflicting class wins
  it('resolves conflicting padding classes (last wins)', () => {
    expect(cn('p-4', 'p-2')).toBe('p-2');
  });

  it('resolves conflicting text-color classes (last wins)', () => {
    expect(cn('text-red-500', 'text-blue-500')).toBe('text-blue-500');
  });

  it('keeps non-conflicting Tailwind utilities from both arguments', () => {
    const result = cn('py-1 px-2', 'px-4');
    // py-1 is kept, px-2 is replaced by px-4
    expect(result).toContain('py-1');
    expect(result).toContain('px-4');
    expect(result).not.toContain('px-2');
  });

  it('merges a mix of strings, objects, and arrays', () => {
    const result = cn('base', { active: true, disabled: false }, ['extra']);
    expect(result).toBe('base active extra');
  });
});
