import { Loader2 } from "lucide-react";

/** Centered spinner with a label, announced politely to screen readers. */
export function LoadingState({ label = "Loading…" }) {
  return (
    <div
      role="status"
      aria-live="polite"
      className="flex flex-col items-center justify-center gap-3 rounded-lg border border-slate-200 bg-white py-16"
    >
      <Loader2 className="h-6 w-6 animate-spin text-blue-600" aria-hidden="true" />
      <p className="text-sm text-slate-500">{label}</p>
    </div>
  );
}
