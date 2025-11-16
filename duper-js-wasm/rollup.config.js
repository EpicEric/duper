import typescript from "@rollup/plugin-typescript";
import wasm from "@rollup/plugin-wasm";
import terser from "@rollup/plugin-terser";

export default {
  input: "src/index.ts",
  output: {
    dir: "dist",
    format: "esm",
    sourcemap: true,
  },
  plugins: [typescript(), wasm({ maxFileSize: 100_000_000 }), terser()],
};
