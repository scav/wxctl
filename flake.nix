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
    { nixpkgs, fenix, ... }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-darwin"
      ];
    in
    {
      devShells = builtins.listToAttrs (
        map (system: {
          name = system;
          value =
            let
              pkgs = import nixpkgs { inherit system; };
              toolchain = fenix.packages.${system}.stable.toolchain;
            in
            {
              default = pkgs.mkShell {
                buildInputs = [ toolchain ];
              };
            };
        }) systems
      );
    };
}
