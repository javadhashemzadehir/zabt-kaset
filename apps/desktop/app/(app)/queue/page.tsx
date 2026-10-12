import { QueueTable } from "@/features/queue/queue-table";

export default function QueuePage() {
  return (
    <div className="mx-auto w-full max-w-5xl space-y-6">
      <header>
        <h1 className="font-display text-3xl font-bold tracking-tight">Queue</h1>
        <p className="text-sm text-muted-foreground">
          Pause, resume, cancel, and retry download jobs.
        </p>
      </header>
      <QueueTable />
    </div>
  );
}
