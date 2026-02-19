# wxctl
wxctl is a simpe CLI to interact with weather services and output the result to stdout.

Use cases could be status bars, terminal apps and so on.

## Supported weather services backends
- [MET](https://api.met.no/)
- [Open-Meteo](https://open-meteo.com)

MET is the default backend

## Running

```bash
Get weather based on location and country.
If unable to lookup a value, output will be empty.

Usage: 

Options:
  -n, --name <NAME>        
  -c, --country <COUNTRY>  
  -d, --debug              Enable debug logging
  -b, --backend <BACKEND>  Select backend yr|open-meteo [default: yr]
  -h, --help               Print help
  -V, --version            Print version
``` 

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

