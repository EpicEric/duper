{
  system ? builtins.currentSystem,
  rustChannel ? "stable",
  rustVersion ? "latest",
}:
let
  sources = import ./npins;

  pkgs = import sources.nixpkgs {
    inherit system;
    overlays = [ (import sources.rust-overlay) ];
  };

  inherit (pkgs) lib;

  craneLib = (import sources.crane { inherit pkgs; }).overrideToolchain (
    p:
    p.rust-bin.${rustChannel}.${rustVersion}.default.override {
      targets = [
        "wasm32-unknown-unknown"
        "wasm32-wasip2"
      ];
    }
  );

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./Cargo.toml
      ./Cargo.lock
      ./.config
      ./.cargo/config.toml
      ./axum_duper/Cargo.toml
      ./axum_duper/src
      ./duper/Cargo.toml
      ./duper/src
      ./duper-js-node/Cargo.toml
      ./duper-js-node/src
      ./duper-js-wasm/rust/Cargo.toml
      ./duper-js-wasm/rust/src
      ./duper-python/Cargo.toml
      ./duper-python/src
      ./duper_lsp/Cargo.toml
      ./duper_lsp/src
      ./duper_rpc/Cargo.toml
      ./duper_rpc/src
      ./duper_uniffi/build.rs
      ./duper_uniffi/Cargo.toml
      ./duper_uniffi/src
      ./duper_website/Cargo.toml
      ./duper_website/src
      ./duper_zed/Cargo.toml
      ./duper_zed/src
      ./duperfmt/Cargo.toml
      ./duperfmt/src
      ./duperq/Cargo.toml
      ./duperq/src
      ./duperq/tests/data
      ./serde_duper/Cargo.toml
      ./serde_duper/src
      ./serde_duper_macros/Cargo.toml
      ./serde_duper_macros/src
      ./tracing_duper/Cargo.toml
      ./tracing_duper/src
      ./tree-sitter-duper/Cargo.toml
      ./tree-sitter-duper/bindings/rust
      ./tree-sitter-duper/src
      ./tree-sitter-duper/queries
      (lib.fileset.fileFilter (file: file.hasExt "md") ./.)
    ];
  };

  commonArgs = {
    inherit src;
    strictDeps = true;
    version = "0";
    pname = "duper";

    nativeBuildInputs = with pkgs; [
      cmake
      llvmPackages.bintools
      python3
    ];
  };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  individualCrateArgs = commonArgs // {
    inherit cargoArtifacts;
    doCheck = false;
  };

  duperfmt = craneLib.buildPackage (
    individualCrateArgs
    // {
      inherit (craneLib.crateNameFromCargoToml { cargoToml = ./duperfmt/Cargo.toml; }) pname version;
      cargoExtraArgs = "-p duperfmt";
      meta.mainProgram = "duperfmt";
    }
  );

  duperq = craneLib.buildPackage (
    individualCrateArgs
    // {
      inherit (craneLib.crateNameFromCargoToml { cargoToml = ./duperq/Cargo.toml; }) pname version;
      cargoExtraArgs = "-p duperq";
      meta.mainProgram = "duperq";
    }
  );

  duper_lsp = craneLib.buildPackage (
    individualCrateArgs
    // {
      inherit (craneLib.crateNameFromCargoToml { cargoToml = ./duper_lsp/Cargo.toml; }) pname version;
      cargoExtraArgs = "-p duper_lsp";
      meta.mainProgram = "duper_lsp";
    }
  );
in
{
  inherit
    pkgs
    lib
    craneLib
    commonArgs
    cargoArtifacts
    duperfmt
    duperq
    duper_lsp
    ;

  checks = {
    inherit
      duperfmt
      duperq
      duper_lsp
      ;

    duper-clippy = craneLib.cargoClippy (
      commonArgs
      // {
        inherit cargoArtifacts;
      }
    );

    duper-doc = craneLib.cargoDoc (
      commonArgs
      // {
        inherit cargoArtifacts;
      }
    );

    duper-fmt = craneLib.cargoFmt (
      commonArgs
      // {
        inherit cargoArtifacts;
      }
    );

    duper-test = craneLib.cargoNextest (
      commonArgs
      // {
        inherit cargoArtifacts;
        cargoNextestExtraArgs = "-P nix";
      }
    );
  };
}
