import { useEffect, useRef, useState } from "preact/hooks";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { route, setRoute, showAbout, type Route } from "../lib/store";

const win = () => getCurrentWindow();

type MenuEntry =
  | { kind: "item"; label: string; onClick: () => void; checked?: boolean }
  | { kind: "separator" };

const routeLabels: Record<Route, string> = {
  static: "Static Wallpapers",
  live: "Live Wallpapers",
  settings: "Settings",
};

const fileMenu: MenuEntry[] = [
  { kind: "item", label: "Exit", onClick: () => void win().close() },
];

const helpMenu: MenuEntry[] = [
  {
    kind: "item",
    label: "About RexPaper",
    onClick: () => {
      showAbout.value = true;
    },
  },
];

function MenuPanel({ entries }: { entries: MenuEntry[] }) {
  return (
    <div class="absolute top-full left-0 z-50 mt-px min-w-52 rounded-md border border-border bg-panel py-1 shadow-lg">
      {entries.map((entry, i) =>
        entry.kind === "separator" ? (
          // eslint-disable-next-line react/no-array-index-key
          <div key={i} class="my-1 border-t border-border" />
        ) : (
          <button
            key={entry.label}
            type="button"
            onClick={() => {
              entry.onClick();
            }}
            class="flex w-full items-center gap-2 px-3 py-1.5 text-left text-sm text-text transition-colors hover:bg-card"
          >
            <span class="w-3 text-center text-xs text-accent">
              {entry.checked ? "\u2713" : ""}
            </span>
            {entry.label}
          </button>
        ),
      )}
    </div>
  );
}

function Titlebar() {
  const [openMenu, setOpenMenu] = useState<string | null>(null);
  const [maximized, setMaximized] = useState(false);
  const barRef = useRef<HTMLDivElement>(null);

  // Track maximize state for the restore/maximize glyph.
  useEffect(() => {
    let active = true;
    let unlisten: (() => void) | undefined;
    void win()
      .isMaximized()
      .then((m) => {
        if (active) setMaximized(m);
      });
    void win()
      .onResized(async () => {
        if (active) setMaximized(await win().isMaximized());
      })
      .then((f) => {
        if (active) unlisten = f;
        else f();
      });
    return () => {
      active = false;
      unlisten?.();
    };
  }, []);

  // Close the open menu on outside click or Escape.
  useEffect(() => {
    if (openMenu === null) return;
    const onDown = (e: MouseEvent) => {
      if (!barRef.current?.contains(e.target as Node)) setOpenMenu(null);
    };
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") setOpenMenu(null);
    };
    document.addEventListener("mousedown", onDown);
    document.addEventListener("keydown", onKey);
    return () => {
      document.removeEventListener("mousedown", onDown);
      document.removeEventListener("keydown", onKey);
    };
  }, [openMenu]);

  const menuButton = (name: string, label: string, entries: MenuEntry[]) => (
    <button
      type="button"
      onClick={() => setOpenMenu(openMenu === name ? null : name)}
      onMouseEnter={() => {
        if (openMenu !== null && openMenu !== name) setOpenMenu(name);
      }}
      class={`relative h-full px-3 text-sm transition-colors ${
        openMenu === name ? "bg-card text-text" : "text-secondary hover:text-text"
      }`}
    >
      {label}
      {openMenu === name && <MenuPanel entries={entries} />}
    </button>
  );

  return (
    <header
      ref={barRef}
      data-tauri-drag-region
      class="relative z-40 flex h-10 shrink-0 select-none items-stretch border-b border-border bg-panel"
    >
      <div
        data-tauri-drag-region
        class="flex items-center pl-3 text-sm font-semibold text-text"
      >
        RexPaper
      </div>

      <nav class="ml-4 flex items-stretch">
        {menuButton("file", "File", fileMenu)}
        {menuButton(
          "view",
          "View",
          (["static", "live", "settings"] as const).map((r) => ({
            kind: "item" as const,
            label: routeLabels[r],
            checked: route.value === r,
            onClick: () => setRoute(r),
          })),
        )}
        {menuButton("help", "Help", helpMenu)}
      </nav>

      <div
        data-tauri-drag-region
        class="flex-1"
        onDblClick={() => void win().toggleMaximize()}
      />

      <div class="flex h-full items-stretch">
        <button
          type="button"
          aria-label="Minimize"
          onClick={() => void win().minimize()}
          class="flex w-11 items-center justify-center text-secondary transition-colors hover:bg-card hover:text-text"
        >
          <svg width="11" height="11" viewBox="0 0 11 11" aria-hidden="true">
            <path d="M1 5.5h9" stroke="currentColor" stroke-width="1" />
          </svg>
        </button>
        <button
          type="button"
          aria-label={maximized ? "Restore" : "Maximize"}
          onClick={() => void win().toggleMaximize()}
          class="flex w-11 items-center justify-center text-secondary transition-colors hover:bg-card hover:text-text"
        >
          {maximized ? (
            <svg width="11" height="11" viewBox="0 0 11 11" aria-hidden="true">
              <rect
                x="0.5"
                y="2.5"
                width="7"
                height="7"
                fill="none"
                stroke="currentColor"
              />
              <path
                d="M3.5 2.5v-2h7v7h-2"
                fill="none"
                stroke="currentColor"
              />
            </svg>
          ) : (
            <svg width="11" height="11" viewBox="0 0 11 11" aria-hidden="true">
              <rect
                x="0.5"
                y="0.5"
                width="10"
                height="10"
                fill="none"
                stroke="currentColor"
              />
            </svg>
          )}
        </button>
        <button
          type="button"
          aria-label="Close"
          onClick={() => void win().close()}
          class="flex w-11 items-center justify-center text-secondary transition-colors hover:bg-[#e81123] hover:text-white"
        >
          <svg width="11" height="11" viewBox="0 0 11 11" aria-hidden="true">
            <path
              d="M1 1l9 9M10 1l-9 9"
              stroke="currentColor"
              stroke-width="1"
            />
          </svg>
        </button>
      </div>
    </header>
  );
}

export default Titlebar;
