{ pkgs ? import <nixpkgs> { } }:

let
  cargo-aoc-src = (import ./npins).cargo-aoc;
  cargo-aoc = pkgs.rustPlatform.buildRustPackage {
    pname = "cargo-aoc";
    version = cargo-aoc-src.version;
    src = cargo-aoc-src;
    cargoLock.lockFile = "${cargo-aoc-src}/Cargo.lock";

    buildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin
      (with pkgs.darwin.apple_sdk.frameworks; [
        CoreServices
        SystemConfiguration
      ]);

    nativeBuildInputs = pkgs.lib.optional pkgs.stdenv.isDarwin pkgs.pkg-config;
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
  ];
}
