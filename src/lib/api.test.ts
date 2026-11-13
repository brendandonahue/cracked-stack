// src/lib/api.test.ts
// Unit tests for the file-upload and Stripe helpers in api.ts.
// global.fetch is mocked so no real network calls are made.

import { describe, it, expect, beforeEach, vi } from 'vitest';
import {
  uploadFile,
  getFileUrl,
  deleteFile,
  getStripePlans,
  createCheckoutSession,
  getPayments,
  formatMoney,
} from './api';

const BASE_URL = 'http://localhost:8000'; // matches src/__mocks__/env-public.ts

function mockFetchResponse(body: any, ok = true, status = 200) {
  return {
    ok,
    status,
    json: () => Promise.resolve(body),
    text: () => Promise.resolve(JSON.stringify(body)),
  } as Response;
}

beforeEach(() => {
  vi.stubGlobal('fetch', vi.fn());
});

describe('uploadFile()', () => {
  it('POSTs a FormData body to /upload with credentials included', async () => {
    const fakeResult = { id: 'abc123', url: '/files/abc123', filename: 'abc123.png', size: 42, public: false };
    (fetch as any).mockResolvedValueOnce(mockFetchResponse(fakeResult));

    const file = new File(['hello'], 'hello.png', { type: 'image/png' });
    const result = await uploadFile(file);

    expect(fetch).toHaveBeenCalledTimes(1);
    const [url, options] = (fetch as any).mock.calls[0];
    expect(url).toBe(`${BASE_URL}/upload`);
    expect(options.method).toBe('POST');
    expect(options.credentials).toBe('include');
    expect(options.body).toBeInstanceOf(FormData);
    expect(options.body.get('file')).toBe(file);
    expect(result).toEqual(fakeResult);
  });

  it('includes a public field in the form data when specified', async () => {
    (fetch as any).mockResolvedValueOnce(mockFetchResponse({ id: '1', url: '/files/1', filename: '1', size: 1, public: true }));

    const file = new File(['x'], 'x.txt', { type: 'text/plain' });
    await uploadFile(file, { public: true });

    const [, options] = (fetch as any).mock.calls[0];
    expect(options.body.get('public')).toBe('true');
  });

  it('omits the public field when not specified', async () => {
    (fetch as any).mockResolvedValueOnce(mockFetchResponse({ id: '1', url: '/files/1', filename: '1', size: 1, public: false }));

    const file = new File(['x'], 'x.txt', { type: 'text/plain' });
    await uploadFile(file);

    const [, options] = (fetch as any).mock.calls[0];
    expect(options.body.get('public')).toBeNull();
  });

  it('throws when the server responds with a non-ok status', async () => {
    (fetch as any).mockResolvedValueOnce(mockFetchResponse('upload failed', false, 500));

    const file = new File(['x'], 'x.txt', { type: 'text/plain' });
    await expect(uploadFile(file)).rejects.toThrow();
  });
});

describe('getFileUrl()', () => {
  it('builds the correct public file URL', () => {
    expect(getFileUrl('abc123')).toBe(`${BASE_URL}/files/abc123`);
  });
});

describe('deleteFile()', () => {
  it('sends a DELETE request to /files/<id> with credentials included', async () => {
    (fetch as any).mockResolvedValueOnce(mockFetchResponse({ success: true }));

    await deleteFile('abc123');

    expect(fetch).toHaveBeenCalledTimes(1);
    const [url, options] = (fetch as any).mock.calls[0];
    expect(url).toBe(`${BASE_URL}/files/abc123`);
    expect(options.method).toBe('DELETE');
    expect(options.credentials).toBe('include');
  });

  it('throws when the server responds with a non-ok status', async () => {
    (fetch as any).mockResolvedValueOnce(mockFetchResponse('not found', false, 404));
    await expect(deleteFile('missing')).rejects.toThrow();
  });
});

// ─── Stripe ────────────────────────────────────────────────────────────────────────────────────────

describe('getStripePlans()', () => {
  it('GETs /stripe/plans with credentials and returns the catalogue', async () => {
    const body = {
      configured: true,
      plans: [{ id: 'pro', name: 'Pro', description: 'd', amount: 1900, currency: 'usd' }],
    };
    (fetch as any).mockResolvedValueOnce(mockFetchResponse(body));

    const result = await getStripePlans();

    const [url, options] = (fetch as any).mock.calls[0];
    expect(url).toBe(`${BASE_URL}/stripe/plans`);
    expect(options.credentials).toBe('include');
    expect(result).toEqual(body);
  });

  it('throws when the server responds with a non-ok status', async () => {
    (fetch as any).mockResolvedValueOnce(mockFetchResponse('boom', false, 500));
    await expect(getStripePlans()).rejects.toThrow();
  });
});

describe('createCheckoutSession()', () => {
  it('POSTs only the plan id to /stripe/checkout and returns the Stripe URL', async () => {
    (fetch as any).mockResolvedValueOnce(mockFetchResponse({ url: 'https://checkout.stripe.com/c/pay/cs_test_1' }));

    const url = await createCheckoutSession('pro');

    expect(fetch).toHaveBeenCalledTimes(1);
    const [endpoint, options] = (fetch as any).mock.calls[0];
    expect(endpoint).toBe(`${BASE_URL}/stripe/checkout`);
    expect(options.method).toBe('POST');
    expect(options.credentials).toBe('include');
    // The client never sends an amount — prices are owned by the server.
    expect(JSON.parse(options.body)).toEqual({ plan_id: 'pro' });
    expect(url).toBe('https://checkout.stripe.com/c/pay/cs_test_1');
  });

  it('throws when Stripe is not configured or the plan is rejected', async () => {
    (fetch as any).mockResolvedValueOnce(mockFetchResponse({ error: 'Payments are not configured' }, false, 503));
    await expect(createCheckoutSession('pro')).rejects.toThrow();
  });
});

describe('getPayments()', () => {
  it('queries the payment table through /graphql', async () => {
    (fetch as any).mockResolvedValueOnce(mockFetchResponse({ data: { payment: [] } }));

    await getPayments();

    const [url, options] = (fetch as any).mock.calls[0];
    expect(url).toBe(`${BASE_URL}/graphql`);
    expect(options.method).toBe('POST');
    const { query } = JSON.parse(options.body);
    expect(query).toContain('payment {');
    expect(query).not.toContain('\n');
  });

  it('returns payments newest first', async () => {
    const older = { id: 'payment:a', created_at: '2026-01-01T00:00:00Z' };
    const newer = { id: 'payment:b', created_at: '2026-02-01T00:00:00Z' };
    (fetch as any).mockResolvedValueOnce(mockFetchResponse({ data: { payment: [older, newer] } }));

    const result = await getPayments();

    expect(result.map((p) => p.id)).toEqual(['payment:b', 'payment:a']);
  });

  it('returns an empty list when GraphQL returns no payment data', async () => {
    (fetch as any).mockResolvedValueOnce(mockFetchResponse({ data: null }));
    expect(await getPayments()).toEqual([]);
  });
});

describe('formatMoney()', () => {
  it('formats minor units as a currency string', () => {
    expect(formatMoney(1900, 'usd')).toBe('$19.00');
    expect(formatMoney(500, 'USD')).toBe('$5.00');
    expect(formatMoney(0, 'usd')).toBe('$0.00');
  });

  it('falls back to a plain amount for an invalid currency code', () => {
    expect(formatMoney(1234, 'not-a-currency')).toBe('12.34 NOT-A-CURRENCY');
  });
});