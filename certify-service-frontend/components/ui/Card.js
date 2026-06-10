import { cn } from "@/lib/utils/cn";

/** Base surface used across the app: white panel, subtle border, soft radius. */
export function Card({ className, ...props }) {
  return (
    <div
      className={cn("rounded-lg border border-slate-200 bg-white", className)}
      {...props}
    />
  );
}

/** Card header row with a bottom divider; pair with CardTitle. */
export function CardHeader({ className, ...props }) {
  return (
    <div
      className={cn(
        "flex items-center justify-between gap-3 border-b border-slate-200 px-5 py-4",
        className
      )}
      {...props}
    />
  );
}

export function CardTitle({ className, ...props }) {
  return (
    <h2
      className={cn("text-sm font-semibold text-slate-900", className)}
      {...props}
    />
  );
}

export function CardContent({ className, ...props }) {
  return <div className={cn("px-5 py-4", className)} {...props} />;
}
