import { SettingsPanel } from "@/features/settings/settings-panel";

export default function SettingsPage() {
  return (
    <div className="mx-auto w-full max-w-5xl space-y-6">
      <header>
        <h1 className="font-display text-3xl font-bold tracking-tight">Settings</h1>
        <p className="text-sm text-muted-foreground">Theme, language, and desktop preferences.</p>
      </header>
      <SettingsPanel />
    </div>
  );
}
