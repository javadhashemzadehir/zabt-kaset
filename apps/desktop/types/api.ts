export type JobState =
  | "waiting"
  | "analyzing"
  | "downloading"
  | "processing"
  | "completed"
  | "failed"
  | "cancelled";

export type SourceType =
  | "video"
  | "playlist"
  | "channel"
  | "shorts"
  | "batch"
  | "community";

export interface DownloadJob {
  id: string;
  url: string;
  title?: string | null;
  sourceType: string;
  state: JobState | string;
  priority: number;
  presetId?: string | null;
  progress: number;
  errorMessage?: string | null;
  videoId?: string | null;
  createdAt: string;
}

export interface QueueAddRequest {
  urls: string[];
  sourceType?: SourceType;
  priority?: number;
  presetId?: string;
}

export interface QueueAddResponse {
  jobIds: string[];
}

export interface DownloadOptions {
  url: string;
  outputTemplate: string;
  format?: string;
  audioOnly: boolean;
  audioFormat?: string;
  subtitleLangs: string[];
  embedSubtitles: boolean;
  writeThumbnail: boolean;
  writeDescription: boolean;
  writeInfoJson: boolean;
  proxy?: string;
  cookiesFromBrowser?: string;
  ytDlpPath: string;
}

export interface ToolStatus {
  name: string;
  found: boolean;
  path?: string | null;
  version?: string | null;
}

export interface ToolsSnapshot {
  ytDlp: ToolStatus;
  ffmpeg: ToolStatus;
}

export interface ProxyProfile {
  id: string;
  name: string;
  protocol: "http" | "https" | "socks4" | "socks5" | "socks5h";
  host: string;
  port: number;
  username?: string;
}

export interface ProxyTestResult {
  ok: boolean;
  latencyMs?: number | null;
  message: string;
}

export interface ChannelSyncRequest {
  url: string;
  includeShorts: boolean;
  includeLives: boolean;
  keywordFilter?: string;
  dateAfter?: string;
  dateBefore?: string;
  sinceVideoId?: string;
  limit?: number;
}

export interface ChannelSyncPlan {
  normalizedUrl: string;
  ytDlpArgs: string[];
}

export interface OrganizerContext {
  channel?: string;
  playlist?: string;
  title?: string;
  date?: string;
  year?: string;
  resolution?: string;
  format?: string;
  ext?: string;
}
