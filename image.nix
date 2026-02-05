{ pkgs ? import <nixpkgs> { } }:
let
  rs3www = pkgs.rustPlatform.buildRustPackage {
    pname = "rs3www";
    version = "0.1.0";
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;
  };
in
  pkgs.dockerTools.buildLayeredImage {
    name = "rs3www";
    tag = "latest";
    contents = [ rs3www ];
    config = {
      Entrypoint = ["/bin/rs3www"];
    };
  }