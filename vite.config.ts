import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte()],
  clearScreen: false,
  server: {
    host: process.env.TAURI_DEV_HOST,
    port: 1420,
    strictPort: true,
  },
});
