import axios from "axios";

/**
 * Shared Axios instance for the certify-service API.
 *
 * The base URL comes from NEXT_PUBLIC_API_URL so the same instance works in
 * both server (SSR prefetch) and client (React Query refetch) environments.
 */
export const apiClient = axios.create({
  baseURL: process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:9168",
  timeout: 10_000,
  headers: {
    "Content-Type": "application/json",
  },
});

/**
 * Normalize Axios failures into a single human-readable message so UI error
 * states never have to dig through response shapes.
 */
export function getApiErrorMessage(error) {
  if (axios.isAxiosError(error)) {
    if (error.response) {
      const detail =
        typeof error.response.data === "string"
          ? error.response.data
          : error.response.data?.message;
      return detail || `Request failed with status ${error.response.status}`;
    }
    if (error.request) {
      return "The certificate service is unreachable. Check that the backend is running.";
    }
  }
  return error?.message ?? "An unexpected error occurred.";
}
