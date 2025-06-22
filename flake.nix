{
  description = "hyprshell - A Rust-based GUI designed to enhance window management in hyprland";
  inputs.nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  inputs.home-manager.inputs.nixpkgs.follows = "nixpkgs";
  inputs.home-manager.url = "github:nix-community/home-manager";
  outputs =
    {
      self,
      nixpkgs,
      home-manager,
    }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = nixpkgs.legacyPackages;
    in
    {
      formatter = forAllSystems (system: pkgsFor.${system}.nixfmt-tree);
      packages = forAllSystems (system: rec {
        hyprshell = pkgsFor.${system}.callPackage ./nix/default.nix { inherit self; };
        hyprshell-with-test-command = pkgsFor.${system}.callPackage ./nix/default.nix {
          inherit self;
          features = [ "config_check_is_default" ];
        };
        default = hyprshell;
      });
      devShells = forAllSystems (system: rec {
        hyprshell = pkgsFor.${system}.callPackage ./nix/shell.nix { inherit self; };
        default = hyprshell;
      });
      homeModules = rec {
        hyprshell = import ./nix/module.nix { inherit self; };
        default = hyprshell;
      };
      
      checks = forAllSystems (
        system:
        let
          filterDisabledAndDropEnable =
            (import ./nix/util.nix { lib = pkgsFor.${system}.lib; }).filterDisabledAndDropEnable;
        in
        {
          test = pkgsFor.${system}.runCommand "test" { } ''
            TMP=$(mktemp -d)
            touch "$TMP/test.json"
            echo "test json created at $TMP"
            cat <<EOF> "$TMP/test.json"
            ${builtins.toJSON (
              filterDisabledAndDropEnable
                self.homeConfigurations.test.${system}.config.programs.hyprshell.settings
            )}
            EOF
            ${pkgsFor.${system}.jq}/bin/jq < "$TMP/test.json"
            ${
              self.packages.${system}.hyprshell-with-test-command
            }/bin/hyprshell config check-if-default -c "$TMP/test.json" && (mkdir "$out")
            rm -r "$TMP"
          '';
        }
      );
      homeConfigurations.test = forAllSystems (
        system:
        home-manager.lib.homeManagerConfiguration rec {
          pkgs = pkgsFor.${system};
          modules = [
            self.homeModules.hyprshell
            {
              home.stateVersion = "24.05";
              home.username = "test";
              home.homeDirectory = "/home/test";
            }
          ];
        }
      );
    };
}
