"use client";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { RefreshCw } from "lucide-react";
import { Button } from "@/components/ui/button";
import { EmptyState } from "@/components/shared/empty-state";
import { LoadingBlock } from "@/components/shared/loading-block";
import { api, isTauri } from "@/lib/tauri";
import { useUiStore } from "@/stores/ui-store";
import en from "@/locales/en/common.json";
import fa from "@/locales/fa/common.json";

export function ToolsPanel() {
  const locale = useUiStore((s) => s.locale);
  const t = locale === "fa" ? fa : en;
  const qc = useQueryClient();
  const { data, isLoading, error, refetch, isFetching } = useQuery({
    queryKey: ["tools"],
    queryFn: () => api.toolsStatus(),
    enabled: isTauri,
  });
  const update = useMutation({
    mutationFn: () => api.toolsUpdateYtDlp(),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["tools"] }),
  });

  if (!isTauri) {
    return <EmptyState title={t.tools.title} description={t.common.browserOnly} />;
  }
  if (isLoading) return <LoadingBlock label={t.common.loading} />;
  if (error || !data) {
    return <EmptyState title={t.common.error} description={String(error)} />;
  }

  const rows = [
    { label: t.tools.ytDlp, tool: data.ytDlp },
    { label: t.tools.ffmpeg, tool: data.ffmpeg },
  ];

  return (
    <div className="space-y-4">
      <div className="flex flex-wrap gap-2">
        <Button variant="outline" onClick={() => refetch()} disabled={isFetching}>
          <RefreshCw className="h-4 w-4" />
          {t.tools.refresh}
        </Button>
        <Button onClick={() => update.mutate()} disabled={update.isPending || !data.ytDlp.found}>
          {t.tools.update}
        </Button>
      </div>
      <div className="grid gap-4 md:grid-cols-2">
        {rows.map((row) => (
          <div key={row.label} className="panel p-4">
            <p className="font-medium">{row.label}</p>
            <p className="mt-2 text-sm text-muted-foreground">
              {row.tool.found
                ? `${row.tool.version ?? "unknown"} · ${row.tool.path}`
                : t.tools.notFound}
            </p>
          </div>
        ))}
      </div>
      {update.data ? (
        <pre className="panel overflow-x-auto p-3 text-xs text-muted-foreground">{update.data}</pre>
      ) : null}
    </div>
  );
}