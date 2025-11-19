import esbuild from "esbuild";

/** @type {import("esbuild").BuildOptions} */
const buildOptions = {
  entryPoints: ["src/index.ts"],
  outdir: "dist",
  assetNames: "[name]",
  platform: "neutral",
  external: ["./wasm-bindgen/index.js"],
  mainFields: ["main"],
  format: "esm",
  bundle: true,
  plugins: [],
  minify: true,
};

await esbuild.build(buildOptions);
