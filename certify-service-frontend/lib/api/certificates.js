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
 * Fetch the full certificate inventory — `GET /certificates`.
 * Returns a JSON array of records in the same shape as `GET /certificate/{id}`:
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
