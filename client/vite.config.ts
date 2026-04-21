import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  build: {
    target: 'esnext', // Required for top-level await (Wasm)
  },
  optimizeDeps: {
    exclude: ['engine'], // Don't pre-bundle the Wasm module
  },
  server: {
    headers: {
      // Required for SharedArrayBuffer / Atomics (used by some wgpu builds)
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
    },
  },
})
