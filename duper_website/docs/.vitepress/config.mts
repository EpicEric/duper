import { readFileSync } from "node:fs";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";
import { defineConfig } from "vitepress";

export default defineConfig({
  title: "Duper",
  description: "The format that's super!",
  head: [
    [
      "link",
      {
        rel: "icon",
        type: "image/png",
        href: "/logos/duper-100-100.png",
      },
    ],
  ],

  markdown: {
    languages: [JSON.parse(readFileSync("duper.tmLanguage.json", "utf-8"))],
  },

  themeConfig: {
    logo: "/logos/duper.svg",

    nav: [
      { text: "Home", link: "/" },
      { text: "Quick start", link: "/quick-start" },
      { text: "Specification", link: "/spec" },
      { text: "GitHub", link: "https://github.com/EpicEric/duper" },
    ],

    sidebar: [
      {
        text: "Getting started",
        items: [
          { text: "Quick start", link: "/quick-start" },
          {
            text: "An introduction to Duper",
            link: "/intro-to-duper",
          },
        ],
      },
      {
        text: "Language guides",
        items: [
          { text: "JavaScript", link: "/guide-javascript" },
          { text: "Python", link: "/guide-python" },
          { text: "Rust", link: "/guide-rust" },
        ],
      },
      {
        text: "Editor support",
        items: [{ text: "Visual Studio Code", link: "/vs-code" }],
      },
      { text: "Specification", link: "/spec" },
    ],

    outline: {
      level: [2, 3],
    },

    socialLinks: [{ icon: "github", link: "https://github.com/EpicEric/duper" }],

    editLink: {
      pattern: "https://github.com/EpicEric/duper.dev.br/edit/main/docs/:path",
    },

    footer: {
      message: `Released under the <a href="https://github.com/EpicEric/duper/blob/main/LICENSE">MIT License</a>.`,
      copyright: "Copyright © 2025 Eric Rodrigues Pires",
    },
  },

  vite: {
    plugins: [
      wasm(),
      topLevelAwait(),
    ],
    ssr: {
      noExternal: ['monaco-editor'],
    },
  },
});
