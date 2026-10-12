import { ChannelsPanel } from "@/features/channels/channels-panel";

export default function ChannelsPage() {
  return (
    <div className="mx-auto w-full max-w-5xl space-y-6">
      <header>
        <h1 className="font-display text-3xl font-bold tracking-tight">Channels</h1>
        <p className="text-sm text-muted-foreground">
          Incremental channel sync with filters for Shorts and keywords.
        </p>
      </header>
      <ChannelsPanel />
    </div>
  );
}
