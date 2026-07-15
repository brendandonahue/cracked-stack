// src/lib/api.test.ts
// Unit tests for the file-upload helpers in api.ts.
// global.fetch is mocked so no real network calls are made.

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { uploadFile, getFileUrl, deleteFile } from './api';

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
