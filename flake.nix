{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { nixpkgs, flake-utils, ... }: flake-utils.lib.eachDefaultSystem (
    system:
    let
      pkgs = import nixpkgs { inherit system; };

      inherit (pkgs) lib stdenv;

      pins = import ./npins;

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
    {
      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          bacon
          cargo
          cargo-aoc
          clippy
          npins
          rustc
          rust-analyzer
          rustfmt
        ] ++ lib.optional stdenv.isDarwin iconv;

        shellHook = ''
          export CARGO_TARGET_DIR="$(pwd)/target"
        '';

        nativeBuildInputs = lib.optional stdenv.isDarwin pkgs.pkg-config;
      };
    }
  );
}
