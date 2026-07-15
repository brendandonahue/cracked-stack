// src/lib/api.ts
//
// All backend HTTP calls go through `apiFetch` so that the httpOnly
// auth_token cookie is always forwarded to the Rocket API.
// For CRUD operations prefer `graphqlQuery` over new REST endpoints.

import { PUBLIC_API_URL } from '$env/static/public';

const BASE_URL = PUBLIC_API_URL;

export async function apiFetch(
  endpoint: string,
  options: RequestInit = {}
): Promise<Response> {
  const url = `${BASE_URL}${endpoint.startsWith('/') ? '' : '/'}${endpoint}`;
  return fetch(url, {
    ...options,
    credentials: 'include', // ← sends the httpOnly auth_token cookie
    headers: {
      'Content-Type': 'application/json',
      ...options.headers,
    },
  });
}

async function handleResponse<T = any>(res: Response): Promise<T> {
  if (!res.ok) {
    const errorText = await res.text().catch(() => 'Request failed');
    throw new Error(errorText);
  }
  return res.json();
}

// ──────────────────────────────────────────────────────────────
// Auth
// ──────────────────────────────────────────────────────────────

export async function getProfile() {
  const res = await apiFetch('/profile');
  return handleResponse(res);
}

// ──────────────────────────────────────────────────────────────
// GraphQL  (POST /graphql — proxied by Rocket to SurrealDB)
// Note: queries must be single-line strings; newlines break the proxy.
// ──────────────────────────────────────────────────────────────

export async function graphqlQuery(query: string, variables: any = {}) {
  const res = await apiFetch('/graphql', {
    method: 'POST',
    body: JSON.stringify({ query, variables }),
  });
  return handleResponse(res);
}

// ──────────────────────────────────────────────────────────────
// Items  (generic demo resource)
// Replace this section with your own domain model.
// ──────────────────────────────────────────────────────────────

export interface Item {
  id: string;
  title: string;
  description?: string;
  created_by?: { id: string; name: string };
  created_at: string;
  updated_at: string;
}

export async function getItems(): Promise<Item[]> {
  const query = `query getItems { item { id title description created_by { id name } created_at updated_at } }`;
  const result = await graphqlQuery(query);
  return result.data?.item ?? [];
}

export async function createItem(data: { title: string; description?: string }): Promise<Item> {
  const query = `mutation createItem($data: CreateItemInput!) { createItem(data: $data) { id title description created_at updated_at } }`;
  const result = await graphqlQuery(query, {
    data: { ...data, created_at: new Date().toISOString(), updated_at: new Date().toISOString() },
  });
  return result.data.createItem;
}

export async function updateItem(id: string, data: { title?: string; description?: string }): Promise<Item> {
  const recordId = id.split(':').pop() ?? id;
  const query = `mutation updateItem($id: ID!, $data: UpdateItemInput!) { updateItem(id: $id, data: $data) { id title description updated_at } }`;
  const result = await graphqlQuery(query, {
    id: recordId,
    data: { ...data, updated_at: new Date().toISOString() },
  });
  return result.data.updateItem;
}

export async function deleteItem(id: string): Promise<void> {
  const recordId = id.split(':').pop() ?? id;
  const query = `mutation deleteItem($id: ID!) { deleteItem(id: $id) }`;
  await graphqlQuery(query, { id: recordId });
}

// ──────────────────────────────────────────────────────────────
// File uploads  (POST /upload, GET /files/<id>, DELETE /files/<id>)
// Uses a raw fetch (not apiFetch) so the browser can set its own
// multipart/form-data boundary; apiFetch always forces
// Content-Type: application/json.
// ──────────────────────────────────────────────────────────────

export interface UploadedFile {
  id: string;
  url: string;
  filename: string;
  size: number;
  public: boolean;
}

export async function uploadFile(file: File, opts: { public?: boolean } = {}): Promise<UploadedFile> {
  const formData = new FormData();
  formData.append('file', file);
  if (opts.public !== undefined) {
    formData.append('public', String(opts.public));
  }

  const res = await fetch(`${BASE_URL}/upload`, {
    method: 'POST',
    credentials: 'include',
    body: formData,
  });

  return handleResponse<UploadedFile>(res);
}

export function getFileUrl(id: string): string {
  return `${BASE_URL}/files/${id}`;
}

export async function deleteFile(id: string): Promise<void> {
  const res = await apiFetch(`/files/${id}`, { method: 'DELETE' });
  await handleResponse(res);
}


