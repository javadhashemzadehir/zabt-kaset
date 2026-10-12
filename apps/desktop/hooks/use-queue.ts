"use client";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { api, isTauri } from "@/lib/tauri";
import type { QueueAddRequest } from "@/types/api";

export function useQueue(limit = 200) {
  return useQuery({
    queryKey: ["queue", limit],
    queryFn: () => api.queueList(limit),
    enabled: isTauri,
    refetchInterval: isTauri ? 2000 : false,
  });
}

export function useAddToQueue() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (request: QueueAddRequest) => api.queueAdd(request),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["queue"] }),
  });
}

export function useQueueActions() {
  const qc = useQueryClient();
  const invalidate = () => qc.invalidateQueries({ queryKey: ["queue"] });
  return {
    pause: useMutation({
      mutationFn: (id: string) => api.queuePause(id),
      onSuccess: invalidate,
    }),
    resume: useMutation({
      mutationFn: (id: string) => api.queueResume(id),
      onSuccess: invalidate,
    }),
    cancel: useMutation({
      mutationFn: (id: string) => api.queueCancel(id),
      onSuccess: invalidate,
    }),
    retry: useMutation({
      mutationFn: (id: string) => api.queueRetry(id),
      onSuccess: invalidate,
    }),
  };
}
