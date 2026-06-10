use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{Certificate, NewCertificate};

pub struct CertificateStore {
    pool: PgPool,
}

impl CertificateStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert_certificate(
        &mut self,
        cert: &NewCertificate,
    ) -> Result<Certificate, sqlx::Error> {
        sqlx::query_as!(
            Certificate,
            r#"
            INSERT INTO certificates
                (serial_number, subject, issuer, not_after, san_entries)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                id as "id!",
                serial_number,
                subject as "subject!",
                issuer as "issuer!",
                not_after as "not_after!",
                san_entries as "san_entries!",
                created_at as "created_at!"
            "#,
            cert.serial_number.clone(),
            cert.subject.clone(),
            cert.issuer.clone(),
            cert.not_after,
            &cert.san_entries.clone(),
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_certificate_by_serial_number(
        &self,
        serial_number: &str,
    ) -> Result<Option<Certificate>, sqlx::Error> {
        sqlx::query_as!(
            Certificate,
            r#"
            SELECT
                id as "id!",
                serial_number,
                subject as "subject!",
                issuer as "issuer!",
                not_after as "not_after!",
                san_entries as "san_entries!",
                created_at as "created_at!"
            FROM certificates
            WHERE serial_number = $1
            "#,
            serial_number,
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn get_certificate_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<Certificate>, sqlx::Error> {
        sqlx::query_as!(
            Certificate,
            r#"
            SELECT
                id as "id!",
                serial_number,
                subject as "subject!",
                issuer as "issuer!",
                not_after as "not_after!",
                san_entries as "san_entries!",
                created_at as "created_at!"
            FROM certificates
            WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
    }
}
