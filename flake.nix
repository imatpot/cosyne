{
  description = "A work-in-progress, general purpose Discord bot written in Rust.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix, flake-compat }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ fenix.overlay ];
        pkgs = import nixpkgs { inherit system overlays; };

      in
      with pkgs; {
        devShell = mkShell {
          buildInputs = [
            (pkgs.fenix.stable.withComponents [
              "cargo"
              "rustc"
              "rustfmt"
              "rust-src"
            ])
          ];
        };
      }
    );
}
