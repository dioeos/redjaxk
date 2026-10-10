{
  description = "Redjaxk Flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-26.05";
  };

  outputs =
    { self, nixpkgs }:
    let
      platforms = [
        "x86_64-linux"
      ];

      forAllPlatforms = f: nixpkgs.lib.genAttrs platforms (sys: f nixpkgs.legacyPackages.${sys});

      mkRedjaxkPackages = pkgs: {
        agent = pkgs.rustPlatform.buildRustPackage {
          pname = "redjaxk-agent";
          version = "0.1.0";

          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          cargoBuildFlags = [
            "-p"
            "redjaxk-agent"
          ];

          nativeBuildInputs = [ pkgs.protobuf ];
        };

        podgate = pkgs.rustPlatform.buildRustPackage {
          pname = "redjaxk-podgate";
          version = "0.1.0";

          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          cargoBuildFlags = [
            "-p"
            "redjaxk-podgate"
          ];

          nativeBuildINputs = [ pkgs.protobuf ];
        };
      };
    in
    {
      packages = forAllPlatforms (pkgs: mkRedjaxkPackages pkgs);

      homeManagerModules.default = { pkgs, lib, ...}: {
        imports = [
          ./nix/modules/agent.nix
          ./nix/modules/podgate.nix
        ];

        services.redjaxk-agent = {
          package = lib.mkDefault
            self.packages.${pkgs.stdenv.hostPlatform.system}.agent;
        };

        services.redjaxk-podgate = {
          package = lib.mkDefault
            self.packages.${pkgs.stdenv.hostPlatform.system}.podgate;
        };
      };


      devShells = forAllPlatforms (
        pkgs:
        let
          redjaxkPackages = mkRedjaxkPackages pkgs;
        in
        {
          default = pkgs.mkShell {
            inputsFrom = [
              redjaxkPackages.agent
            ];
            packages = with pkgs; [
              rustfmt
              clippy
              rust-analyzer
              cargo-watch

              sqlx-cli
              sqlite

              nixd
              nixfmt
              just

              stdenv.cc
            ];
          };
        }
      );
    };
}
