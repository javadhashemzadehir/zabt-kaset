#!/usr/bin/env node
/**
 * Prints guidance for installing yt-dlp and ffmpeg.
 * Binary download automation can be added later per platform.
 */

const tips = {
  windows: [
    "winget install yt-dlp.yt-dlp",
    "winget install Gyan.FFmpeg",
  ],
  macos: ["brew install yt-dlp ffmpeg"],
  linux: ["sudo apt install yt-dlp ffmpeg", "# or use your distro packages"],
};

const platform = process.platform;
const key = platform === "win32" ? "windows" : platform === "darwin" ? "macos" : "linux";

console.log("Zabet Kaset — media tool setup");
console.log("Ensure yt-dlp and ffmpeg are on PATH, or set absolute paths in Settings → Tools.\n");
console.log(`Suggested commands (${key}):`);
for (const line of tips[key]) {
  console.log(`  ${line}`);
}
