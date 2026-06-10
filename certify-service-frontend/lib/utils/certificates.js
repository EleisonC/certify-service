import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";

dayjs.extend(relativeTime);

/** Certificates expiring within this many days are flagged as "expiring". */
export const EXPIRY_WARNING_DAYS = 30;

export const CERT_STATUS = {
  VALID: "valid",
  EXPIRING: "expiring",
  EXPIRED: "expired",
};

/**
 * Classify a certificate by its `expiration` timestamp relative to now.
 * `now` is injectable for testability.
 */
export function getCertificateStatus(certificate, now = dayjs()) {
  const expiration = dayjs(certificate.expiration);
  if (expiration.isBefore(now)) return CERT_STATUS.EXPIRED;
  if (expiration.diff(now, "day") < EXPIRY_WARNING_DAYS)
    return CERT_STATUS.EXPIRING;
  return CERT_STATUS.VALID;
}

/** Whole days until expiry — negative once the certificate has expired. */
export function daysUntilExpiry(certificate, now = dayjs()) {
  return dayjs(certificate.expiration).diff(now, "day");
}

/** Aggregate counts that power the dashboard stat cards. */
export function summarizeCertificates(certificates, now = dayjs()) {
  const summary = {
    total: certificates.length,
    valid: 0,
    expiring: 0,
    expired: 0,
  };
  for (const certificate of certificates) {
    summary[getCertificateStatus(certificate, now)] += 1;
  }
  return summary;
}

/**
 * Extract the Common Name from a distinguished name string such as
 * "CN=example.com, O=Example Inc, C=US". Falls back to the full DN.
 */
export function getCommonName(distinguishedName) {
  const match = /CN=([^,]+)/.exec(distinguishedName ?? "");
  return match ? match[1].trim() : (distinguishedName ?? "");
}

export function formatDate(value) {
  return value ? dayjs(value).format("MMM D, YYYY") : "—";
}

export function formatDateTime(value) {
  return value ? dayjs(value).format("MMM D, YYYY HH:mm") : "—";
}

/** Relative phrasing, e.g. "in 2 months" or "3 days ago". */
export function fromNow(value) {
  return value ? dayjs(value).fromNow() : "—";
}
