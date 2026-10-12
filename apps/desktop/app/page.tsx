import { DownloadForm } from "@/features/downloads/download-form";
import { QueueTable } from "@/features/queue/queue-table";

export default function HomePage() {
  return (
    <div className="mx-auto flex w-full max-w-5xl flex-col gap-6">
      <header className="space-y-1">
        <h1 className="font-display text-3xl font-bold tracking-tight">Downloads</h1>
        <p className="text-sm text-muted-foreground">
          Queue videos, playlists, channels, and Shorts with smart presets.
        </p>
      </header>
      <DownloadForm />
      <section className="space-y-3">
        <h2 className="font-display text-xl font-semibold">Active queue</h2>
        <QueueTable />
      </section>
    </div>
  );
}
