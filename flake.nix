{
  description = "hyprshell - A Rust-based GUI designed to enhance window management in hyprland";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };
  outputs =
    inputs@{
      self,
      nixpkgs,
      home-manager,
      flake-parts,
      crane,
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "aarch64-linux"
        "x86_64-linux"
      ];
      perSystem =
        { pkgs, self', ... }:
        let
          craneLib = crane.mkLib pkgs;
          buildLib = import ./nix/build.nix { inherit craneLib pkgs; };
        in
        {
          formatter = pkgs.nixfmt-tree;
          packages = rec {
            hyprshell-no-hyprland-wrap = craneLib.buildPackage (
              buildLib.commonArgs
              // {
                cargoArtifacts = buildLib.cargoReleaseArtifacts;
                postInstall = buildLib.wrapWithGcc;
              }
            );
            hyprshell = self.helpers.wrap-hyprshell pkgs.hyprland pkgs;
            default = hyprshell;
          };
          checks = import ./nix/checks.nix {
            inherit
              self
              pkgs
              craneLib
              buildLib
              home-manager
              ;
          };
          devShells.default = craneLib.devShell {
            checks = self'.checks;
            stdenv = buildLib.stdenv;
            packages = [
              pkgs.rust-analyzer
            ];
          };
        };
      flake = {
        homeModules = rec {
          hyprshell = import ./nix/module.nix self;
          default = hyprshell;
        };
        helpers = {
          wrap-hyprshell =
            hyprland: pkgs:
            pkgs.runCommand "hyprshell"
              {
                buildInputs = [ pkgs.makeBinaryWrapper ];
                # TODO doesnt work, getExe still complains
                meta = import ./nix/meta.nix { inherit pkgs; };
              }
              ''
                mkdir -p $out/bin
                cp ${
                  self.packages.${pkgs.stdenv.hostPlatform.system}.hyprshell-no-hyprland-wrap
                }/bin/hyprshell $out/bin/hyprshell
                wrapProgram $out/bin/hyprshell \
                  --prefix CPATH : ${
                    pkgs.lib.makeIncludePath (
                      hyprland.buildInputs
                      ++ [
                        hyprland
                      ]
                    )
                  }
              '';
        };
      };
    };
}
