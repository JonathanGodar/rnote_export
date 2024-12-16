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
        nixosModules.default = {config, pkg, lib, ...}: let
          cfg = config.services.rnote-export;

          defaultUser = "rnote-export";
          defaultGroup = defaultUser;
          defaultOutputDirectory = "/var/lib/rnote-export";
          in {
            options.services.rnote-export = {
              enable = lib.mkEnableOption "Enable rnote_export module";

              inputDirectory = lib.mkOption {
                type = lib.types.path;
                description = "Directory to look for rnote_files";
              };

              outputDirectory = lib.mkOption {
                type = lib.types.path;
                description = "Directory to put the exported pngs";
                default = defaultOutputDirectory;
              };

              includeString = lib.mkOption {
                  type = lib.types.str;
                  description = "What rnote files should be included";
                  default = "**/*.rnote";
                };
              };

              config = lib.mkIf config.services.rnote-export.enable {
                environment.systemPackages = [
                  self.packages.${system}.default
                ];

                users.users.${defaultUser} = {
                  isSystemUser = true;
                  # isNormalUser = true;
                  group = defaultGroup;
                  # Make rnote-export command available for the borg user
                  packages = [self.packages.${system}.default];
                  extraGroups = ["users"];
                };

                users.groups.${defaultGroup} = {};

                systemd.tmpfiles.settings.rnote-export.${cfg.outputDirectory}.d = {
                  group = defaultGroup;
                  user = defaultUser;
                  mode = "0755";
                };

                systemd.services.rnote-export = {
                  enable = true;

                  wantedBy = [ "multi-user.target" ];
                  # Make rnote-export available for the systemd service
                  path = [self.packages.${system}.default];

                  serviceConfig = {
                    ExecStart = "${self.packages.${system}.default}/bin/rnote-export \"${cfg.inputDirectory}\" \"${cfg.outputDirectory}\" \"${cfg.includeString}\"";
                    User=defaultUser;
                  };
                
                };
              };
        };

    });
}
