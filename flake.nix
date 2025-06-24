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
          buildLib = import ./nix/build.nix { inherit craneLib pkgs self; };
          customLib = import ./nix/util.nix { lib = pkgs.lib; };
        in
        {
          formatter = pkgs.nixfmt-tree;
          packages = rec {
            hyprshell = craneLib.buildPackage buildLib.commonArgsCached;
            default = hyprshell;
          };
          checks = rec {
            hyprshell-default-check = craneLib.buildPackage (
              buildLib.commonArgsCached
              // {
                cargoExtraArgs = "--features config_check_is_default";
              }
            );
            hyprshell-clippy = craneLib.cargoClippy (
              buildLib.commonArgsCached
              // {
                buildPhaseCargoCommand = "cargo clippy";
                cargoClippyExtraArgs = "--all-targets -- --deny warnings";
              }
            );
            hyprshell-fmt = craneLib.cargoFmt buildLib.commonArgsCached;
            check-nix-config = pkgs.runCommand "check-nix-config" { } ''
              TMP=$(mktemp -d)
              touch "$TMP/test.json"
              echo "test json created at $TMP"
              cat <<EOF> "$TMP/test.json"
              ${builtins.toJSON (
                customLib.filterDisabledAndDropEnable self.homeConfigurations.test.config.programs.hyprshell.settings
              )}
              EOF
              ${pkgs.jq}/bin/jq < "$TMP/test.json"
              ${hyprshell-default-check}/bin/hyprshell -vv config check-if-default -c "$TMP/test.json" && (mkdir "$out")
              rm -r "$TMP"
            '';
            check-all-feature-combinations = craneLib.buildPackage (
              buildLib.commonArgsCached
              // {
                pname = "check-all-feature-combinations";
                nativeBuildInputs = [ pkgs.bash ] ++ buildLib.commonArgs.nativeBuildInputs;
                buildPhaseCargoCommand = ''
                  cargoBuildLog=$(mktemp cargoBuildLogXXXX.json)

                  # Define the features as an array
                  # features=$(cargo metadata --format-version 1 --no-deps | ${pkgs.jq}/bin/jq -r '.packages[4].features | keys[]')
                  declare -a features=("generate_config_command" "config_check_is_default" "launcher_calc" "debug_command")

                  # Get the total number of features
                  num_features=''${#features[@]}

                  # Function to build with a specific combination of features
                  build_with_features() {
                    local feature_combination="$1"
                    local iteration="$2"
                    local start_time=$(date +%s.%N)

                    if [[ -z "$feature_combination" ]]; then
                      echo -n "[$iteration] Building without any features..."
                      cargo build --locked --no-default-features --message-format json-render-diagnostics > "$cargoBuildLog"
                    else
                      echo -n "[$iteration] Building with features: $feature_combination"
                      cargo build --locked --no-default-features --features "$feature_combination" --message-format json-render-diagnostics > "$cargoBuildLog"
                    fi

                    local duration=$(awk "BEGIN {print $(date +%s.%N) - $start_time}")
                    printf " took %.2f seconds\n" "$duration"
                  }

                  echo "num_features: $num_features, iterations: $((1 << num_features))"
                  for ((i = 0; i < (1 << num_features); i++)); do
                    combination=()
                    for ((j = num_features - 1; j >= 0; j--)); do
                      if ((i & (1 << j))); then
                        combination+=("''${features[j]}")
                      fi
                    done
                    build_with_features "$(IFS=,; printf '%s' "''${combination[*]}")" "$i"
                  done
                  echo "all features tested"
                '';
              }
            );
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
        homeConfigurations.test = home-manager.lib.homeManagerConfiguration {
          pkgs = nixpkgs.legacyPackages."x86_64-linux";
          modules = [
            self.homeModules.hyprshell
            {
              home.stateVersion = "24.05";
              home.username = "test";
              home.homeDirectory = "/home/test";
            }
          ];
        };
      };
    };
}
