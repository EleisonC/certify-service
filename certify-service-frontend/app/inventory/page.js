import {
  dehydrate,
  HydrationBoundary,
  QueryClient,
} from "@tanstack/react-query";
import InventoryView from "@/components/certificates/InventoryView";
import { certificateKeys, fetchCertificates } from "@/lib/api/certificates";

export const metadata = { title: "Inventory" };

// Inventory data changes at runtime, so always render on request rather than
// prerendering a stale snapshot at build time.
export const dynamic = "force-dynamic";

export default async function InventoryPage() {
  const queryClient = new QueryClient();

  await queryClient.prefetchQuery({
    queryKey: certificateKeys.all,
    queryFn: fetchCertificates,
  });

  return (
    <HydrationBoundary state={dehydrate(queryClient)}>
      <InventoryView />
    </HydrationBoundary>
  );
}
