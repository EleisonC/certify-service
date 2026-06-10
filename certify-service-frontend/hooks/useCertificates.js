"use client";

import { useQuery } from "@tanstack/react-query";
import { certificateKeys, fetchCertificates } from "@/lib/api/certificates";

export function useCertificates() {
  return useQuery({
    queryKey: certificateKeys.all,
    queryFn: fetchCertificates,
    staleTime: 30_000,
  });
}
