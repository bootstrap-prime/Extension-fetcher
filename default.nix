{ rustPlatform, lib, pkg-config, openssl }:

rustPlatform.buildRustPackage rec {
  pname = "extensionfetcher";
  version = "0.1.0";

  src = ./.;

  buildInputs = [ pkg-config openssl ];

  cargoSha256 = "9+p2hTDG+UsLBg52qc/k96TqQKAkvfe1fpVUVddcYpc=";

  meta = with lib; {
    description = "A fast, rust tool to interact with the mozilla addons api and output a nix derivation";
    license = licenses.mit;
    maintainers = [ maintainers.bootstrap-prime ];
  };
}
