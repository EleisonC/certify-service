import axios from "axios";

/**
 * Shared Axios instance for the certify-service API.
 *
 * Two base URLs are involved because the SSR prefetch and the browser reach
 * the backend over different networks:
 *  - API_URL (server-only, read at runtime): used for SSR. In Docker this is
 *    the compose-internal hostname, e.g. https://certify-service-backend:9168.
 *  - NEXT_PUBLIC_API_URL (inlined at build time): used by the browser, which
 *    reaches the backend through the published host port.
 */
const baseURL =
  (typeof window === "undefined" ? process.env.API_URL : undefined) ??
  process.env.NEXT_PUBLIC_API_URL ??
  "https://localhost:9168";

export const apiClient = axios.create({
  baseURL,
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
