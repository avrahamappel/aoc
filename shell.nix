let
  pins = import ./npins;

  pkgs = import pins.nixpkgs-unstable { };

  fenix = import pins.fenix { inherit pkgs; };

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
    bacon
    cargo
    cargo-aoc
    clippy
    npins
    rustc
    fenix.rust-analyzer
    rustfmt
  ] ++ lib.optional stdenv.isDarwin iconv;

  CARGO_TARGET_DIR = (builtins.toString ./.) + "/target";

  nativeBuildInputs = lib.optional stdenv.isDarwin pkgs.pkg-config;
}
