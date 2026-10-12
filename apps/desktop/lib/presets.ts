export const BUILTIN_PRESETS = [
  {
    id: "preset-best",
    name: "Best Quality",
    videoFormats: ["mp4", "mkv", "webm"] as const,
    audioFormats: ["mp3", "m4a", "opus", "flac"] as const,
  },
  {
    id: "preset-balanced",
    name: "Balanced",
    videoFormats: ["mp4"] as const,
    audioFormats: ["m4a"] as const,
  },
  {
    id: "preset-mobile",
    name: "Mobile",
    videoFormats: ["mp4"] as const,
    audioFormats: ["m4a"] as const,
  },
  {
    id: "preset-audio",
    name: "Audio Only",
    videoFormats: [] as const,
    audioFormats: ["mp3", "m4a", "opus", "flac"] as const,
  },
  {
    id: "preset-archive",
    name: "Archive",
    videoFormats: ["mkv"] as const,
    audioFormats: ["flac"] as const,
  },
] as const;

export const SUBTITLE_LANGUAGES = ["fa", "en", "ar", "de", "fr"] as const;
export const SUBTITLE_FORMATS = ["srt", "vtt", "ass", "ssa", "ttml"] as const;
