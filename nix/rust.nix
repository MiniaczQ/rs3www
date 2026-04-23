{
  inputs,
  ...
}:
{
  perSystem =
    { pkgs, self', ... }:
    let
      fenixToolchain = (import inputs.fenix { inherit pkgs; }).stable;

      rustToolchain = fenixToolchain.withComponents [
        "rustc"
        "cargo"
        "rustfmt"
        "clippy"
      ];

      craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

      commonArgs = {
        src = craneLib.cleanCargoSource ../.;
        strictDeps = true;
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      rs3www = craneLib.buildPackage (
        commonArgs
        // {
          inherit cargoArtifacts;
        }
      );

      rs3www-image = pkgs.dockerTools.buildLayeredImage {
        name = "rs3www";
        tag = rs3www.version;
        contents = [
          rs3www
          pkgs.cacert
        ];
        config.Entrypoint = [ "/bin/rs3www" ];
      };

      rs3www-docs = craneLib.cargoDoc (
        commonArgs
        // {
          inherit cargoArtifacts;
          env.RUSTDOCFLAGS = "--deny warnings";
        }
      );

      rs3www-clippy = craneLib.cargoClippy (
        commonArgs
        // {
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        }
      );

    in
    {
      packages = {
        inherit
          rs3www
          rs3www-image
          rs3www-docs
          ;
      };

      checks = {
        inherit
          rs3www
          rs3www-image
          rs3www-docs
          rs3www-clippy
          ;
      };

      devShells.rust = craneLib.devShell {
        checks = self'.checks;
        packages = builtins.attrValues {
          inherit (pkgs)
            cargo-watch
            rust-analyzer
            ;
        };
      };

      treefmt.programs.rustfmt.package = fenixToolchain.rustfmt;
    };
}
