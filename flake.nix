{
  description = "The format that's super!";

  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        rustChannel = "stable";
        rustVersion = "latest";

        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        inherit (pkgs) lib;

        craneLib = (crane.mkLib pkgs).overrideToolchain (
          pkgs:
          pkgs.rust-bin.${rustChannel}.${rustVersion}.default.override {
            targets = [ "wasm32-unknown-unknown" ];
          }
        );

        cargo-rail = craneLib.buildPackage rec {
          pname = "cargo-rail";
          version = "0.8.1";
          src = pkgs.fetchFromGitHub {
            owner = "loadingalias";
            repo = "cargo-rail";
            tag = "v${version}";
            hash = "sha256-GlApp4rJ/X5lSD2c3KJ5ll0ZBXEIY3DbWwMM1O/ryXw=";
          };
          doCheck = false;
        };

        src = lib.fileset.toSource {
          root = ./.;
          fileset = lib.fileset.unions [
            ./Cargo.toml
            ./Cargo.lock
            (craneLib.fileset.commonCargoSources ./axum_duper)
            (craneLib.fileset.commonCargoSources ./duper)
            (craneLib.fileset.commonCargoSources ./duper-js-node)
            (craneLib.fileset.commonCargoSources ./duper-js-wasm/rust)
            (craneLib.fileset.commonCargoSources ./duper-python)
            (craneLib.fileset.commonCargoSources ./duper_lsp)
            (craneLib.fileset.commonCargoSources ./duper_uniffi)
            (craneLib.fileset.commonCargoSources ./duper_website)
            (craneLib.fileset.commonCargoSources ./duperfmt)
            (craneLib.fileset.commonCargoSources ./duperq)
            (craneLib.fileset.commonCargoSources ./serde_duper)
            (craneLib.fileset.commonCargoSources ./serde_duper_macros)
            (craneLib.fileset.commonCargoSources ./tracing_duper)
            (craneLib.fileset.commonCargoSources ./tree-sitter-duper)
            (lib.fileset.fileFilter (file: file.hasExt "md") ./.)
            ./.config
            ./.cargo/config.toml
            ./duper/src/visitor/snapshots
            ./duper/src/serde/snapshots
            ./duperfmt/src/duper.scm
            ./duperfmt/src/snapshots
            ./duperq/tests/data
            ./duper_uniffi/src/duper.udl
            ./tree-sitter-duper/src
            ./tree-sitter-duper/queries
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
          }
        );

        duperq = craneLib.buildPackage (
          individualCrateArgs
          // {
            inherit (craneLib.crateNameFromCargoToml { cargoToml = ./duperq/Cargo.toml; }) pname version;
            cargoExtraArgs = "-p duperq";
          }
        );

        duper_lsp = craneLib.buildPackage (
          individualCrateArgs
          // {
            inherit (craneLib.crateNameFromCargoToml { cargoToml = ./duper_lsp/Cargo.toml; }) pname version;
            cargoExtraArgs = "-p duper_lsp";
          }
        );
      in
      {
        packages = {
          inherit duperfmt duperq duper_lsp;
        };

        apps = {
          duperfmt =
            (flake-utils.lib.mkApp {
              drv = duperfmt;
            })
            // {
              meta = {
                description = "Official Duper formatting library and CLI";
                homepage = "https://duper.dev.br";
                license = lib.licenses.mit;
                mainProgram = "sandhole";
                platforms = lib.platforms.linux ++ lib.platforms.darwin;
              };
            };
          duperq =
            (flake-utils.lib.mkApp {
              drv = duperq;
            })
            // {
              meta = {
                description = "A fast Duper and JSON filter/processor";
                homepage = "https://duper.dev.br";
                license = lib.licenses.mit;
                mainProgram = "sandhole";
                platforms = lib.platforms.linux ++ lib.platforms.darwin;
              };
            };
          duper_lsp =
            (flake-utils.lib.mkApp {
              drv = duper_lsp;
            })
            // {
              meta = {
                description = "Official Duper language server, with auto-formatting and diagnostics";
                homepage = "https://duper.dev.br";
                license = lib.licenses.mit;
                mainProgram = "sandhole";
                platforms = lib.platforms.linux ++ lib.platforms.darwin;
              };
            };
        };

        checks = {
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

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};

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
        };
      }
    );
}
