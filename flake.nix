{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };
  outputs = {self, nixpkgs, flake-utils, rust-overlay}@inputs: 
    flake-utils.lib.eachDefaultSystem (system:
      let
        inherit (nixpkgs) lib;
        pkgs = import nixpkgs {
          inherit system;
          overlays = [(import rust-overlay)];
        };

        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs.buildPackages; [ rust-bin.beta.latest.default ];
          packages = [];
        };

        packages.default = pkgs.rustPlatform.buildRustPackage rec {
          pname = manifest.name;
          version = manifest.version;
          cargoLock.lockFile = ./Cargo.lock;
          src = pkgs.lib.cleanSource ./.;
        };
        nixosModules.default = {config, pkg, lib, ...}: {
          options.services.rnote-export.enable = lib.mkEnableOption "Enable rnote_export module";

          config = lib.mkIf config.services.rnote-export.enable {
            environment.systemPackages = [
              self.packages.${system}.default
            ];
          };
        };

    });
}
