"use client";

import { motion } from "framer-motion";
import { Link2, Plus } from "lucide-react";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { useAddToQueue } from "@/hooks/use-queue";
import { isTauri } from "@/lib/tauri";
import { validateUrlList } from "@/lib/url";
import { useQueueStore } from "@/stores/queue-store";
import { useUiStore } from "@/stores/ui-store";
import en from "@/locales/en/common.json";
import fa from "@/locales/fa/common.json";
import type { SourceType } from "@/types/api";

const presets = [
  { id: "preset-best", label: "Best Quality" },
  { id: "preset-balanced", label: "Balanced" },
  { id: "preset-mobile", label: "Mobile" },
  { id: "preset-audio", label: "Audio Only" },
  { id: "preset-archive", label: "Archive" },
];

const sources: SourceType[] = ["video", "playlist", "channel", "shorts", "batch", "community"];

export function DownloadForm() {
  const locale = useUiStore((s) => s.locale);
  const t = locale === "fa" ? fa : en;
  const draftUrls = useQueueStore((s) => s.draftUrls);
  const setDraftUrls = useQueueStore((s) => s.setDraftUrls);
  const presetId = useQueueStore((s) => s.presetId);
  const setPresetId = useQueueStore((s) => s.setPresetId);
  const sourceType = useQueueStore((s) => s.sourceType);
  const setSourceType = useQueueStore((s) => s.setSourceType);
  const add = useAddToQueue();
  const [error, setError] = useState<string | null>(null);
  const [dragging, setDragging] = useState(false);

  async function onSubmit(e: React.FormEvent) {
    e.preventDefault();
    setError(null);
    const { urls, errors } = validateUrlList(draftUrls);
    if (!urls.length) {
      setError(errors.length ? `Invalid URLs: ${errors.join(", ")}` : "Enter at least one URL");
      return;
    }
    if (!isTauri) {
      setError(t.common.browserOnly);
      return;
    }
    try {
      await add.mutateAsync({
        urls,
        presetId,
        sourceType: sourceType as SourceType,
      });
      setDraftUrls("");
    } catch (err) {
      setError(err instanceof Error ? err.message : t.common.error);
    }
  }

  return (
    <motion.form
      onSubmit={onSubmit}
      initial={{ opacity: 0, y: 8 }}
      animate={{ opacity: 1, y: 0 }}
      className="panel space-y-4 p-5"
      onDragOver={(e) => {
        e.preventDefault();
        setDragging(true);
      }}
      onDragLeave={() => setDragging(false)}
      onDrop={(e) => {
        e.preventDefault();
        setDragging(false);
        const text = e.dataTransfer.getData("text");
        if (text) setDraftUrls(draftUrls ? `${draftUrls}\n${text}` : text);
      }}
    >
      <div className="flex items-center gap-2">
        <Link2 className="h-4 w-4 text-primary" />
        <h2 className="font-display text-lg font-semibold">{t.downloads.title}</h2>
      </div>

      <Textarea
        value={draftUrls}
        onChange={(e) => setDraftUrls(e.target.value)}
        placeholder={t.downloads.placeholder}
        className={dragging ? "border-primary ring-2 ring-primary/30" : undefined}
      />

      <div className="grid gap-3 md:grid-cols-2">
        <label className="space-y-1.5 text-sm">
          <span className="text-muted-foreground">{t.downloads.preset}</span>
          <select
            className="h-10 w-full rounded-md border border-input bg-background px-3"
            value={presetId}
            onChange={(e) => setPresetId(e.target.value)}
          >
            {presets.map((p) => (
              <option key={p.id} value={p.id}>
                {p.label}
              </option>
            ))}
          </select>
        </label>
        <label className="space-y-1.5 text-sm">
          <span className="text-muted-foreground">{t.downloads.sourceType}</span>
          <select
            className="h-10 w-full rounded-md border border-input bg-background px-3"
            value={sourceType}
            onChange={(e) => setSourceType(e.target.value as SourceType)}
          >
            {sources.map((s) => (
              <option key={s} value={s}>
                {s}
              </option>
            ))}
          </select>
        </label>
      </div>

      {error ? <p className="text-sm text-destructive">{error}</p> : null}

      <Button type="submit" disabled={add.isPending} className="w-full sm:w-auto">
        <Plus className="h-4 w-4" />
        {t.downloads.submit}
      </Button>
    </motion.form>
  );
}
