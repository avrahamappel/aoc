{ pkgs ? import <nixpkgs> { } }:

let
  cargo-aoc-src = (import ./npins).cargo-aoc;
  cargo-aoc = pkgs.rustPlatform.buildRustPackage {
    pname = "cargo-aoc";
    version = cargo-aoc-src.version;
    src = cargo-aoc-src;
    cargoLock.lockFile = "${cargo-aoc-src}/Cargo.lock";
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
