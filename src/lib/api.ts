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

// ──────────────────────────────────────────────────────────────
// Stripe payments  (GET /stripe/plans, POST /stripe/checkout)
// Prices live on the server. The browser only picks a plan id and is then
// redirected to Stripe-hosted Checkout. Payments are written by the signed
// webhook (POST /stripe/webhook, called by Stripe — never by the frontend) and
// read back through GraphQL, where table PERMISSIONS scope them to the owner.
// ──────────────────────────────────────────────────────────────

export interface StripePlan {
  id: string;
  name: string;
  description: string;
  /** Smallest currency unit (cents). */
  amount: number;
  currency: string;
}

export interface StripePlans {
  /** False when the server has no STRIPE_SECRET_KEY, so Checkout would fail. */
  configured: boolean;
  plans: StripePlan[];
}

export interface Payment {
  id: string;
  session_id: string;
  plan_id: string;
  plan_name: string;
  amount: number;
  currency: string;
  status: string;
  created_at: string;
}

export async function getStripePlans(): Promise<StripePlans> {
  const res = await apiFetch('/stripe/plans');
  return handleResponse<StripePlans>(res);
}

/** Creates a Stripe Checkout Session and returns the hosted-page URL to redirect to. */
export async function createCheckoutSession(planId: string): Promise<string> {
  const res = await apiFetch('/stripe/checkout', {
    method: 'POST',
    body: JSON.stringify({ plan_id: planId }),
  });
  const data = await handleResponse<{ url: string }>(res);
  return data.url;
}

export async function getPayments(): Promise<Payment[]> {
  const query = `query getPayments { payment { id session_id plan_id plan_name amount currency status created_at } }`;
  const result = await graphqlQuery(query);
  return (result.data?.payment ?? [])
    .slice()
    .sort((a: Payment, b: Payment) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime());
}

/** Formats a minor-unit amount (e.g. 1900, 'usd') as a localized currency string ($19.00). */
export function formatMoney(amount: number, currency: string): string {
  try {
    return new Intl.NumberFormat('en-US', { style: 'currency', currency: currency.toUpperCase() }).format(amount / 100);
  } catch {
    return `${(amount / 100).toFixed(2)} ${currency.toUpperCase()}`;
  }
}