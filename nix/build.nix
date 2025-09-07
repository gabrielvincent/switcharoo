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
  postInstall = ''
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
      ;
    strictDeps = true;
    doCheck = false;
    cargoBuildCommand = "cargo build --profile release --locked";
    cargoTestCommand = "";
    cargoCheckCommand = "";
    cargoCheckExtraArgs = "";
    cargoExtraArgs = "";

    nativeBuildInputs = [
      pkgs.pkg-config
      pkgs.wrapGAppsHook4
      pkgs.makeBinaryWrapper
      #      pkgs.makeWrapper
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
      pname = "hyprshell-release";
    }
  );
  cargoFullArtifacts = craneLib.buildDepsOnly (
    commonArgs
    // {
      mkDummySrc = craneLib.mkDummySrc {
        inherit stdenv;
        src = craneLib.cleanCargoSource ../.;
      };
      pname = "hyprshell-full-release";
      doCheck = true;
      cargoBuildCommand = "cargo build --profile release --locked --all-targets --all-features";
      cargoCheckCommand = "cargo check --release --locked --all-targets --all-features";
      cargoTestCommand = "cargo test --release --locked --all-targets --all-features";
    }
  );

  commonArgsCachedRelease = (
    commonArgs
    // {
      inherit cargoArtifacts meta;
    }
  );

  commonArgsFullCachedRelease = (
    commonArgs
    // {
      inherit meta;
      cargoArtifacts = cargoFullArtifacts;
    }
  );
}
