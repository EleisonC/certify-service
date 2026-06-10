"use client";

import { useState } from "react";
import { ShieldOff, RotateCw } from "lucide-react";
import { useCertificates } from "@/hooks/useCertificates";
import { getApiErrorMessage } from "@/lib/api/client";
import { LoadingState } from "@/components/ui/LoadingState";
import { ErrorState } from "@/components/ui/ErrorState";
import { EmptyState } from "@/components/ui/EmptyState";
import { cn } from "@/lib/utils/cn";
import CertificateDashboard from "./CertificateDashboard";
import CertificateTable from "./CertificateTable";
import CertificateDetails from "./CertificateDetails";

export default function InventoryView() {
  const {
    data: certificates,
    isPending,
    isError,
    error,
    refetch,
    isFetching,
  } = useCertificates();
  const [selectedId, setSelectedId] = useState(null);

  // Toggle selection so clicking the active row deselects it.
  const handleSelect = (id) =>
    setSelectedId((current) => (current === id ? null : id));

  const selectedCertificate =
    certificates?.find((certificate) => certificate.id === selectedId) ?? null;

  return (
    <div className="space-y-6">
      <div className="flex flex-wrap items-start justify-between gap-4">
        <div>
          <h1 className="text-xl font-semibold tracking-tight text-slate-900">
            Certificate Inventory
          </h1>
          <p className="mt-1 text-sm text-slate-500">
            TLS certificates tracked by certify-service, with expiry monitoring.
          </p>
        </div>
        <button
          type="button"
          onClick={() => refetch()}
          disabled={isFetching}
          className="inline-flex items-center gap-2 rounded-md border border-slate-200 bg-white px-3.5 py-2 text-sm font-medium text-slate-700 transition-colors hover:bg-slate-50 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 disabled:opacity-60"
        >
          <RotateCw
            className={cn("h-4 w-4", isFetching && "animate-spin")}
            aria-hidden="true"
          />
          Refresh
        </button>
      </div>

      {isPending ? (
        <LoadingState label="Loading certificate inventory…" />
      ) : isError ? (
        <ErrorState
          title="Could not load certificates"
          message={getApiErrorMessage(error)}
          onRetry={refetch}
        />
      ) : (
        <>
          <CertificateDashboard certificates={certificates} />

          {certificates.length === 0 ? (
            <EmptyState
              icon={ShieldOff}
              title="No certificates yet"
              message="Add one with POST /certificates or POST /certificates/parse and it will appear here."
            />
          ) : (
            <div className="space-y-6">
              <CertificateTable
                certificates={certificates}
                selectedId={selectedId}
                onSelect={handleSelect}
              />
              <CertificateDetails
                certificate={selectedCertificate}
                onClose={() => setSelectedId(null)}
              />
            </div>
          )}
        </>
      )}
    </div>
  );
}
