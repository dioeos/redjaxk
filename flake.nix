{
  description = "Redjaxk Flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-26.05";
  };

  outputs = { self, nixpkgs }:
  let
    platforms = [
      "x86_64-linux"
    ];

    forAllPlatforms = f: nixpkgs.lib.genAttrs platforms (sys: f nixpkgs.legacyPackages.${sys});
  in
  {
    devShells = forAllPlatforms (pkgs: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          cargo
          rustc
          rustfmt
          clippy
          rust-analyzer

          sqlx-cli
          sqlite

          nixd
          nixfmt
        ];
      };
    });
  };
}
