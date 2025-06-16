import { MutationCache, QueryClient, QueryKey } from "@tanstack/react-query";
import { showErrorNotification } from "./util/showErrorNotification";

declare module "@tanstack/react-query" {
  interface Register {
    defaultError: {
      status: number;
    };

    mutationMeta: {
      invalidate?: Parameters<QueryClient["invalidateQueries"]>[0];
      invalidateQueryKey?: QueryKey;
      successMessage?: string; // unused
      error?: {
        title: string;
        message: string;
      };
    };
  }
}

export const queryClient = new QueryClient({
  mutationCache: new MutationCache({
    onError: (error, _variables, _context, mutation) => {
      if (error?.status === 401) {
        // perform logout
        window.location.href = "/signin"; // or navigate with router
      }

      if (mutation.meta?.error) {
        const { title, message } = mutation.meta.error;

        showErrorNotification(title, message);
      } else if (typeof error === "string") {
        showErrorNotification("Error", error);
      }
    },
    onSettled: (_data, _error, _variables, _context, mutation) => {
      if (mutation.meta?.invalidateQueryKey) {
        queryClient.invalidateQueries({
          queryKey: mutation.meta.invalidateQueryKey
        });
      }
      if (mutation.meta?.invalidate) {
        queryClient.invalidateQueries(mutation.meta.invalidate);
      }
    }
  })
});
