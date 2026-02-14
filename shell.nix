{
  system ? builtins.currentSystem,
}:
let
  inherit (import ./lib.nix { inherit system; }) pkgs craneLib;

  cargo-rail-version = "0.8.1";
  cargo-rail = craneLib.buildPackage {
    pname = "cargo-rail";
    version = cargo-rail-version;
    src = pkgs.fetchFromGitHub {
      owner = "loadingalias";
      repo = "cargo-rail";
      tag = "v${cargo-rail-version}";
      hash = "sha256-GlApp4rJ/X5lSD2c3KJ5ll0ZBXEIY3DbWwMM1O/ryXw=";
    };
    doCheck = false;
  };
in
craneLib.devShell {
  packages = [
    pkgs.binaryen
    pkgs.bun
    pkgs.cargo-insta
    cargo-rail
    pkgs.dotnet-sdk_8
    pkgs.jdk21_headless
    pkgs.just
    pkgs.llvmPackages.bintools
    pkgs.nodejs_24
    pkgs.tree-sitter
    pkgs.uv
    pkgs.wasm-bindgen-cli_0_2_100
  ];
}
