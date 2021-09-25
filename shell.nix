let
  environment = import ./environment.nix;

  fenix-tarball = builtins.fetchTarball https://github.com/nix-community/fenix/archive/5860fbb9954d23a09dba45d7a0bba53eb558573d.tar.gz;
  nixpkgs = import (builtins.fetchTarball https://github.com/NixOS/nixpkgs/archive/1a56d76d718afb6c47dd96602c915b6d23f7c45d.tar.gz) {
    overlays = [ (import "${fenix-tarball}/overlay.nix") ];
  };

in
  with nixpkgs; mkShell (
    environment // {
      buildInputs = [
        (
          fenix.stable.withComponents [
            "cargo"
            "rustc"
            "rustfmt"
            "rust-src"
          ]
        )
      ];
    }
  )
