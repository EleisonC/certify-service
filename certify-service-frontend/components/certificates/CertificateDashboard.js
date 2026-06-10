"use client";

import { useMemo } from "react";
import { Shield, ShieldCheck, AlertTriangle, ShieldX } from "lucide-react";
import { StatCard } from "@/components/ui/StatCard";
import {
  summarizeCertificates,
  EXPIRY_WARNING_DAYS,
} from "@/lib/utils/certificates";

/**
 * Top-of-page metrics: total inventory size plus a health breakdown computed
 * with Day.js (see summarizeCertificates).
 */
export default function CertificateDashboard({ certificates }) {
  const stats = useMemo(() => summarizeCertificates(certificates), [certificates]);

  return (
    <section aria-label="Certificate overview">
      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-4">
        <StatCard
          icon={Shield}
          tone="info"
          label="Total certificates"
          value={stats.total}
          hint="Tracked in inventory"
        />
        <StatCard
          icon={ShieldCheck}
          tone="success"
          label="Valid"
          value={stats.valid}
          hint={`Expiring beyond ${EXPIRY_WARNING_DAYS} days`}
        />
        <StatCard
          icon={AlertTriangle}
          tone="warning"
          label="Expiring soon"
          value={stats.expiring}
          hint={`Within the next ${EXPIRY_WARNING_DAYS} days`}
        />
        <StatCard
          icon={ShieldX}
          tone="danger"
          label="Expired"
          value={stats.expired}
          hint="Past their not-after date"
        />
      </div>
    </section>
  );
}
