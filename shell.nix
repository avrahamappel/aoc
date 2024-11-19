let
  pins = import ./npins;

  pkgs = import pins.nixpkgs-unstable { };

  inherit (pkgs) lib stdenv;

  cargo-aoc-src = pins.cargo-aoc;
  cargo-aoc = pkgs.rustPlatform.buildRustPackage {
    pname = "cargo-aoc";
    version = cargo-aoc-src.version;
    src = cargo-aoc-src;
    cargoLock.lockFile = "${cargo-aoc-src}/Cargo.lock";

    buildInputs = lib.optionals stdenv.isDarwin
      (with pkgs.darwin.apple_sdk.frameworks; [
        CoreServices
        SystemConfiguration
      ]);

    nativeBuildInputs = lib.optional stdenv.isDarwin pkgs.pkg-config;
  };
in

pkgs.mkShell {
  packages = with pkgs; [
    cargo
    cargo-aoc
    clippy
    npins
    rustc
    rust-analyzer
    rustfmt
  ] ++ lib.optional stdenv.isDarwin iconv;

  nativeBuildInputs = lib.optional stdenv.isDarwin pkgs.pkg-config;
}
