{
  description = "The format that's super!";

  outputs =
    { ... }:
    let
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
        inherit (import ./lib.nix { inherit system; })
          duperfmt
          duperq
          duper_lsp
          lib
          checks
          ;
      in
      {
        packages.${system} = {
          inherit duperfmt duperq duper_lsp;
        };

        apps.${system} = {
          duperfmt = {
            type = "app";
            program = lib.getExe duperfmt;
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
            program = lib.getExe duperq;
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
            program = lib.getExe duper_lsp;
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

        devShells.${system}.default = import ./shell.nix { inherit system; };
      }
    );
}
