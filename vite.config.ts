import { defineConfig } from "vite";
import preact from "@preact/preset-vite";
import tailwindcss from "@tailwindcss/vite";

// Sources live in frontend/; build output (frontend/dist) is what
// src-tauri/tauri.conf.json embeds as frontendDist.
export default defineConfig({
  root: "frontend",
  plugins: [preact(), tailwindcss()],
  // Keep cargo/vite output from wiping each other's terminal during
  // `tauri dev` (beforeDevCommand + cargo run share the console).
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
  },
});
