"use client";

import { Sidebar } from "@/components/layout/sidebar";
import { useUiStore } from "@/stores/ui-store";

export function AppShell({ children }: { children: React.ReactNode }) {
  const locale = useUiStore((s) => s.locale);

  return (
    <div
      className="flex h-screen overflow-hidden"
      lang={locale}
      dir={locale === "fa" ? "rtl" : "ltr"}
    >
      <Sidebar />
      <main className="flex min-w-0 flex-1 flex-col overflow-hidden">
        <div className="flex-1 overflow-y-auto p-6 md:p-8">{children}</div>
      </main>
    </div>
  );
}
