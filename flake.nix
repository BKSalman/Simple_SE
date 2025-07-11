{
  description = "simple redirects";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    rust-overlay,
    crane,
    flake-parts,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      perSystem = {
        pkgs,
        system,
        ...
      }: let
        craneLib = crane.mkLib nixpkgs.legacyPackages.${system};

        unfilteredRoot = ./.;
        src = pkgs.lib.fileset.toSource {
          root = unfilteredRoot;
          fileset = pkgs.lib.fileset.unions [
            # Default files from crane (Rust and cargo files)
            (craneLib.fileset.commonCargoSources unfilteredRoot)
            (pkgs.lib.fileset.maybeMissing ./static)
          ];
        };

        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src;

          buildInputs = with pkgs; [
            protobuf
            pkg-config
            openssl
          ];

          pname = "simple_se";
        };
      in
        with pkgs; {
          devShells.default = mkShell {
            buildInputs = [
              (rust-bin.stable.latest.default.override {
                extensions = ["rust-src" "rust-analyzer"];
              })
              protobuf
              pkg-config
              openssl
            ];
          };

          packages = rec {
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

          formatter = pkgs.alejandra;
        };

      flake = {
        overlays.default = final: prev: {
          simple_se = self.packages.simple_se;
        };

        nixosModules.simple-se-module = {
          config,
          lib,
          pkgs,
          ...
        }: let
          simple_se = inputs.self.packages.${pkgs.system}.simple_se;
        in {
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
              after = ["network.target"];
              wantedBy = ["multi-user.target"];
              serviceConfig = {
                ExecStart = "${simple_se}/bin/simple_se --port ${toString config.services.simple_se.port}";
                Restart = "always";
                User = "simple-se";
                Group = "simple-se";
                WorkingDirectory = "${simple_se}";
              };
            };

            users.users.simple-se = {
              isSystemUser = true;
              group = "simple-se";
              description = "Simple SE service user";
            };
            users.groups.simple-se = {};
          };
        };
      };
    };
}
