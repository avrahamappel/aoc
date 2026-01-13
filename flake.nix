{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { nixpkgs, flake-utils, ... }: flake-utils.lib.eachDefaultSystem (
    system:
    let
      pkgs = import nixpkgs { inherit system; };

      inherit (pkgs) lib stdenv;
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
