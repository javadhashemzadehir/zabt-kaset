"use client";

import { useTheme } from "next-themes";
import { Button } from "@/components/ui/button";
import { useUiStore } from "@/stores/ui-store";
import en from "@/locales/en/common.json";
import fa from "@/locales/fa/common.json";

export function SettingsPanel() {
  const locale = useUiStore((s) => s.locale);
  const setLocale = useUiStore((s) => s.setLocale);
  const clipboardMonitor = useUiStore((s) => s.clipboardMonitor);
  const setClipboardMonitor = useUiStore((s) => s.setClipboardMonitor);
  const { theme, setTheme } = useTheme();
  const t = locale === "fa" ? fa : en;

  return (
    <div className="panel max-w-xl space-y-6 p-5">
      <div className="space-y-2">
        <p className="text-sm text-muted-foreground">{t.settings.theme}</p>
        <div className="flex flex-wrap gap-2">
          {(["light", "dark", "system"] as const).map((value) => (
            <Button
              key={value}
              size="sm"
              variant={theme === value ? "default" : "outline"}
              onClick={() => setTheme(value)}
            >
              {value}
            </Button>
          ))}
        </div>
      </div>

      <div className="space-y-2">
        <p className="text-sm text-muted-foreground">{t.settings.language}</p>
        <div className="flex flex-wrap gap-2">
          <Button
            size="sm"
            variant={locale === "en" ? "default" : "outline"}
            onClick={() => setLocale("en")}
          >
            English
          </Button>
          <Button
            size="sm"
            variant={locale === "fa" ? "default" : "outline"}
            onClick={() => setLocale("fa")}
          >
            فارسی
          </Button>
        </div>
      </div>

      <div className="flex items-center justify-between gap-4">
        <p className="text-sm">{t.settings.clipboard}</p>
        <Button
          size="sm"
          variant={clipboardMonitor ? "default" : "outline"}
          onClick={() => setClipboardMonitor(!clipboardMonitor)}
        >
          {clipboardMonitor ? "On" : "Off"}
        </Button>
      </div>
    </div>
  );
}
