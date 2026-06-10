import { AlertTriangle, RotateCw } from "lucide-react";

/** Inline failure panel with an optional retry action. */
export function ErrorState({
  title = "Something went wrong",
  message,
  onRetry,
}) {
  return (
    <div
      role="alert"
      className="flex flex-col items-center justify-center gap-3 rounded-lg border border-red-200 bg-red-50/50 px-6 py-16 text-center"
    >
      <AlertTriangle className="h-6 w-6 text-red-600" aria-hidden="true" />
      <div>
        <p className="text-sm font-semibold text-slate-900">{title}</p>
        {message ? <p className="mt-1 text-sm text-slate-600">{message}</p> : null}
      </div>
      {onRetry ? (
        <button
          type="button"
          onClick={onRetry}
          className="mt-2 inline-flex items-center gap-2 rounded-md bg-blue-600 px-3.5 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-700 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600"
        >
          <RotateCw className="h-4 w-4" aria-hidden="true" />
          Try again
        </button>
      ) : null}
    </div>
  );
}
