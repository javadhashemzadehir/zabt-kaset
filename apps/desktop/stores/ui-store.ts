import { create } from "zustand";
import { persist } from "zustand/middleware";
import type { AppLocale } from "@/lib/i18n";

type ThemePreference = "light" | "dark" | "system";

interface UiState {
  locale: AppLocale;
  theme: ThemePreference;
  clipboardMonitor: boolean;
  sidebarCollapsed: boolean;
  setLocale: (locale: AppLocale) => void;
  setTheme: (theme: ThemePreference) => void;
  setClipboardMonitor: (value: boolean) => void;
  toggleSidebar: () => void;
}

export const useUiStore = create<UiState>()(
  persist(
    (set) => ({
      locale: "en",
      theme: "system",
      clipboardMonitor: false,
      sidebarCollapsed: false,
      setLocale: (locale) => set({ locale }),
      setTheme: (theme) => set({ theme }),
      setClipboardMonitor: (clipboardMonitor) => set({ clipboardMonitor }),
      toggleSidebar: () => set((s) => ({ sidebarCollapsed: !s.sidebarCollapsed })),
    }),
    { name: "zabt-kaset-ui" },
  ),
);
