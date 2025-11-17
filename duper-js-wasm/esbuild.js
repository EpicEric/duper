import esbuild from "esbuild";
import { wasmLoader } from "esbuild-plugin-wasm";

// // https://github.com/evanw/esbuild/issues/408#issuecomment-757555771
// import path from "node:path"
// import fs from "node:fs"
// const wasmPlugin = {
//   name: "wasm",
//   setup(build) {
//     build.onResolve({ filter: /\.wasm$/ }, args => {
//       if (args.resolveDir === "") {
//         return;
//       }
//       return {
//         path: path.isAbsolute(args.path) ? args.path : path.join(args.resolveDir, args.path),
//         namespace: "wasm-binary",
//       };
//     })
//     build.onLoad({ filter: /.*/, namespace: "wasm-binary" }, async (args) => ({
//       contents: await fs.promises.readFile(args.path),
//       loader: "binary",
//     }));
//   },
// };

/** @type {import("esbuild").BuildOptions} */
const buildOptions = {
  entryPoints: ["src/index.ts"],
  outdir: "dist",
  assetNames: "[name][ext]",
  platform: "node",
  mainFields: ["main"],
  format: "esm",
  sourcemap: true,
  bundle: true,
  plugins: [
    wasmLoader()
  ],
  minify: true,
};

await esbuild.build(buildOptions);
