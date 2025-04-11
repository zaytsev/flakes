{
  description = "Basic Rust dev environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        overlays = [fenix.overlays.default];
        pkgs = import nixpkgs {inherit system overlays;};

        projectConfig = builtins.fromTOML (builtins.readFile ./Cargo.toml);

        package = let
          toolchain = fenix.packages.${system}.stable.toolchain;
        in
          (pkgs.makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          })
          .buildRustPackage {
            pname = projectConfig.package.name;
            version = projectConfig.package.version;
            src = ./.;

            doCheck = false;

            nativeBuildInputs = with pkgs; [
              pkg-config
              openssl
            ];

            buildInputs = with pkgs; [
              pkg-config
              openssl
            ];

            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
      in rec {
        packages = flake-utils.lib.flattenTree {
          main = package;
        };
        defaultPackage = packages.main;

        devShell =
          pkgs.mkShell
          {
            buildInputs = with pkgs; [
              (
                with fenix.packages.${system};
                  combine [
                    stable.rustc
                    stable.cargo
                    stable.rust-src
                    stable.rustfmt
                    # rust-analyzer
                  ]
              )
              cargo-edit
              cargo-update
              cargo-geiger
              cargo-outdated
              cargo-audit

              cocogitto
            ];
          };
      }
    );
}
