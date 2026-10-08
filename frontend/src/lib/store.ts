import { signal } from "@preact/signals";
import {
  getSettings,
  scanLive,
  scanStatic,
  type Settings,
} from "./api";

export type Route = "static" | "live" | "settings";

export const route = signal<Route>("static");
export const setRoute = (r: Route): void => {
  route.value = r;
};

export const settings = signal<Settings | null>(null);
export const settingsError = signal<string | null>(null);

export const scanning = signal(false);
export const scanCounts = signal<{
  staticCount: number;
  liveCount: number;
} | null>(null);
export const scanError = signal<string | null>(null);

export const showAbout = signal(false);

export const errorMessage = (err: unknown): string =>
  err instanceof Error ? err.message : String(err);

export async function loadSettings(): Promise<void> {
  try {
    settings.value = await getSettings();
    settingsError.value = null;
  } catch (err) {
    settingsError.value = errorMessage(err);
  }
}

export async function runScan(): Promise<void> {
  scanning.value = true;
  scanError.value = null;
  try {
    const statics = await scanStatic();
    const lives = await scanLive();
    scanCounts.value = {
      staticCount: statics.length,
      liveCount: lives.length,
    };
  } catch (err) {
    scanError.value = errorMessage(err);
  } finally {
    scanning.value = false;
  }
}
