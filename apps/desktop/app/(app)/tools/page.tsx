import { ToolsPanel } from "@/features/tools/tools-panel";

export default function ToolsPage() {
  return (
    <div className="mx-auto w-full max-w-5xl space-y-6">
      <header>
        <h1 className="font-display text-3xl font-bold tracking-tight">Tools</h1>
        <p className="text-sm text-muted-foreground">Detect and update yt-dlp and FFmpeg.</p>
      </header>
      <ToolsPanel />
    </div>
  );
}