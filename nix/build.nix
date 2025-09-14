{
  self,
  craneLib,
  pkgs,
}:
rec {
  pname = "hyprshell";
  version = (pkgs.lib.trivial.importTOML ../Cargo.toml).workspace.package.version;
  # no more filtering, excluded to many files
  src = ../.;
  meta = {
    mainProgram = pname;
    description = "A modern GTK4-based window switcher and application launcher for Hyprland";
    homepage = "https://github.com/h3rmt/hyprshell";
    license = pkgs.lib.licenses.mit;
    platforms = pkgs.hyprland.meta.platforms;
  };
  stdenv = p: p.stdenv;
  wrapProgram = ''
    wrapProgram $out/bin/hyprshell \
      --prefix PATH : ${pkgs.lib.makeBinPath [ pkgs.gcc ]} \
      --prefix CPATH : ${
        pkgs.lib.makeIncludePath (
          pkgs.hyprland.buildInputs
          ++ [
            pkgs.hyprland
            pkgs.pixman
          ]
        )
      }
  '';
  commonArgs = {
    inherit
      src
      stdenv
      pname
      version
      meta
      ;
    strictDeps = true;
    doCheck = false;
    cargoBuildCommand = "cargo build --release --locked";
    cargoTestCommand = "";
    cargoCheckCommand = "";
    cargoCheckExtraArgs = "";
    cargoExtraArgs = "";

    nativeBuildInputs = [
      pkgs.pkg-config
      pkgs.wrapGAppsHook4
      pkgs.makeBinaryWrapper
    ];

    buildInputs = [
      pkgs.gtk4
      pkgs.gtk4-layer-shell
    ];
  };

  cargoArtifacts = craneLib.buildDepsOnly (
    commonArgs
    // {
      mkDummySrc = craneLib.mkDummySrc {
        inherit stdenv;
        src = ../.;
      };
      pname = "hyprshell";
    }
  );
  cargoFullArtifacts = craneLib.buildDepsOnly (
    commonArgs
    // {
      mkDummySrc = craneLib.mkDummySrc {
        inherit stdenv;
        src = craneLib.cleanCargoSource ../.;
      };
      pname = "hyprshell-full";
      doCheck = true;
      cargoBuildCommand = "cargo build --profile dev --locked --all-targets --all-features";
      cargoCheckCommand = "cargo check --profile dev --locked --all-targets --all-features";
      cargoTestCommand = "cargo test --profile dev --locked --all-targets --all-features";
    }
  );

  commonArgsFullCached = (
    commonArgs
    // {
      cargoArtifacts = cargoFullArtifacts;
    }
  );
}
