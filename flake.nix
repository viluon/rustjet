{
  description = "rustjet - RegioJet Telegram bot";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        buildInputs = with pkgs; [
          openssl
        ];

      in {
        packages.default = rustPlatform.buildRustPackage {
          pname = "rustjet";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          inherit nativeBuildInputs buildInputs;

          meta = {
            description = "Telegram bot for managing RegioJet tickets";
            license = pkgs.lib.licenses.mit;
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.openapi-generator-cli
            pkgs.clang
            pkgs.just
            pkgs.mold
          ] ++ nativeBuildInputs ++ buildInputs;

          shellHook = ''
            export RUST_BACKTRACE=1
            echo "rustjet dev environment"
            echo "Rust: $(rustc --version)"
          '';
        };

        devShells.openapi = pkgs.mkShell {
          buildInputs = [
            pkgs.openapi-generator-cli
            pkgs.curl
            pkgs.jq
          ];
        };
      }
    );
}