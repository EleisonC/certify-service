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

The backend is expected at `https://localhost:9168` by default
(refer to `README.md` for details).

## Run with Docker

From the repo root, `docker compose up --build` starts the database, backend,
and this frontend (http://localhost:3000).

The image is a multi-stage build (`Dockerfile`) producing Next.js standalone
output (`output: "standalone"` in `next.config.mjs`), run as a non-root user.
Two API URLs are involved because SSR and the browser sit on different
networks:

* `NEXT_PUBLIC_API_URL` (build arg, baked into the browser bundle) —
  `https://localhost:9168`, the host-published backend port.
* `API_URL` (runtime env, server-only) —
  `https://certify-service-backend:9168`, the compose-internal hostname used
  by the SSR prefetch.

The backend's mkcert certificate only names `localhost`/`127.0.0.1`, so the
frontend container sets `NODE_TLS_REJECT_UNAUTHORIZED=0` for its server-side
fetches (dev-only; a production deployment would use a certificate covering
the internal hostname instead).

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
