import { preact } from "@preact/preset-vite";
import { defineConfig } from "vite";
import { ViteMinifyPlugin } from "vite-plugin-minify";

// https://vite.dev/config/
export default defineConfig(() => {
  return {
    plugins: [
      preact({ include: "**.tsx" }),
      ViteMinifyPlugin(),
    ],
    preview: { port: 8579 },
    server: {
      allowedHosts: [".localhost", ".lexi.fyi"],
      port: 8579,
    },
    css: { modules: { generateScopedName: "[local]_[hash:4]" } },
    build: {
      // my current thinking on this is: if an image is small enough to embed
      // in JS, it's better off in a spritesheet.
      assetsInlineLimit: 0,
      rolldownOptions: {
        output: {
          advancedChunks: {
            groups: [
              {
                name: "common",
                minShareCount: 2,
              },
            ],
          },
        },
      },
    },
  };
});
