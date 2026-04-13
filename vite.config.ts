/// <reference types="vitest/config" />
import { defineConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';

// https://vite.dev/config/
export default defineConfig({
  plugins: [sveltekit(), tailwindcss()],
  server: {
    port: 8000,
  },
  preview: {
    port: 8000,
  },
  test: {
    testTimeout: 30000,
  },
});
