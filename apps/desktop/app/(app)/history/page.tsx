import { HistoryPanel } from "@/features/history/history-panel";

export default function HistoryPage() {
  return (
    <div className="mx-auto w-full max-w-5xl space-y-6">
      <header>
        <h1 className="font-display text-3xl font-bold tracking-tight">History</h1>
        <p className="text-sm text-muted-foreground">Search completed and failed downloads.</p>
      </header>
      <HistoryPanel />
    </div>
  );
}
