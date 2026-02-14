{
  system ? builtins.currentSystem,
}:
{
  inherit (import ./lib.nix { inherit system; })
    duperfmt
    duperq
    duper_lsp
    ;
}
