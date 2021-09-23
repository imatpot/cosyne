let
  mozOverlay = import (fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/0510159186dd2ef46e5464484fbdf119393afa58.tar.gz);
  nixpkgs = import (fetchTarball https://github.com/NixOS/nixpkgs/archive/1a56d76d718afb6c47dd96602c915b6d23f7c45d.tar.gz) {
    overlays = [ mozOverlay ];
  };

  rustChannel = nixpkgs.rustChannelOf {
    date = "2021-09-09"; # https://github.com/rust-lang/rust/blob/master/RELEASES.md
    channel = "stable";
  };

in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "mozilla_overlay";
    buildInputs = [
      rustChannel.rust
    ];
  }
