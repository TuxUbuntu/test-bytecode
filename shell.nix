
# start overlay
let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
# end overlay

with import <nixpkgs> {};

# Make a new "derivation" that represents our shell
stdenv.mkDerivation {
  name = "bytecode";

  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

  # The packages in the `buildInputs` list will be added to the PATH in our shell
  buildInputs = [
    # see https://nixos.org/nixos/packages.html to search for more

    # use overlay
    #nixpkgs.latest.rustChannels.nightly.rust
    (nixpkgs.rustChannelOf { date = "2022-03-11"; channel = "nightly"; }).rust

    pkgs.watchexec
    pkgs.just

  ];

}

