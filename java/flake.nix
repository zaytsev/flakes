{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (
      system:
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            jdk
            maven
            gradle
          ];
        };
      }
    );
}
