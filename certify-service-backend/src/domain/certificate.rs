// Parse don't validate
// parse a pem string and return a certificate struct or an error
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use x509_parser::extensions::{GeneralName, ParsedExtension};
use x509_parser::parse_x509_certificate;
use x509_parser::pem::parse_x509_pem;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Certificate {
    pub id: Uuid,
    pub serial_number: Option<String>,
    pub subject: String,
    pub issuer: String,
    #[serde(rename = "expiration")]
    pub not_after: DateTime<Utc>,
    pub san_entries: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewCertificate {
    pub serial_number: Option<String>,
    pub subject: String,
    pub issuer: String,
    #[serde(rename = "expiration")]
    pub not_after: DateTime<Utc>,
    #[serde(default)]
    pub san_entries: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ParseCertError {
    #[error("Invalid PEM format structure")]
    InvalidPem,
    #[error("PEM does not contain a valid X.509 certificate")]
    InvalidCertificate,
}

impl NewCertificate {
    pub fn parse_pem(pem_str: &str) -> Result<NewCertificate, ParseCertError> {
        let (_, pem) =
            parse_x509_pem(pem_str.as_bytes()).map_err(|_| ParseCertError::InvalidPem)?;
        let (_, cert) = parse_x509_certificate(&pem.contents)
            .map_err(|_| ParseCertError::InvalidCertificate)?;

        let mut san_entries: Vec<String> = Vec::new();
        for ext in cert.extensions() {
            if let ParsedExtension::SubjectAlternativeName(san) = ext.parsed_extension() {
                for name in &san.general_names {
                    if let GeneralName::DNSName(domain) = name {
                        san_entries.push(domain.to_string());
                    }
                }
            }
        }

        Ok(NewCertificate {
            serial_number: Some(cert.raw_serial_as_string()),
            subject: cert.subject().to_string(),
            issuer: cert.issuer().to_string(),
            not_after: DateTime::from_timestamp(cert.validity().not_after.timestamp(), 0)
                .expect("X.509 validity date is always a valid timestamp"),
            san_entries,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Self-signed cert: serial 11:46:2c:...:48:07, notAfter 2036-06-05T21:17:08Z,
    // SANs = DNS test.example.com, DNS www.example.com, IP, email, URI.
    const VALID_CERT_PEM: &str = r#"-----BEGIN CERTIFICATE-----
MIIDuzCCAqOgAwIBAgIUEUYshp4fKoXtb+SUxcC7nm2dSAcwDQYJKoZIhvcNAQEL
BQAwPjEZMBcGA1UEAwwQdGVzdC5leGFtcGxlLmNvbTEUMBIGA1UECgwLRXhhbXBs
ZSBPcmcxCzAJBgNVBAYTAlVTMB4XDTI2MDYwODIxMTcwOFoXDTM2MDYwNTIxMTcw
OFowPjEZMBcGA1UEAwwQdGVzdC5leGFtcGxlLmNvbTEUMBIGA1UECgwLRXhhbXBs
ZSBPcmcxCzAJBgNVBAYTAlVTMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKC
AQEAqKDmxDlyeKLjdc6OQ9jVeoSaJ2MYO171WL9yMT4ay6klC/sXrrAmoito+3/U
Gke7CR+zAD+Tww24GNKSLBwNdsYQLhrg8AL9MDNg11Xj06uxd3LRDMZPKcoQGiz2
ihxoxugqIWlDzLncVTcqmRTgCHgpSZ+vcT8CRVvAbHbBF1WehGHTMeY8+kluENOL
Aqx7SDUGbgtblnLyAtN+l6TTfbvyyqC8LAT5/dfo8eyW/DxKClaxbbQTYV8pbI7z
rjujV4CocCCZ+HKKzEOMYO0ILOx8+xZrRYnD8GFjdYuqVrOs8oJZl7qZJ/KMqAUy
5YbFawlBvo5hJpFmTt1ieMSnWQIDAQABo4GwMIGtMB0GA1UdDgQWBBSgNqzeuWfy
b6Ocy2GDFGY1sJdg5TAfBgNVHSMEGDAWgBSgNqzeuWfyb6Ocy2GDFGY1sJdg5TAP
BgNVHRMBAf8EBTADAQH/MFoGA1UdEQRTMFGCEHRlc3QuZXhhbXBsZS5jb22CD3d3
dy5leGFtcGxlLmNvbYcEwKgBCoERYWRtaW5AZXhhbXBsZS5jb22GE2h0dHBzOi8v
ZXhhbXBsZS5jb20wDQYJKoZIhvcNAQELBQADggEBAHmZEtPX7XphRmwvB49l8j4P
k7PImlatc+3RPEsV5ikpQb4/tMer/QKOptGjxMxnBGIcXRz0DWv3azHOzGxAO9Rb
MO57XPze3W//JxBZqkqkFKYjOW7vsq2R5UXxhrdNCVYjkhIFOydNjdX42qndjDOl
N6c0PpBLRVFFa97AeDdb/uOBFyeCiqMqeBeRSXRJ2WLCAAOqiRo72ygzD5f+/GmM
SYQh6k2cA1HzZx9mp7gGeRL9GeFyLHE55RXkSJuDsRObV2TpZ7kDWdgG910bXu1f
9ZjZrhDEhnoG2TYhkpFb/ZIYPx6kBlxUBsfJpq5YDzCi+06gvwBlj57bP1OxT6Q=
-----END CERTIFICATE-----
"#;

    // Valid PEM framing, but the base64 body is not a DER certificate.
    const NOT_A_CERT_PEM: &str = r#"-----BEGIN CERTIFICATE-----
dGhpcyBpcyB2YWxpZCBiYXNlNjQgYnV0IG5vdCBhIERFUiBjZXJ0aWZpY2F0ZQ==
-----END CERTIFICATE-----
"#;

    #[test]
    fn parses_valid_certificate_fields_exactly() {
        let cert = NewCertificate::parse_pem(VALID_CERT_PEM).expect("should parse");

        assert!(cert.subject.contains("test.example.com"));
        assert!(cert.issuer.contains("test.example.com")); // self-signed

        let serial = cert.serial_number.expect("serial present");
        assert_eq!(
            serial.replace(':', "").to_lowercase(),
            "11462c869e1f2a85ed6fe494c5c0bb9e6d9d4807"
        );

        assert_eq!(cert.not_after.to_rfc3339(), "2036-06-05T21:17:08+00:00");

        // Exact list pins DNS-only behavior: count, order, and exclusion of the
        // IP / email / URI SANs all matter.
        assert_eq!(cert.san_entries, ["test.example.com", "www.example.com"]);
    }

    #[test]
    fn invalid_pem() {
        let err = NewCertificate::parse_pem("clearly not a certificate").unwrap_err();
        assert!(matches!(err, ParseCertError::InvalidPem));
    }

    #[test]
    fn invalid_certificate() {
        // Passes the PEM stage, fails DER parsing — distinct from InvalidPem.
        let err = NewCertificate::parse_pem(NOT_A_CERT_PEM).unwrap_err();
        assert!(matches!(err, ParseCertError::InvalidCertificate));
    }
}
