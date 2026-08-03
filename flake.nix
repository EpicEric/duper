{
  description = "The format that's super!";

  outputs =
    { self, ... }@args:
    let
      inputs = import ./.tack { overrides = args.tackOverrides or { }; };

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      eachSystem =
        f:
        (builtins.foldl' (
          acc: system:
          let
            fSystem = f system;
          in
          builtins.foldl' (
            acc': attr:
            acc'
            // {
              ${attr} = (acc'.${attr} or { }) // fSystem.${attr};
            }
          ) acc (builtins.attrNames fSystem)
        ) { } systems);
    in
    eachSystem (
      system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ (import inputs.rust-overlay) ];
        };
        craneLib = (import inputs.crane { inherit pkgs; }).overrideToolchain (
          p:
          p.rust-bin.stable.latest.default.override {
            targets = [
              "wasm32-unknown-unknown"
              "wasm32-wasip2"
            ];
          }
        );

        inherit (import ./nix { inherit system pkgs craneLib; })
          packages
          checks
          shell
          ;
        inherit (pkgs) lib;
      in
      {
        packages.${system} = packages;

        apps.${system} = {
          duperfmt = {
            type = "app";
            program = lib.getExe self.packages.${system}.duperfmt;
            meta = {
              description = "Official Duper formatting library and CLI";
              homepage = "https://duper.dev.br";
              license = lib.licenses.mit;
              mainProgram = "duperfmt";
              platforms = lib.platforms.linux ++ lib.platforms.darwin;
            };
          };
          duperq = {
            type = "app";
            program = lib.getExe self.packages.${system}.duperq;
            meta = {
              description = "A fast Duper and JSON filter/processor";
              homepage = "https://duper.dev.br";
              license = lib.licenses.mit;
              mainProgram = "duperq";
              platforms = lib.platforms.linux ++ lib.platforms.darwin;
            };
          };
          duper_lsp = {
            type = "app";
            program = lib.getExe self.packages.${system}.duper_lsp;
            meta = {
              description = "Official Duper language server, with auto-formatting and diagnostics";
              homepage = "https://duper.dev.br";
              license = lib.licenses.mit;
              mainProgram = "duper_lsp";
              platforms = lib.platforms.linux ++ lib.platforms.darwin;
            };
          };
        };

        checks.${system} = checks;

        devShells.${system}.default = shell;
      }
    );
}
