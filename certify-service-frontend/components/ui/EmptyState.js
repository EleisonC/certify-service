/** Neutral placeholder for views that have no data yet. */
export function EmptyState({ icon: Icon, title, message, children }) {
  return (
    <div className="flex flex-col items-center justify-center gap-3 rounded-lg border border-dashed border-slate-300 bg-white px-6 py-16 text-center">
      {Icon ? (
        <div className="flex h-10 w-10 items-center justify-center rounded-md bg-slate-100">
          <Icon className="h-5 w-5 text-slate-500" aria-hidden="true" />
        </div>
      ) : null}
      <div>
        <p className="text-sm font-semibold text-slate-900">{title}</p>
        {message ? <p className="mt-1 text-sm text-slate-500">{message}</p> : null}
      </div>
      {children}
    </div>
  );
}
