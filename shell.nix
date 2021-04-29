{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    cargo
    #(callPackage ./default.nix {})
    cargo-edit
  ];

  CARGO_NET_GIT_FETCH_WITH_CLI = "true";
}
