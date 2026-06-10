# certify-service frontend

A TLS certificate inventory dashboard for `certify-service-backend`.
Next.js (App Router) + vanilla JavaScript, React Query, Axios, Tailwind CSS,
Lucide, Day.js.

## Quickstart

```bash
npm install
cp .env.example .env.local   # adjust NEXT_PUBLIC_API_URL if needed
npm run dev                  # http://localhost:3000 (redirects to /inventory)
```

The backend is expected at `http://localhost:9168` by default
(`docker compose up --build` in the repo root). While the backend has no list
endpoint yet, a faithful mock is included:

```bash
node scripts/mock-api.mjs    # serves the API shape on :9168
```

## Backend endpoint assumptions

`certify-service-backend` currently exposes `POST /certificate`,
`GET /certificate/{id}`, and `GET /health-check` — but **no list endpoint**.
This UI assumes:

* **`GET /certificates`** returns a JSON array of certificate records in the
  same shape as `GET /certificate/{id}`:
  `{ id, serial_number, subject, issuer, expiration, san_entries, created_at }`

Notes on the real shape:

* The backend serializes its `not_after` column as **`expiration`**
  (`#[serde(rename = "expiration")]` in `domain/certificate.rs`).
* There is **no `not_before`** field, so the UI shows expiry-based validity
  only.

The list assumption is isolated in `lib/api/certificates.js`
(`fetchCertificates`); if the endpoint ships under a different path (e.g. the
singular `GET /certificate`) or adds pagination, only that function changes.
The mock serves the list on both `/certificates` and `/certificate`.

## Architecture

```
app/
  layout.js                 app shell: header, fonts, Providers
  providers.js              client QueryClientProvider (one client per browser session)
  page.js                   redirects / -> /inventory
  inventory/page.js         server component: SSR prefetch + HydrationBoundary

components/
  ui/                       generic primitives (Card, Badge, StatCard, Loading/Error/EmptyState)
  certificates/             feature components (Dashboard, Table, Row, Details, StatusBadge, InventoryView)

lib/
  api/client.js             shared Axios instance + error-message normalizer
  api/certificates.js       endpoint functions + React Query cache keys
  utils/cn.js               clsx + tailwind-merge composition
  utils/certificates.js     Day.js domain logic (status, expiry math, formatting)

hooks/
  useCertificates.js        inventory query hook
```

Key decisions:

* **SSR + hydration** — `app/inventory/page.js` is a server component that
  prefetches the list into a fresh `QueryClient` and dehydrates it into a
  `HydrationBoundary`. The client hook (`useCertificates`) uses the same query
  key, so the first client render is instant with server data and React Query
  owns freshness from there. `prefetchQuery` swallows SSR fetch errors, so a
  dead backend degrades to the client-side error state instead of a 500.
* **Query keys live in `lib/api`**, not the hook, because the hook is a
  `"use client"` module and the server page must import the same keys.
* **Domain logic is framework-free** — status classification, expiry math, and
  DN parsing live in `lib/utils/certificates.js` as pure functions (with an
  injectable `now` for testability), so components stay presentational.
* **Status rendering has one source of truth** —
  `CertificateStatusBadge` maps status → icon/label/variant; the table, the
  details panel, and (conceptually) the dashboard all derive from
  `getCertificateStatus`.

## UX notes

* Loading, error (with retry), and empty states are all handled in
  `InventoryView`.
* Row selection is accessible: the subject cell is a real `<button>` with
  `aria-pressed`; the whole-row click is a pointer convenience.
* Issuer / SAN columns collapse on smaller viewports; the details panel stacks
  under the table below `lg`.
* Palette: neutral slate surfaces, blue for primary actions, green = valid,
  amber = expiring within 30 days, red = expired/errors only.
