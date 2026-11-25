import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// Config adaptée à Tauri : port fixe, pas d'écran effacé pour garder les erreurs Rust visibles.
export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: { ignored: ["**/src-tauri/**"] },
  },
});
