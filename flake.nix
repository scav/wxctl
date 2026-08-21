{
  description = "wxctl is a CLI for interacting with weather services";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      fenix,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
          toolchain = fenix.packages.${system}.stable.toolchain;
          craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
        in
        {
          default = craneLib.devShell {
            buildInputs = [ ];
          };
        }
      );
      packages = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
          inherit (pkgs) lib;

          toolchain = fenix.packages.${system}.stable.toolchain;
          craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
          src = craneLib.cleanCargoSource ./.;

          commonArgs = {
            inherit src;
            strictDeps = true;
          };

          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          individualCrateArgs = commonArgs // {
            inherit cargoArtifacts;
            inherit (craneLib.crateNameFromCargoToml { inherit src; }) version;
            doCheck = false;
          };

          fileSetForCrate =
            crate:
            lib.fileset.toSource {
              root = ./.;
              fileset = lib.fileset.unions [
                ./Cargo.toml
                ./Cargo.lock
                (craneLib.fileset.commonCargoSources ./wx)
                (craneLib.fileset.commonCargoSources crate)
              ];
            };
          wxctl = craneLib.buildPackage (
            individualCrateArgs
            // {
              pname = "wxctl";
              cargoExtraArgs = "-p wxctl";
              src = fileSetForCrate ./wxctl;
            }
          );
        in
        {
          default = wxctl;
        }
      );

      hydraJobs = forAllSystems (system: {
        wxctl = self.packages.${system}.default;
      });

    };
}
