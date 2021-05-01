{ pkgs ? import <nixpkgs> {} }:

with pkgs;

let
  LORRI_ROOT = toString ./.;
in
mkShell {
  buildInputs = [
    cargo
    #(callPackage ./default.nix {})
    cargo-edit
    #openssl
    #curl
    #pkg-config
    rustfmt
    rustc
  ];

  CARGO_NET_GIT_FETCH_WITH_CLI = "true";
  #RUST_SRC_PATH = "${pkgs.rustc.src}/lib/rustlib/src/rust/src/";
  CARGO_INSTALL_ROOT = "${LORRI_ROOT}/.cargo";
}
