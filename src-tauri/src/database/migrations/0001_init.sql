-- Zabt Kaset initial schema

CREATE TABLE IF NOT EXISTS schema_migrations (
    version TEXT PRIMARY KEY NOT NULL,
    applied_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS presets (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('builtin', 'custom')),
    video_format TEXT,
    audio_format TEXT,
    quality TEXT NOT NULL DEFAULT 'best',
    embed_subtitles INTEGER NOT NULL DEFAULT 0,
    subtitle_langs TEXT NOT NULL DEFAULT '["en","fa"]',
    write_thumbnail INTEGER NOT NULL DEFAULT 1,
    write_description INTEGER NOT NULL DEFAULT 1,
    write_metadata INTEGER NOT NULL DEFAULT 1,
    organizer_template TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS proxy_profiles (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    protocol TEXT NOT NULL CHECK (protocol IN ('http', 'https', 'socks4', 'socks5', 'socks5h')),
    host TEXT NOT NULL,
    port INTEGER NOT NULL CHECK (port > 0 AND port <= 65535),
    username TEXT,
    password_ref TEXT,
    enabled INTEGER NOT NULL DEFAULT 1,
    last_latency_ms INTEGER,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS cookie_profiles (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    browser TEXT NOT NULL CHECK (browser IN ('chrome', 'edge', 'firefox', 'brave')),
    vault_key TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS channels (
    id TEXT PRIMARY KEY NOT NULL,
    url TEXT NOT NULL UNIQUE,
    handle TEXT,
    title TEXT,
    incremental INTEGER NOT NULL DEFAULT 1,
    last_video_id TEXT,
    last_synced_at TEXT,
    include_shorts INTEGER NOT NULL DEFAULT 1,
    include_lives INTEGER NOT NULL DEFAULT 0,
    keyword_filter TEXT,
    date_after TEXT,
    date_before TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS playlists (
    id TEXT PRIMARY KEY NOT NULL,
    url TEXT NOT NULL UNIQUE,
    title TEXT,
    channel_id TEXT REFERENCES channels(id) ON DELETE SET NULL,
    last_synced_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS download_jobs (
    id TEXT PRIMARY KEY NOT NULL,
    url TEXT NOT NULL,
    title TEXT,
    source_type TEXT NOT NULL CHECK (source_type IN ('video', 'playlist', 'channel', 'shorts', 'batch', 'community')),
    state TEXT NOT NULL CHECK (state IN ('waiting', 'analyzing', 'downloading', 'processing', 'completed', 'failed', 'cancelled')),
    priority INTEGER NOT NULL DEFAULT 0,
    preset_id TEXT REFERENCES presets(id) ON DELETE SET NULL,
    proxy_id TEXT REFERENCES proxy_profiles(id) ON DELETE SET NULL,
    cookie_profile_id TEXT REFERENCES cookie_profiles(id) ON DELETE SET NULL,
    output_dir TEXT,
    progress REAL NOT NULL DEFAULT 0,
    speed_bps INTEGER,
    eta_seconds INTEGER,
    error_message TEXT,
    video_id TEXT,
    retry_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    started_at TEXT,
    finished_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_download_jobs_state ON download_jobs(state);
CREATE INDEX IF NOT EXISTS idx_download_jobs_created ON download_jobs(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_download_jobs_video_id ON download_jobs(video_id);

CREATE TABLE IF NOT EXISTS media_files (
    id TEXT PRIMARY KEY NOT NULL,
    job_id TEXT NOT NULL REFERENCES download_jobs(id) ON DELETE CASCADE,
    path TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('video', 'audio', 'subtitle', 'thumbnail', 'description', 'metadata', 'image')),
    format TEXT,
    resolution TEXT,
    size_bytes INTEGER,
    sha256 TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_media_files_job ON media_files(job_id);
CREATE INDEX IF NOT EXISTS idx_media_files_sha ON media_files(sha256);

CREATE TABLE IF NOT EXISTS duplicates (
    id TEXT PRIMARY KEY NOT NULL,
    video_id TEXT,
    sha256 TEXT,
    filename TEXT,
    media_file_id TEXT REFERENCES media_files(id) ON DELETE SET NULL,
    matched_by TEXT NOT NULL CHECK (matched_by IN ('sha256', 'video_id', 'metadata', 'filename')),
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_duplicates_video ON duplicates(video_id);
CREATE INDEX IF NOT EXISTS idx_duplicates_sha ON duplicates(sha256);

-- Seed built-in presets
INSERT OR IGNORE INTO presets (id, name, kind, video_format, audio_format, quality, embed_subtitles, subtitle_langs, organizer_template)
VALUES
    ('preset-best', 'Best Quality', 'builtin', 'mp4', NULL, 'bestvideo*+bestaudio/best', 1, '["en","fa","ar","de","fr"]', 'YouTube/{channel}/Videos/{title}.{ext}'),
    ('preset-balanced', 'Balanced', 'builtin', 'mp4', NULL, 'bv*[height<=1080]+ba/b', 1, '["en","fa"]', 'YouTube/{channel}/Videos/{title}.{ext}'),
    ('preset-mobile', 'Mobile', 'builtin', 'mp4', NULL, 'bv*[height<=720]+ba/b', 0, '["en"]', 'YouTube/{channel}/Mobile/{title}.{ext}'),
    ('preset-audio', 'Audio Only', 'builtin', NULL, 'mp3', 'bestaudio/best', 0, '[]', 'YouTube/{channel}/Audio/{title}.{ext}'),
    ('preset-archive', 'Archive', 'builtin', 'mkv', NULL, 'bestvideo*+bestaudio/best', 1, '["en","fa","ar","de","fr"]', 'Archive/{year}/{channel}/{title}.{ext}');

INSERT OR IGNORE INTO settings (key, value) VALUES
    ('download_root', '""'),
    ('theme', '"system"'),
    ('locale', '"en"'),
    ('clipboard_monitor', 'false'),
    ('max_concurrent_downloads', '2'),
    ('default_preset_id', '"preset-balanced"');
