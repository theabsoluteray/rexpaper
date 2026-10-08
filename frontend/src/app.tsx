import { useEffect, useState } from "preact/hooks";
import { getVersion } from "@tauri-apps/api/app";
import Titlebar from "./components/titlebar";
import { Sidebar } from "./components/sidebar";
import { StaticPage } from "./pages/static";
import { LivePage } from "./pages/live";
import { SettingsPage } from "./pages/settings";
import { loadSettings, route, showAbout } from "./lib/store";

function AboutModal() {
  const [version, setVersion] = useState<string>("");

  useEffect(() => {
    void getVersion()
      .then(setVersion)
      .catch(() => setVersion(""));
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") showAbout.value = false;
    };
    document.addEventListener("keydown", onKey);
    return () => document.removeEventListener("keydown", onKey);
  }, []);

  return (
    <div
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/60"
      onClick={() => {
        showAbout.value = false;
      }}
    >
      <div
        role="dialog"
        aria-label="About RexPaper"
        class="w-80 rounded-lg border border-border bg-card p-5 shadow-lg"
        onClick={(e) => e.stopPropagation()}
      >
        <h2 class="text-sm font-semibold">RexPaper</h2>
        <p class="mt-1 text-sm text-secondary">
          {version !== "" ? `Version ${version}` : "Wallpaper manager for Windows"}
        </p>
        <p class="mt-3 text-sm text-muted">
          A fast, native wallpaper manager for Windows with hardware-accelerated
          live video wallpapers.
        </p>
        <p class="mt-3 text-xs text-muted">GPL-3.0 licensed.</p>
        <div class="mt-4 flex justify-end">
          <button
            type="button"
            onClick={() => {
              showAbout.value = false;
            }}
            class="rounded-md border border-border px-3 py-1.5 text-sm text-text transition-colors hover:bg-panel"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
}

export function App() {
  useEffect(() => {
    void loadSettings();
  }, []);

  return (
    <div class="flex h-screen flex-col overflow-hidden bg-bg text-text">
      <Titlebar />
      <div class="flex min-h-0 flex-1">
        <Sidebar />
        <main class="min-w-0 flex-1 overflow-y-auto bg-bg">
          {route.value === "static" && <StaticPage />}
          {route.value === "live" && <LivePage />}
          {route.value === "settings" && <SettingsPage />}
        </main>
      </div>
      {showAbout.value && <AboutModal />}
    </div>
  );
}
