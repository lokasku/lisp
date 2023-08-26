{
  description = "Desugar LISP implementation of McCarthy's Meta-Circular Evaluator.";
  
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix, naersk, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        toolchain = fenix.packages.${system}.complete;

        naersk-lib = naersk.lib.${system}.override {
          inherit (toolchain) cargo rustc;
        };

        lisp = naersk-lib.buildPackage {
          name = "lisp";
          src = ./.;
        };
      in {
        packages.lisp = lisp;
        defaultPackage = self.packages.${system}.lisp;

        devShell = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.packages.${system};
          nativeBuildInputs = [
            (toolchain.withComponents [
              "cargo" "rustc" "rust-src" "rustfmt" "clippy"
            ])
          ];
          RUST_BACKTRACE = 0;
        };
      });
}
