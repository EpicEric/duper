import esbuild from "esbuild";

/** @type {import("esbuild").BuildOptions} */
const buildOptions = {
  entryPoints: ["lib/index.ts"],
  outdir: "dist",
  assetNames: "[name]",
  platform: "node",
  loader: { ".node": "copy" },
  mainFields: ["main"],
  format: "cjs",
  bundle: true,
  // minify: true,
};

await esbuild.build(buildOptions);
