"use client";

import { Card, CardHeader, CardTitle } from "@/components/ui/Card";
import CertificateRow from "./CertificateRow";

export default function CertificateTable({
  certificates,
  selectedId,
  onSelect,
}) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Certificates</CardTitle>
        <span className="text-xs text-slate-500">
          {certificates.length} record{certificates.length === 1 ? "" : "s"}
        </span>
      </CardHeader>
      <div className="max-h-[19.5rem] overflow-auto">
        <table className="w-full text-left">
          <caption className="sr-only">
            TLS certificate inventory. Select a row to view details.
          </caption>
          <thead className="sticky top-0 z-10 bg-white shadow-[inset_0_-1px_0_#e2e8f0]">
            <tr className="text-xs font-medium uppercase tracking-wide text-slate-500">
              <th scope="col" className="px-5 py-3">
                Subject
              </th>
              <th scope="col" className="hidden px-5 py-3 md:table-cell">
                Issuer
              </th>
              <th scope="col" className="px-5 py-3">
                Status
              </th>
              <th scope="col" className="px-5 py-3">
                Expires
              </th>
              <th scope="col" className="hidden px-5 py-3 lg:table-cell">
                SANs
              </th>
            </tr>
          </thead>
          <tbody>
            {certificates.map((certificate) => (
              <CertificateRow
                key={certificate.id}
                certificate={certificate}
                isSelected={certificate.id === selectedId}
                onSelect={onSelect}
              />
            ))}
          </tbody>
        </table>
      </div>
    </Card>
  );
}
