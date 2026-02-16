# wxctl
wxctl is a simpe CLI to interact with weather services and output the result to stdout.

Use cases could be status bars, terminal apps and so on.

## Supported weather services
- [Open-Meteo](https://open-meteo.com)


## Development
Have Nix, enter folder.

The build happens in a different repository, but its possible to build this using
```nix
{ pkgs }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "wxctl";
  version = "";

  src = pkgs.fetchFromGitHub {
    owner = "scav";
    repo = "wxctl";
    tag = version;
    hash = "";
  };

  nativeBuildInputs = [ pkgs.git ];

  cargoHash = "";
  cargoLock.lockFile = ./Cargo.lock;

  meta = with pkgs.lib; {
    description = "CLI tool for interacting with weather services";
    license = licenses.unlicense;
  };
}
```

