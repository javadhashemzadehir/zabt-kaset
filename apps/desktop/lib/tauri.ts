import { invoke } from "@tauri-apps/api/core";
import type {
  ChannelSyncPlan,
  ChannelSyncRequest,
  DownloadJob,
  DownloadOptions,
  OrganizerContext,
  ProxyProfile,
  ProxyTestResult,
  QueueAddRequest,
  QueueAddResponse,
  ToolsSnapshot,
} from "@/types/api";

const isTauri =
  typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

async function call<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauri) {
    throw new Error(`Tauri API unavailable for command: ${cmd}`);
  }
  return invoke<T>(cmd, args);
}

export const api = {
  queueList: (limit?: number) => call<DownloadJob[]>("queue_list", { limit }),
  queueAdd: (request: QueueAddRequest) =>
    call<QueueAddResponse>("queue_add", { request }),
  queuePause: (id: string) => call<void>("queue_pause", { id }),
  queueResume: (id: string) => call<void>("queue_resume", { id }),
  queueCancel: (id: string) => call<void>("queue_cancel", { id }),
  queueRetry: (id: string) => call<void>("queue_retry", { id }),
  queuePreviewArgs: (options: DownloadOptions) =>
    call<string[]>("queue_preview_args", { options }),
  historySearch: (query: string, limit?: number) =>
    call<DownloadJob[]>("history_search", { query, limit }),
  settingsGet: (key: string) => call<unknown>("settings_get", { key }),
  settingsSet: (key: string, value: unknown) =>
    call<void>("settings_set", { key, value }),
  toolsStatus: (ytDlpPath?: string, ffmpegPath?: string) =>
    call<ToolsSnapshot>("tools_status", {
      yt_dlp_path: ytDlpPath,
      ffmpeg_path: ffmpegPath,
    }),
  toolsUpdateYtDlp: (path?: string) =>
    call<string>("tools_update_yt_dlp", { path }),
  proxyTest: (profile: ProxyProfile) =>
    call<ProxyTestResult>("proxy_test", { profile }),
  channelPlanSync: (request: ChannelSyncRequest) =>
    call<ChannelSyncPlan>("channel_plan_sync", { request }),
  organizerExpand: (template: string, context: OrganizerContext) =>
    call<string>("organizer_expand", { template, context }),
};

export { isTauri };
