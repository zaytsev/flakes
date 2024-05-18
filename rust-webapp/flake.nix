{
  description = "Basic Rust dev environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    flake-compat,
    fenix,
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        overlays = [fenix.overlays.default];
        pkgs = import nixpkgs {inherit system overlays;};

        workspaceConfig = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        projectConfig = builtins.fromTOML (builtins.readFile ./app/Cargo.toml);

        pkgName = projectConfig.package.name;
        pkgVersion = workspaceConfig.workspace.package.version;

        server = let
          toolchain = fenix.packages.${system}.stable.toolchain;
        in
          (pkgs.makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          })
          .buildRustPackage {
            pname = pkgName;
            version = pkgVersion; # projectConfig.package.version;
            src = ./.;

            doCheck = false;

            cargoBuildFlags = [
              "--bin"
              "app"
            ];

            nativeBuildInputs = with pkgs; [
            ];

            buildInputs = with pkgs; [
            ];

            preBuild = ''
              export PG_TMP_DATA_DIR=`${pkgs.ephemeralpg}/bin/pg_tmp initdb`
              export PG_TMP_URL=`${pkgs.ephemeralpg}/bin/pg_tmp -d $PG_TMP_DATA_DIR start`
            '';

            postBuild = ''
              pg_tmp -d $PG_TMP_DATA_DIR stop || true
            '';

            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
        image = pkgs.dockerTools.streamLayeredImage {
          name = pkgName;
          tag = pkgVersion;
          contents = [server];
          config = {
            Entrypoint = ["${server}/bin/app"];
          };
        };
      in {
        packages = flake-utils.lib.flattenTree {
          inherit server image;
        };
        defaultPackage = server;

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
                  ]
              )
              cargo-edit
              cargo-update
              cargo-geiger
              cargo-outdated
              cargo-audit
              cargo-expand

              cocogitto

              podman
              podman-compose
            ];
          };
      }
    );
}
