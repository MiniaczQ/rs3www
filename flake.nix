{
  description = "rs3www";

  inputs = {
    # keep-sorted start block=yes
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # keep-sorted end
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        # keep-sorted start
        ./nix/rust.nix
        ./nix/treefmt.nix
        # keep-sorted end
      ];
      perSystem =
        { self', ... }:
        {
          packages.default = self'.packages.rs3www;
          devShells.default = self'.devShells.rust;
        };
    };
}
