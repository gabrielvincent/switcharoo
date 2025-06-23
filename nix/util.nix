{
  self,
  craneLib,
  pkgs,
  ...
}:
rec {
  filterDisabledAndDropEnable =
    value:
    if pkgs.lib.isAttrs value then
      if value ? enable && value.enable == false then
        null
      else
        pkgs.lib.filterAttrs (k: v: v != null && k != "enable") (
          pkgs.lib.mapAttrs (_: filterDisabledAndDropEnable) value
        )
    else if pkgs.lib.isList value then
      pkgs.lib.filter (v: v != null) (map filterDisabledAndDropEnable value)
    else
      value;

  pname = "hyprshell";
  markdownFilter = path: _type: builtins.match ".*(css|service)$" path != null;
  markdownOrCargo = path: type: (markdownFilter path type) || (craneLib.filterCargoSources path type);
  src = pkgs.lib.cleanSourceWith {
    src = ../.;
    filter = markdownOrCargo;
    name = "source";
  };
  version =
    (pkgs.lib.trivial.importTOML ../Cargo.toml).workspace.package.version
    + "_"
    + (self.shortRev or "dirty");
  meta = {
    mainProgram = pname;
    description = "A modern GTK4-based window switcher and application launcher for Hyprland";
    homepage = "https://github.com/h3rmt/hyprshell";
    license = pkgs.lib.licenses.mit;
    platforms = pkgs.hyprland.meta.platforms;
  };
  stdenv = p: p.stdenv;
  commonArgs = {
    inherit
      src
      stdenv
      meta
      pname
      version
      ;
    strictDeps = true;
    doCheck = false;
    cargoBuildCommand = "cargo build --profile dev";
    cargoTestCommand = "";
    cargoCheckCommand = "";

    nativeBuildInputs = [
      pkgs.pkg-config
      pkgs.wrapGAppsHook4
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
    }
  );

  commonArgsCached = (
    commonArgs
    // {
      cargoArtifacts = cargoArtifacts;
    }
  );
}
