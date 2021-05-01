{ pkgs ? import <nixpkgs> {} }:

with pkgs;

let
  LORRI_ROOT = toString ./.;
in
mkShell {
  buildInputs = [
    #(callPackage ./default.nix {})
    cargo
    cargo-edit
    openssl
    pkg-config
    rustfmt
    rustc
  ];

  CARGO_NET_GIT_FETCH_WITH_CLI = "true";
  CARGO_INSTALL_ROOT = "${LORRI_ROOT}/.cargo";
}
