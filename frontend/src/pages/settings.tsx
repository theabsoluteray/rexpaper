import {
  runScan,
  scanCounts,
  scanError,
  scanning,
  settings,
  settingsError,
} from "../lib/store";

function Row({ label, value }: { label: string; value: string }) {
  return (
    <div class="flex items-baseline justify-between gap-4 border-b border-border py-2 last:border-b-0">
      <dt class="shrink-0 text-sm text-secondary">{label}</dt>
      <dd class="text-right text-sm break-all">{value}</dd>
    </div>
  );
}

export function SettingsPage() {
  const s = settings.value;
  const counts = scanCounts.value;

  return (
    <div class="mx-auto w-full max-w-3xl space-y-6 p-6">
      <section class="rounded-lg border border-border bg-card p-5">
        <h2 class="mb-4 text-sm font-medium text-secondary">Settings</h2>
        {settingsError.value !== null && (
          <p class="text-sm text-red-400">{settingsError.value}</p>
        )}
        {settingsError.value === null && s === null && (
          <p class="text-sm text-muted">Loading&hellip;</p>
        )}
        {settingsError.value === null && s !== null && (
          <dl>
            <Row label="Wallpaper folder" value={s.wallpaper_dir ?? "Not set"} />
            <Row
              label="Live wallpaper folder"
              value={s.live_wallpaper_dir ?? "Not set"}
            />
            <Row label="Mode" value={s.wallpaper_mode || "\u2014"} />
            <Row
              label="Active static"
              value={s.active_static_wallpaper ?? "\u2014"}
            />
            <Row label="Active live" value={s.active_live_wallpaper ?? "\u2014"} />
            <Row label="Run on startup" value={s.run_on_startup ? "On" : "Off"} />
            <Row
              label="Pause on fullscreen"
              value={s.pause_on_fullscreen ? "On" : "Off"}
            />
            <Row
              label="Mute live wallpapers"
              value={s.mute_live_wallpapers ? "On" : "Off"}
            />
          </dl>
        )}
      </section>

      <section class="rounded-lg border border-border bg-card p-5">
        <h2 class="mb-4 text-sm font-medium text-secondary">Libraries</h2>
        <div class="flex items-center gap-4">
          <button
            type="button"
            onClick={() => void runScan()}
            disabled={scanning.value}
            class="rounded-md bg-accent px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-accent/80 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent disabled:opacity-50"
          >
            {scanning.value ? "Scanning\u2026" : "Scan libraries"}
          </button>
          {counts !== null && (
            <span class="text-sm text-muted">
              {counts.staticCount} static &middot; {counts.liveCount} live
            </span>
          )}
        </div>
        {scanError.value !== null && (
          <p class="mt-3 text-sm text-red-400">{scanError.value}</p>
        )}
      </section>
    </div>
  );
}
