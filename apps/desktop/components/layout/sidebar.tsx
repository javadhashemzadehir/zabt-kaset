"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import {
  Clapperboard,
  Download,
  History,
  ListVideo,
  Radio,
  Settings,
  Wrench,
} from "lucide-react";
import { cn } from "@/lib/utils";
import { useUiStore } from "@/stores/ui-store";
import en from "@/locales/en/common.json";
import fa from "@/locales/fa/common.json";

const items = [
  { href: "/", icon: Download, key: "downloads" as const },
  { href: "/queue/", icon: ListVideo, key: "queue" as const },
  { href: "/history/", icon: History, key: "history" as const },
  { href: "/channels/", icon: Radio, key: "channels" as const },
  { href: "/tools/", icon: Wrench, key: "tools" as const },
  { href: "/settings/", icon: Settings, key: "settings" as const },
];

export function Sidebar() {
  const pathname = usePathname();
  const locale = useUiStore((s) => s.locale);
  const collapsed = useUiStore((s) => s.sidebarCollapsed);
  const messages = locale === "fa" ? fa : en;

  return (
    <aside
      className={cn(
        "flex h-full flex-col border-e border-border/70 bg-card/60 backdrop-blur-md transition-all",
        collapsed ? "w-[72px]" : "w-60",
      )}
    >
      <div className="flex items-center gap-3 px-4 py-5">
        <div className="flex h-10 w-10 items-center justify-center rounded-xl bg-primary text-primary-foreground shadow-soft">
          <Clapperboard className="h-5 w-5" />
        </div>
        {!collapsed ? (
          <div className="min-w-0">
            <p className="truncate font-display text-base font-bold tracking-tight">
              {messages.app.name}
            </p>
            <p className="truncate text-xs text-muted-foreground">{messages.app.tagline}</p>
          </div>
        ) : null}
      </div>
      <nav className="flex flex-1 flex-col gap-1 px-2 pb-4">
        {items.map((item) => {
          const active =
            item.href === "/"
              ? pathname === "/" || pathname === ""
              : pathname?.startsWith(item.href.replace(/\/$/, ""));
          const Icon = item.icon;
          return (
            <Link
              key={item.href}
              href={item.href}
              className={cn(
                "flex items-center gap-3 rounded-md px-3 py-2.5 text-sm transition-colors",
                active
                  ? "bg-primary/12 text-primary"
                  : "text-muted-foreground hover:bg-secondary hover:text-foreground",
              )}
            >
              <Icon className="h-4 w-4 shrink-0" />
              {!collapsed ? <span>{messages.nav[item.key]}</span> : null}
            </Link>
          );
        })}
      </nav>
    </aside>
  );
}
