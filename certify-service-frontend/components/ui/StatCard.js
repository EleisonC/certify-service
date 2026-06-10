import { Card } from "./Card";
import { cn } from "@/lib/utils/cn";

const TONES = {
  neutral: "bg-slate-100 text-slate-600",
  info: "bg-blue-50 text-blue-600",
  success: "bg-emerald-50 text-emerald-600",
  warning: "bg-amber-50 text-amber-600",
  danger: "bg-red-50 text-red-600",
};

export function StatCard({ icon: Icon, label, value, hint, tone = "neutral" }) {
  return (
    <Card className="flex items-start gap-4 p-5">
      <div
        className={cn(
          "flex h-10 w-10 shrink-0 items-center justify-center rounded-md",
          TONES[tone],
        )}
        aria-hidden="true"
      >
        <Icon className="h-5 w-5" />
      </div>
      <div className="min-w-0">
        <p className="text-sm font-medium text-slate-500">{label}</p>
        <p className="mt-1 text-2xl font-semibold tracking-tight text-slate-900">
          {value}
        </p>
        {hint ? <p className="mt-1 text-xs text-slate-500">{hint}</p> : null}
      </div>
    </Card>
  );
}
