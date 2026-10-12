"use client";

import { useQuery } from "@tanstack/react-query";
import { useState } from "react";
import { EmptyState } from "@/components/shared/empty-state";
import { LoadingBlock } from "@/components/shared/loading-block";
import { Input } from "@/components/ui/input";
import { Badge } from "@/components/ui/badge";
import { api, isTauri } from "@/lib/tauri";
import { useUiStore } from "@/stores/ui-store";
import en from "@/locales/en/common.json";
import fa from "@/locales/fa/common.json";

export function HistoryPanel() {
  const locale = useUiStore((s) => s.locale);
  const t = locale === "fa" ? fa : en;
  const [query, setQuery] = useState("");
  const { data, isLoading, error } = useQuery({
    queryKey: ["history", query],
    queryFn: () => api.historySearch(query, 100),
    enabled: isTauri,
  });

  return (
    <div className="space-y-4">
      <Input
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        placeholder={t.history.search}
      />
      {!isTauri ? (
        <EmptyState title={t.history.empty} description={t.common.browserOnly} />
      ) : isLoading ? (
        <LoadingBlock label={t.common.loading} />
      ) : error ? (
        <EmptyState title={t.common.error} description={String(error)} />
      ) : !data?.length ? (
        <EmptyState title={t.history.empty} />
      ) : (
        <div className="panel divide-y divide-border/70">
          {data.map((item) => (
            <div key={item.id} className="flex items-center justify-between gap-3 p-4">
              <div className="min-w-0">
                <p className="truncate font-medium">{item.title || item.url}</p>
                <p className="truncate text-xs text-muted-foreground">{item.createdAt}</p>
              </div>
              <Badge variant="secondary">{item.state}</Badge>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
