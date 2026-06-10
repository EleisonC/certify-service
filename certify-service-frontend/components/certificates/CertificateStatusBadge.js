import { Shield, AlertTriangle, ShieldX } from "lucide-react";
import { Badge } from "@/components/ui/Badge";
import { CERT_STATUS } from "@/lib/utils/certificates";

/** Single source of truth for how each certificate status is rendered. */
const STATUS_CONFIG = {
  [CERT_STATUS.VALID]: { label: "Valid", variant: "success", icon: Shield },
  [CERT_STATUS.EXPIRING]: {
    label: "Expiring soon",
    variant: "warning",
    icon: AlertTriangle,
  },
  [CERT_STATUS.EXPIRED]: { label: "Expired", variant: "danger", icon: ShieldX },
};

export function CertificateStatusBadge({ status }) {
  const config = STATUS_CONFIG[status] ?? STATUS_CONFIG[CERT_STATUS.VALID];
  const Icon = config.icon;
  return (
    <Badge variant={config.variant}>
      <Icon className="h-3 w-3" aria-hidden="true" />
      {config.label}
    </Badge>
  );
}
