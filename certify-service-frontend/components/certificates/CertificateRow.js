"use client";

import { cn } from "@/lib/utils/cn";
import {
  getCertificateStatus,
  getCommonName,
  formatDate,
  fromNow,
} from "@/lib/utils/certificates";
import { CertificateStatusBadge } from "./CertificateStatusBadge";

export default function CertificateRow({ certificate, isSelected, onSelect }) {
  const status = getCertificateStatus(certificate);

  return (
    <tr
      onClick={() => onSelect(certificate.id)}
      className={cn(
        "cursor-pointer border-b border-slate-100 transition-colors last:border-b-0",
        isSelected ? "bg-blue-50/60" : "hover:bg-slate-50",
      )}
    >
      <td className="px-5 py-3">
        <button
          type="button"
          onClick={(event) => {
            event.stopPropagation();
            onSelect(certificate.id);
          }}
          aria-pressed={isSelected}
          className="text-left font-medium text-slate-900 hover:text-blue-700 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
        >
          {getCommonName(certificate.subject)}
        </button>
        <p className="mt-0.5 max-w-xs truncate text-xs text-slate-500">
          {certificate.subject}
        </p>
      </td>
      <td className="hidden max-w-xs truncate px-5 py-3 text-sm text-slate-600 md:table-cell">
        {getCommonName(certificate.issuer)}
      </td>
      <td className="px-5 py-3">
        <CertificateStatusBadge status={status} />
      </td>
      <td className="whitespace-nowrap px-5 py-3 text-sm text-slate-600">
        <span className="block">{formatDate(certificate.expiration)}</span>
        <span className="text-xs text-slate-400">
          {fromNow(certificate.expiration)}
        </span>
      </td>
      <td className="hidden px-5 py-3 text-sm text-slate-600 lg:table-cell">
        {certificate.san_entries.length}
      </td>
    </tr>
  );
}
