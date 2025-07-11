{
  description = "simple redirects";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
  };
  
  outputs = {self, nixpkgs, rust-overlay, crane, ... }:
      let
        system = "x86_64-linux";
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        craneLib = (crane.mkLib nixpkgs.legacyPackages.${system});

        unfilteredRoot = ./.;
        src = pkgs.lib.fileset.toSource {
          root = unfilteredRoot;
          fileset = pkgs.lib.fileset.unions [
            # Default files from crane (Rust and cargo files)
            (craneLib.fileset.commonCargoSources unfilteredRoot)
            (pkgs.lib.fileset.maybeMissing ./static)
          ];
        };

        cargoArtifacts = craneLib.buildDepsOnly ({
          inherit src;

          buildInputs = with pkgs; [
            protobuf
            pkg-config
            openssl
          ];

          pname = "simple_se";
        });
      in
      with pkgs;
      {
        devShells.${system}.default = mkShell {
          buildInputs = [
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" "rust-analyzer" ];
            })
            protobuf
            pkg-config
            openssl
          ];
        };

        packages.${system} = rec {
          simple_se = craneLib.buildPackage {
            inherit src;

            inherit cargoArtifacts;

            buildInputs = [
              protobuf
              pkg-config
              openssl
            ];

            postInstall = ''
              install -Dm644 "$src/static/images/URLbar.png" "$out/static/images/URLbar.png"
              install -Dm644 "$src/static/index.html" -t "$out/static/"
              install -Dm644 "$src/static/styles.css" -t "$out/static/"
            '';
          };

          default = simple_se;
        };

        overlays.default = final: prev: {
          simple_se = self.packages.${system}.simple_se;
        };

        nixosModules.simple-se-module =
          { config, lib, pkgs, ... }:
          let
            simple_se = self.packages.${system}.default;
          in{
            options.services.simple_se = with lib; {
              enable = mkOption {
                type = types.bool;
                default = false;
                description = "Enable the Simple_SE service.";
              };

              port = mkOption {
                type = types.int;
                default = 4433;
                description = "The port the Simple_SE server should listen on.";
              };
            };

            config = lib.mkIf config.services.simple_se.enable {
              systemd.services.simple_se-http-server = {
                description = "Simple_SE HTTP Server";
                after = [ "network.target" ];
                wantedBy = [ "multi-user.target" ];
                serviceConfig = {
                  ExecStart = "${simple_se}/bin/simple_se --port ${toString config.services.simple_se.port}";
                  Restart = "always";
                  User = "www-data";
                  Group = "www-data";
                  WorkingDirectory = "${simple_se}";
                };
              };
            };
          };

        formatter.x86_64-linux = nixpkgs.legacyPackages.${system}.nixpkgs-fmt;
      };
}
