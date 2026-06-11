# certify-service

A TLS certificate inventory application. A Rust HTTPS service parses
PEM-encoded X.509 certificates and stores their metadata in PostgreSQL; a
Next.js dashboard lists the stored certificates, flags upcoming expirations,
and shows per-certificate details.

```
            HTTPS :9168                SQLx / TCP 5432
  browser ──────────────▶  backend  ──────────────────▶  PostgreSQL
    │                  (Axum + parser)
    │  HTTP :3000
    └──────────────▶  frontend (Next.js dashboard)
```

The repository contains two services plus a Compose file to run everything:

| Directory                  | What it is                                          |
| -------------------------- | --------------------------------------------------- |
| `certify-service-backend/` | Rust API — parse, store, and serve certificates     |
| `certify-service-frontend/`| Next.js inventory dashboard consuming that API      |
| `compose.yml`              | PostgreSQL + backend + frontend, one command        |

Each service has its own README with API reference, architecture notes, and
standalone (non-Docker) instructions.

## Prerequisites

* **Docker** with the Compose plugin.
* **mkcert** — the backend serves HTTPS and loads TLS certificate files from
  `certify-service-backend/certs/`.

### Generate local TLS certificates

```bash
# macOS
brew install mkcert
# Optional: only required if you use Firefox
brew install nss
# Windows (Using Chocolatey)
choco install mkcert
# Optional: only required if you use Firefox
choco install nss

# Install mkcert's local Certificate Authority
mkcert -install
# From the backend directory i.e `certify-service-backend/`
mkdir -p certs
mkcert \
  -cert-file certs/cert.pem \
  -key-file certs/key.pem \
  localhost 127.0.0.1 ::1
```

## Quick Start

1. Create a `.env` file in the repository root:

```bash
POSTGRES_PASSWORD=your_password
DATABASE_URL=postgres://pos:your_password@localhost:5432/pos?sslmode=disable
SQLX_OFFLINE=true
```

2. Start PostgreSQL, the backend, and the frontend with Docker Compose (run in
   the repository root):

```bash
docker compose up --build
```

3. Verify the backend is running:

```bash
curl -ik https://localhost:9168/health-check
```

Expected response:

```text
200 OK
```

4. Open the dashboard:

```text
http://localhost:3000
```

It redirects to `/inventory`, server-rendered with the certificates already in
the database. To add a certificate:

```bash
curl -sk -X POST https://localhost:9168/certificate \
  -H 'content-type: application/json' \
  -d '{ "pem_data": "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----\n" }'
```

## Service endpoints

| Service  | URL                       | Notes                                  |
| -------- | ------------------------- | -------------------------------------- |
| Frontend | `http://localhost:3000`   | Certificate inventory dashboard        |
| Backend  | `https://localhost:9168`  | JSON API (self-signed cert locally)    |
| Database | `localhost:5432`          | PostgreSQL 16 (`pos` / `pos`)          |

More detail:

* Backend API reference and design notes — `certify-service-backend/README.md`
* Frontend architecture and local development — `certify-service-frontend/README.md`
