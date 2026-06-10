-- Add up migration script here
CREATE TABLE IF NOT EXISTS certificates (
    id            UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    serial_number TEXT,                              -- X.509 serial (hex), optional
    subject       TEXT        NOT NULL,              -- subject DN
    issuer        TEXT        NOT NULL,              -- issuer DN (the signing CA)
    not_after     TIMESTAMPTZ NOT NULL,             -- expiration (X.509 notAfter)
    san_entries   TEXT[]      NOT NULL DEFAULT '{}',
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);
