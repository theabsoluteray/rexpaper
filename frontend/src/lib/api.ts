import { invoke } from "@tauri-apps/api/core";

// Typed mirror of the Rust command layer (src-tauri/src/commands.rs).
// Every entry point in and out of the backend crosses this module only;
// command names and argument names must stay in sync with the Rust side.

/** Mirrors `rexpaper_core::Settings` (serde field names are snake_case). */
export interface Settings {
  wallpaper_dir: string | null;
  live_wallpaper_dir: string | null;
  active_live_wallpaper: string | null;
  active_static_wallpaper: string | null;
  wallpaper_mode: string;
  run_on_startup: boolean;
  pause_on_fullscreen: boolean;
  mute_live_wallpapers: boolean;
}

/** Mirrors `rexpaper_core::WallpaperItem`. */
export interface WallpaperItem {
  path: string;
  category: string;
}

/** Mirrors `rexpaper_core::LiveWallpaperItem`. */
export interface LiveWallpaperItem {
  path: string;
  category: string;
  duration: number | null;
}

export const getSettings = (): Promise<Settings> => invoke("get_settings");

export const setWallpaperDir = (path: string): Promise<Settings> =>
  invoke("set_wallpaper_dir", { path });

export const setLiveWallpaperDir = (path: string): Promise<Settings> =>
  invoke("set_live_wallpaper_dir", { path });

export const setRunOnStartup = (enabled: boolean): Promise<Settings> =>
  invoke("set_run_on_startup", { enabled });

export const setPauseOnFullscreen = (enabled: boolean): Promise<Settings> =>
  invoke("set_pause_on_fullscreen", { enabled });

export const setMuteLiveWallpapers = (enabled: boolean): Promise<Settings> =>
  invoke("set_mute_live_wallpapers", { enabled });

export const scanStatic = (): Promise<WallpaperItem[]> => invoke("scan_static");

export const scanLive = (): Promise<LiveWallpaperItem[]> => invoke("scan_live");

export const applyStatic = (path: string): Promise<Settings> =>
  invoke("apply_static", { path });
