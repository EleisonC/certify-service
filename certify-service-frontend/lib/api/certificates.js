import { apiClient } from "./client";

/**
 * React Query cache keys for the certificates domain. Centralized here (not in
 * the hook) so server components can prefetch with the exact same keys.
 */
export const certificateKeys = {
  all: ["certificates"],
  detail: (id) => ["certificates", id],
};

/**
 * Fetch the full certificate inventory.
 *
 * ASSUMPTION: certify-service-backend currently exposes POST /certificate and
 * GET /certificate/{id} but no list endpoint. We assume `GET /certificates`
 * (plural, the usual REST collection form) returns a JSON array of records in
 * the same shape as `GET /certificate/{id}`:
 *
 *   [{ id, serial_number, subject, issuer, expiration, san_entries,
 *      created_at }, ...]
 *
 * Note: the backend serializes its `not_after` column as `expiration`.
 * If the list endpoint ships under a different path (e.g. the singular
 * `GET /certificate`) only this function needs to change.
 */
export async function fetchCertificates() {
  const { data } = await apiClient.get("/certificates");
  return data;
}

/** Fetch a single certificate by UUID — `GET /certificate/{id}`. */
export async function fetchCertificate(id) {
  const { data } = await apiClient.get(`/certificate/${id}`);
  return data;
}
