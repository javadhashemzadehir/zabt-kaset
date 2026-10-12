"use client";

import { useMutation } from "@tanstack/react-query";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { EmptyState } from "@/components/shared/empty-state";
import { api, isTauri } from "@/lib/tauri";
import { useUiStore } from "@/stores/ui-store";
import en from "@/locales/en/common.json";
import fa from "@/locales/fa/common.json";

export function ChannelsPanel() {
  const locale = useUiStore((s) => s.locale);
  const t = locale === "fa" ? fa : en;
  const [url, setUrl] = useState("");
  const plan = useMutation({
    mutationFn: () =>
      api.channelPlanSync({
        url,
        includeShorts: true,
        includeLives: false,
        limit: 50,
      }),
  });

  return (
    <div className="space-y-4">
      <div className="panel flex flex-col gap-3 p-4 sm:flex-row">
        <Input
          value={url}
          onChange={(e) => setUrl(e.target.value)}
          placeholder="https://www.youtube.com/@channel"
        />
        <Button
          onClick={() => plan.mutate()}
          disabled={!url || plan.isPending || !isTauri}
        >
          {t.channels.sync}
        </Button>
      </div>
      {!isTauri ? (
        <EmptyState title={t.channels.empty} description={t.common.browserOnly} />
      ) : plan.data ? (
        <pre className="panel overflow-x-auto p-4 text-xs leading-relaxed text-muted-foreground">
          {JSON.stringify(plan.data, null, 2)}
        </pre>
      ) : (
        <EmptyState title={t.channels.empty} />
      )}
      {plan.error ? (
        <p className="text-sm text-destructive">{String(plan.error)}</p>
      ) : null}
    </div>
  );
}
