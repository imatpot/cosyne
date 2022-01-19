# https://nixos.wiki/wiki/Flakes#Using_flakes_project_from_a_legacy_Nix

(import
  (
    let
      lock = builtins.fromJSON (builtins.readFile ./flake.lock);
      compat = lock.nodes.flake-compat.locked;

    in
    fetchTarball {
      url = "https://github.com/edolstra/flake-compat/archive/${compat.rev}.tar.gz";
      sha256 = compat.narHash;
    }
  )
  {
    src = ./.;
  }
).defaultNix
