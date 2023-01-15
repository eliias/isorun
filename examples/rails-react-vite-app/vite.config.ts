import { defineConfig } from 'vite';
import RubyPlugin from 'vite-plugin-ruby';
import ReactPlugin from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [
    ReactPlugin(),
    RubyPlugin(),
  ],
  ssr: {
    format: "esm",
    noExternal: true,
    target: "webworker"
  }
})
