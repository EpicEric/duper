import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { parse } from "@duper-js/node";
import { globalConst } from "vite-plugin-global-const";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";
import type { DefaultTheme, UserConfig } from "vitepress";

const DUPER_GRAMMAR = parse(
  readFileSync(resolve(__dirname, "../../duper.tmLanguage.duper"), "utf-8"),
  true,
);

const EBNF_GRAMMAR = parse(
  readFileSync(resolve(__dirname, "../../ebnf.tmLanguage.duper"), "utf-8"),
  true,
);

export default async () => {
  return {
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
      languages: [DUPER_GRAMMAR, EBNF_GRAMMAR],
    },

    themeConfig: {
      logo: "/logos/duper.svg",

      nav: [
        { text: "Home", link: "/" },
        { text: "Quick start", link: "/quick-start" },
        { text: "Blog", link: "/blog/" },
        { text: "Specification", link: "/spec" },
      ],

      sidebar: {
        "/blog": {
          base: "/blog/",
          items: [
            {
              text: "Posts",
              link: "/",
              items: [
                {
                  text: "Duper's new superpowers!",
                  link: "/duper-s-new-superpowers",
                },
              ],
            },
          ],
        },
        "/": {
          base: "/",
          items: [
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
                { text: "JavaScript", link: "/guide/javascript" },
                { text: ".NET (alpha)", link: "/guide/dotnet" },
                { text: "Python", link: "/guide/python" },
                { text: "Rust", link: "/guide/rust" },
              ],
            },
            {
              text: "Editor support",
              items: [{ text: "Visual Studio Code", link: "/tools/vs-code" }],
            },
            {
              text: "Tools",
              items: [
                { text: "duperq", link: "/tools/duperq" },
                { text: "duperfmt", link: "/tools/duperfmt" },
              ],
            },
            { text: "Specification", link: "/spec" },
          ],
        },
      },

      outline: {
        level: [2, 3],
      },

      socialLinks: [
        { icon: "github", link: "https://github.com/EpicEric/duper" },
      ],

      editLink: {
        pattern:
          "https://github.com/EpicEric/duper/edit/main/duper_website/docs/:path",
      },

      footer: {
        message: `Released under the <a href="https://github.com/EpicEric/duper/blob/main/LICENSE">MIT License</a>.`,
        copyright: "Copyright © 2025 Eric Rodrigues Pires",
      },

      search: {
        provider: "local",
      },
    },

    vite: {
      plugins: [
        wasm(),
        topLevelAwait(),
        globalConst({
          DUPER_GRAMMAR,
        }),
      ],
      ssr: {
        noExternal: ["monaco-editor"],
      },
      resolve: {
        alias: {
          "@": resolve(__dirname, "../.."),
        },
      },
    },
  } as UserConfig<DefaultTheme.Config>;
};
