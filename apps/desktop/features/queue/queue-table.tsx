"use client";

import { Pause, Play, RotateCcw, X } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Progress } from "@/components/ui/progress";
import { EmptyState } from "@/components/shared/empty-state";
import { LoadingBlock } from "@/components/shared/loading-block";
import { useQueue, useQueueActions } from "@/hooks/use-queue";
import { isTauri } from "@/lib/tauri";
import { useUiStore } from "@/stores/ui-store";
import en from "@/locales/en/common.json";
import fa from "@/locales/fa/common.json";

function stateVariant(state: string) {
  switch (state) {
    case "completed":
      return "success" as const;
    case "failed":
      return "destructive" as const;
    case "downloading":
    case "processing":
    case "analyzing":
      return "default" as const;
    case "cancelled":
      return "secondary" as const;
    default:
      return "outline" as const;
  }
}

export function QueueTable() {
  const locale = useUiStore((s) => s.locale);
  const t = locale === "fa" ? fa : en;
  const { data, isLoading, error } = useQueue();
  const actions = useQueueActions();

  if (!isTauri) {
    return <EmptyState title={t.queue.empty} description={t.common.browserOnly} />;
  }
  if (isLoading) return <LoadingBlock label={t.common.loading} />;
  if (error) {
    return <EmptyState title={t.common.error} description={String(error)} />;
  }
  if (!data?.length) {
    return <EmptyState title={t.queue.empty} description={t.downloads.empty} />;
  }

  return (
    <div className="panel divide-y divide-border/70 overflow-hidden">
      {data.map((job) => (
        <div key={job.id} className="space-y-3 p-4">
          <div className="flex flex-wrap items-start justify-between gap-3">
            <div className="min-w-0 space-y-1">
              <p className="truncate font-medium">{job.title || job.url}</p>
              <p className="truncate text-xs text-muted-foreground">{job.url}</p>
            </div>
            <Badge variant={stateVariant(job.state)}>
              {(t.states as Record<string, string>)[job.state] ?? job.state}
            </Badge>
          </div>
          <Progress value={Math.min(100, Math.max(0, job.progress))} />
          <div className="flex flex-wrap gap-2">
            <Button size="sm" variant="outline" onClick={() => actions.pause.mutate(job.id)}>
              <Pause className="h-3.5 w-3.5" />
              {t.queue.pause}
            </Button>
            <Button size="sm" variant="outline" onClick={() => actions.resume.mutate(job.id)}>
              <Play className="h-3.5 w-3.5" />
              {t.queue.resume}
            </Button>
            <Button size="sm" variant="outline" onClick={() => actions.retry.mutate(job.id)}>
              <RotateCcw className="h-3.5 w-3.5" />
              {t.queue.retry}
            </Button>
            <Button size="sm" variant="destructive" onClick={() => actions.cancel.mutate(job.id)}>
              <X className="h-3.5 w-3.5" />
              {t.queue.cancel}
            </Button>
          </div>
        </div>
      ))}
    </div>
  );
}
