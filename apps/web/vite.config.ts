import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue()],
  server: {
    port: 5173,
    proxy: {
      // SI_API lets a second checkout (e.g. a worktree backend on :7778) run side by side.
      '/api': process.env.SI_API ?? 'http://127.0.0.1:7777',
    },
  },
  build: {
    outDir: 'dist',
    emptyOutDir: true,
  },
})
