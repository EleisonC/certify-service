# certify-service-backend

A Rust HTTPS service that accepts PEM-encoded X.509 certificates, extracts and stores certificate metadata in PostgreSQL, and exposes APIs to retrieve individual certificates by ID or list all stored certificates.

The service uses TLS for encrypted communication and includes support for local development using a self-signed certificate.

Built with [Axum](https://github.com/tokio-rs/axum), [SQLx](https://github.com/launchbadge/sqlx)
(PostgreSQL, compile-time-checked queries), and
[x509-parser](https://github.com/rusticata/x509-parser).

## Prerequisites

The service runs over HTTPS and expects TLS certificate files to be present under `certify-service-backend/certs/`.

For local development, generate a trusted certificate using `mkcert`.

### Generate local TLS certificates

```bash
# macOS
brew install mkcert
# Optional: only required if you use Firefox
brew install nss
# Install mkcert's local Certificate Authority
mkcert -install
# From the backend directory i.e `certify-service-backend/`
mkdir -p certs
mkcert \
  -cert-file certs/cert.pem \
  -key-file certs/key.pem \
  localhost 127.0.0.1 ::1
```

This creates:

```text
certs/
├── cert.pem
└── key.pem
```

which are loaded by the application at startup.

### TLS Client Notes

If you generated the certificates with `mkcert`, browsers should trust the certificate automatically after running `mkcert -install`.

If your client does not trust the certificate:



* In Insomnia or Postman, disable certificate verification for local development requests.

* In a browser, you may need to accept the certificate warning before proceeding to `https://localhost:9168`.

**NB** This is only required for local development. In production, certificates should be issued by a trusted Certificate Authority.

## Quick Start

1. Create a `.env` file in the repository root:

```bash
POSTGRES_PASSWORD=your_password
DATABASE_URL=postgres://pos:your_password@localhost:5432/pos?sslmode=disable
SQLX_OFFLINE=true
```

2. Start the application and PostgreSQL with Docker Compose(run in the repository root):

```bash
docker compose up --build
```

3. Verify the service is running:

```bash
curl i- http://localhost:9168/health-check
```

Expected response:

```text
200 OK
```

The service will be available at `https://localhost:9168`.


## What it does

- Parses a PEM certificate into structured metadata (serial number, subject,
  issuer, expiration, and DNS Subject Alternative Names).
- Persists each certificate to a PostgreSQL `certificates` table.
- Returns the stored record, including a generated `id`, as JSON.
- Lets you fetch all stored certificates.
- Lets you fetch a stored certificate by its `id`.

## API

| Method | Path                | Description                                  | Success |
| ------ | ------------------- | -------------------------------------------- | ------- |
| GET    | `/health-check`     | Liveness check                               | `200`   |
| POST   | `/certificate`      | Parse a PEM certificate and store it         | `201`   |
| GET    | `/certificate/{id}` | Fetch a stored certificate by its UUID `id`  | `200`   |
| GET    | `/certificates`     | Fetch all stored certificates                | `200`   |

### `POST /certificate`

Request body:

```json
{ "pem_data": "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----\n" }
```

Response (`201 Created`):

```json
{
  "id": "f0e1d2c3-....",
  "serial_number": "11:46:2c:...:48:07",
  "subject": "CN=test.example.com, O=Example Org, C=US",
  "issuer": "CN=test.example.com, O=Example Org, C=US",
  "expiration": "2036-06-05T21:17:08+00:00",
  "san_entries": ["test.example.com", "www.example.com"],
  "created_at": "2026-06-09T18:30:00+00:00"
}
```

Notes on the response shape:

- `expiration` is the certificate's `notAfter` (serialized under the `expiration` key).
- `san_entries` contains only the DNS Subject Alternative Names; IP, email, and
  URI SAN entries are not included.

### `GET /certificate/{id}`

Returns `200` with the stored certificate, or `404` if no certificate has that id.

### Error responses

Errors are returned as `{ "error": "<message>" }` with these status codes:

- `404 Not Found` — no certificate with the requested id.
- `422 Unprocessable Entity` — the body isn't a valid PEM / X.509 certificate.
- `400 Bad Request` — validation error.
- `500 Internal Server Error` — database error.

### `GET /certificates`

Returns `200` with a list of all stored certificates.

## Tech stack

- **Rust** — primary implementation language.
- **Axum 0.8** + **Tokio** — asynchronous HTTP server and request handling.
- **SQLx** + **PostgreSQL** — database access with compile-time checked queries and migrations.
- **x509-parser 0.18** — parsing and extracting metadata from PEM-encoded X.509 certificates.
- **Serde / serde_json** — request and response serialization.
- **Chrono** — date and time handling.
- **UUID** — generation and handling of certificate identifiers.
- **thiserror** — ergonomic application error types.
- **dotenvy** — environment-based configuration loading.
- **Reqwest** (dev-dependency) — HTTP client used by integration tests.
- **tracing** — structured application logging for requests and certificate operations.
- **tracing-subscriber** — configurable log filtering and formatting via environment variables.
- **tracing-error** — improved error diagnostics with span-aware error reporting.
- **tower-http** — HTTP middleware such as CORS for API access control.
- **axum-server** + **rustls** — HTTPS server implementation using Rust-native TLS.

## Configuration

Configuration is read from the environment, loaded from a single `.env` file in
the project root (via `dotenvy` for the app, and used by Docker Compose for
variable substitution):

```bash
DATABASE_URL=postgres://pos:[your_password]@localhost:5432/pos?sslmode=disable
POSTGRES_PASSWORD=[your_password]
SQLX_OFFLINE=true
```

- `DATABASE_URL` — PostgreSQL connection string used when running the binary
  directly (`cargo run`) and by the integration tests. It uses `localhost`.
- `POSTGRES_PASSWORD` — used by Docker Compose to substitute `${POSTGRES_PASSWORD}`
  into both the `db` service and the service's `DATABASE_URL`.
- `SQLX_OFFLINE` — set to `true` to build against the committed `.sqlx` query cache
  instead of a live database.

Note: under Docker Compose the service's `DATABASE_URL` comes from
`docker-compose.yml` (host `db`), which overrides the `localhost` value above —
that `localhost` one is for running the binary on your own machine.

The service binds to `0.0.0.0:9168`.

## Database

The schema is managed with SQLx migrations under `migrations/`. The
`certificates` table:

```sql
CREATE TABLE IF NOT EXISTS certificates (
    id            UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    serial_number TEXT,
    subject       TEXT        NOT NULL,
    issuer        TEXT        NOT NULL,
    not_after     TIMESTAMPTZ NOT NULL,
    san_entries   TEXT[]      NOT NULL DEFAULT '{}',
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

Migrations are also applied automatically at startup (`sqlx::migrate!()` in
`main`), so a fresh database is usable on first run.

## Running locally

You need a running PostgreSQL whose database, user, and password match your
`DATABASE_URL`. For example, with Docker:

```bash
docker run --name certify-postgres \
  -e POSTGRES_USER=pos \
  -e POSTGRES_PASSWORD=[your_password] \
  -e POSTGRES_DB=pos \
  -p 5432:5432 \
  -d postgres:16-alpine
```

Then run the service:

```bash
cargo run
```

It will apply migrations and start listening on `0.0.0.0:9168`. Quick check:

```bash
curl -i https://localhost:9168/health-check
```

## Tests

```bash
cargo test
```

There are two test layers:

- Unit tests for the certificate parser, in `src/domain/certificate.rs`.
- HTTP integration tests under `tests/api/`. Each test spins up the application
  on an ephemeral port and provisions its own freshly-migrated PostgreSQL
  database (named with a random UUID), then drops it on clean-up — so tests are
  isolated and can run in parallel. The integration tests connect to PostgreSQL
  at `postgres://pos:[your_password]@localhost:5432`, so that server must be running.

## SQLx offline cache

This project uses SQLx's compile-time query checking. The query metadata is
committed under `.sqlx/`, and with `SQLX_OFFLINE=true` the project builds without
a live database connection. Regenerate the cache after changing any query with:

```bash
cargo sqlx prepare
```

## Docker

The easiest way to run the whole stack (service + PostgreSQL) is Docker Compose.
The `docker-compose.yml` lives at the repository root (its build context is
`./certify-service-backend`), so run this from that root:

```bash
docker compose up --build
```

This builds the image, starts PostgreSQL, waits for it to become healthy, then
starts the service on `http://localhost:9168`. Inside the Compose network the
service reaches the database at host `db` (the service name), so the
`DATABASE_URL` in `docker-compose.yml` uses `db:5432`, not `localhost`.

Compose reads `POSTGRES_PASSWORD` from the project-root `.env` (see
[Configuration](#configuration)) for variable substitution, so make sure that
file is present.

The service's `Dockerfile` is a multi-stage build that uses `cargo-chef` to cache
dependencies, compiles the release binary with `SQLX_OFFLINE=true`, and copies it
into a slim runtime image — so the build needs no database connection.

## Project layout

```
certify-service-
backend/
  src/
    main.rs                       binary entrypoint: pool, migrations, run
    lib.rs                        Application builder, router, pool helper
    app_state.rs                  shared AppState (certificate store)
    domain/
      certificate.rs              Certificate / NewCertificate + PEM parser (+ unit tests)
      error.rs                    AppError and its HTTP response mapping
    routes/
      health_check.rs             GET /health-check
      create_certificate.rs       POST /certificate
      fetch_certificate.rs        GET /certificate/{id}
      fetch_all_certificates.rs   GET /certificates
    services/
      certificate_store.rs        SQLx queries (insert / get by id / get by serial)
    utils/
      constants.rs                DATABASE_URL loading
  migrations/                     SQL migrations
  tests/api/                      HTTP integration tests + TestApp helper
  .sqlx/                          committed SQLx offline query cache
  Dockerfile, build.rs, Cargo.toml
```

## Design Notes

- Only DNS SAN entries are stored.
- When running tests, Certificates are not deduplicated. Uncomment out the duplicate check in `create_certificate.rs` to enable it.
- The PEM payload itself is not persisted.
- TLS self-signed certificates are used for HTTPS.
- Authentication was intentionally omitted because it was outside the scope of the exercise.
