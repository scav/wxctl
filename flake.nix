{
  description = "wxctl is a CLI for interacting with weather services";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
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
        in
        {
          default = pkgs.mkShell {
            buildInputs = [ toolchain ];
          };
        }
      );
      packages = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "wxctl";
            version = "v0.0.3";
            src = self;
            cargoHash = "sha256-2uQzqZl0EVHF3zSsV5fJaPDFn09cTVYBQ6/Kq7aeU+I=";
            meta = with pkgs.lib; {
              description = "CLI tool for interacting with weather services";
              license = licenses.unlicense;
            };
          };
        }
      );

      hydraJobs = forAllSystems (system: {
        wxctl = self.packages.${system}.default;
      });

    };
}
