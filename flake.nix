{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {  nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        devShells = {
          default = pkgs.mkShell {
            name = "dev";
            buildInputs = with pkgs; [
              # c
              gcc
              gdb
              perf
            
              # rust
              rustc
              cargo
              rustfmt
              clippy
              rust-analyzer
            ];

            shellHook = ''
              # export RUST_BACKTRACE="full"
              echo " : $(rustc --version)"
            '';
          };
        };
       }
    );
}
