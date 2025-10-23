{
  description = "A CLI tool to generate Nix dev shells";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      crane,
      rust-overlay,
      ...
    }@inputs:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [
          (import rust-overlay)
          crane.overlay
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };

        craneLib = pkgs.craneLib.overrideToolchain rustToolchain;

        nix-shell-gen = craneLib.buildPackage {
          pname = "nix-shell-gen";
          src = craneLib.cleanCargoSource (craneLib.path ./.);

          nativeBuildInputs = [ ];
        };

      in
      {
        packages.default = nix-shell-gen;

        apps.default = flake-utils.lib.mkApp {
          drv = nix-shell-gen;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ nix-shell-gen ];

          packages = with pkgs; [
            rustToolchain
            rust-analyzer
            cargo-watch
            cargo-edit
          ];

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };
      }
    );
}
