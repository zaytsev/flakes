{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (
      system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs { inherit system overlays; };
          rust-bin = pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" ];
            targets = [ "x86_64-unknown-linux-gnu" ];
            #targets = [ "x86_64-unknown-linux-gnu" "wasm32-unknown-unknown" "wasm32-wasi" ];
          };
        in
          {
            devShell = pkgs.mkShell {
              buildInputs = with pkgs; [
                rust-bin
                rust-analyzer
                cargo-generate
                cargo-edit
                cargo-update
                cargo-geiger
                cargo-outdated

                # If the project requires openssl, uncomment these
                #pkg-config
                #openssl
                #protobuf
              ];
              # If the project requires openssl, uncomment this
              # PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
            };
          }
    );
}
