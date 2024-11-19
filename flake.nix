{
  description = "Only Provide the support of qemu";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ 
          (import rust-overlay)
          (_: super: {
            rust-toolchain =
              let
                rust = super.rust-bin;
              in
              if builtins.pathExists ./rust-toolchain.toml then
                rust.fromRustupToolchainFile ./rust-toolchain.toml
              else if builtins.pathExists ./rust-toolchain then
                rust.fromRustupToolchainFile ./rust-toolchain
              else
                # rust.nightly.latest.default;
                # The rust-toolchain when i make this file, which maybe change
                (rust.nightly.latest.default.override {
                  extensions = [ "rust-src" "llvm-tools-preview" "rustfmt" "clippy" "cargo" ];
                  targets = [ "x86_64-unknown-linux-gnu" ];
                });
          })
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in {
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            # Require of repos
            gnumake

            # Require of rust-overlay
            openssl
            pkg-config
            eza
            fd

            # Development Tools
            ripgrep
            zellij
            fzf

            rust-toolchain
          ];

          shellHook = ''
            alias ls=eza
            alias find=fd

            export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
            export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
          '';
        };
      }
    );
}
