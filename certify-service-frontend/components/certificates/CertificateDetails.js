"use client";

import { Lock, FileText, CalendarClock, X } from "lucide-react";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/Card";
import { Badge } from "@/components/ui/Badge";
import { cn } from "@/lib/utils/cn";
import {
  getCertificateStatus,
  getCommonName,
  daysUntilExpiry,
  formatDateTime,
  fromNow,
  CERT_STATUS,
} from "@/lib/utils/certificates";
import { CertificateStatusBadge } from "./CertificateStatusBadge";

/** Label/value pair used throughout the details panel. */
function DetailField({ label, children, mono = false }) {
  return (
    <div>
      <dt className="text-xs font-medium uppercase tracking-wide text-slate-500">
        {label}
      </dt>
      <dd
        className={cn(
          "mt-1 break-words text-sm text-slate-900",
          mono && "font-mono text-xs",
        )}
      >
        {children}
      </dd>
    </div>
  );
}

function ValiditySummary({ certificate }) {
  const status = getCertificateStatus(certificate);
  const days = daysUntilExpiry(certificate);

  const summary =
    status === CERT_STATUS.EXPIRED
      ? `Expired ${Math.abs(days)} day${Math.abs(days) === 1 ? "" : "s"} ago`
      : `${days} day${days === 1 ? "" : "s"} remaining`;

  return (
    <p
      className={cn(
        "text-sm font-medium",
        status === CERT_STATUS.VALID && "text-emerald-700",
        status === CERT_STATUS.EXPIRING && "text-amber-700",
        status === CERT_STATUS.EXPIRED && "text-red-700",
      )}
    >
      {summary}
    </p>
  );
}

export default function CertificateDetails({ certificate, onClose }) {
  if (!certificate) {
    return (
      <Card className="flex flex-col items-center justify-center gap-3 px-6 py-12 text-center">
        <div className="flex h-10 w-10 items-center justify-center rounded-md bg-slate-100">
          <FileText className="h-5 w-5 text-slate-500" aria-hidden="true" />
        </div>
        <div>
          <p className="text-sm font-semibold text-slate-900">
            No certificate selected
          </p>
          <p className="mt-1 text-sm text-slate-500">
            Select a row to inspect its details.
          </p>
        </div>
      </Card>
    );
  }

  return (
    <Card aria-label="Certificate details">
      <CardHeader>
        <div className="min-w-0">
          <CardTitle className="truncate text-base">
            {getCommonName(certificate.subject)}
          </CardTitle>
          <div className="mt-1.5">
            <CertificateStatusBadge
              status={getCertificateStatus(certificate)}
            />
          </div>
        </div>
        <button
          type="button"
          onClick={onClose}
          aria-label="Close details"
          className="rounded-md p-1.5 text-slate-400 transition-colors hover:bg-slate-100 hover:text-slate-600 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
        >
          <X className="h-4 w-4" aria-hidden="true" />
        </button>
      </CardHeader>

      <CardContent className="grid gap-x-10 gap-y-6 md:grid-cols-2">
        <div className="space-y-6">
          <section aria-label="Validity period">
            <h3 className="mb-3 flex items-center gap-2 text-xs font-semibold uppercase tracking-wide text-slate-500">
              <CalendarClock className="h-3.5 w-3.5" aria-hidden="true" />
              Validity
            </h3>
            <ValiditySummary certificate={certificate} />
            <dl className="mt-4">
              <DetailField label="Expiration">
                {formatDateTime(certificate.expiration)}{" "}
                <span className="text-slate-400">
                  ({fromNow(certificate.expiration)})
                </span>
              </DetailField>
            </dl>
          </section>

          <section
            aria-label="Record metadata"
            className="border-t border-slate-100 pt-4"
          >
            <dl className="space-y-4">
              <DetailField label="Record ID" mono>
                {certificate.id}
              </DetailField>
              <DetailField label="Added to inventory">
                {formatDateTime(certificate.created_at)}
              </DetailField>
            </dl>
          </section>
        </div>

        <section aria-label="Security metadata">
          <h3 className="mb-3 flex items-center gap-2 text-xs font-semibold uppercase tracking-wide text-slate-500">
            <Lock className="h-3.5 w-3.5" aria-hidden="true" />
            Security metadata
          </h3>
          <dl className="space-y-4">
            <DetailField label="Subject">{certificate.subject}</DetailField>
            <DetailField label="Issuer">{certificate.issuer}</DetailField>
            <DetailField label="Serial number" mono>
              {certificate.serial_number ?? "Not recorded"}
            </DetailField>
            <DetailField label="Subject alternative names">
              {certificate.san_entries.length > 0 ? (
                <span className="flex flex-wrap gap-1.5">
                  {certificate.san_entries.map((entry) => (
                    <Badge key={entry} variant="neutral" className="font-mono">
                      {entry}
                    </Badge>
                  ))}
                </span>
              ) : (
                <span className="text-slate-500">None</span>
              )}
            </DetailField>
          </dl>
        </section>
      </CardContent>
    </Card>
  );
}
