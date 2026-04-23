{ inputs, ... }:
{
  imports = [
    inputs.treefmt-nix.flakeModule
  ];
  perSystem =
    { pkgs, ... }:
    {
      treefmt = {
        projectRootFile = "flake.nix";
        settings.on-unmatched = "info";
        programs = {
          # keep-sorted start block=yes
          keep-sorted.enable = true;
          mdformat.enable = true;
          nixfmt.enable = true;
          rustfmt.enable = true;
          taplo.enable = true;
          # keep-sorted end
        };
      };
    };
}
