{
  self
}: {
  pkgs,
  config,
  lib,
  ...
}:
let
  inherit (lib.types)
    either
    bool
    float
    int
    enum
    lines
    listOf
    nullOr
    package
    path
    str
    submodule
    ;

  cfg = config.programs.hyprshell;
  mkOpt =
    description: type: default:
    lib.mkOption { inherit description type default; };
in
{
  options.programs.hyprshell = {
    enable = lib.mkEnableOption "Configure Hyprshell";

    package = lib.mkOption {
      description = "The Hyprshell package";
      type = package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.hyprshell;
    };

    systemd = {
      enable = lib.mkEnableOption "Hyprshell systemd service" // { default = true; };
      target = lib.mkOption {
        description = "The systemd target that will automatically start the Hyprshell service";
        type = str;
        default = config.wayland.systemd.target;
      };
    };

    styleFile = lib.mkOption {
      description = ''
        File containing Hyprshell CSS overrides (either a file or text).
      '';
      type = nullOr (either path lines);
      default = null;
    };

    configFile = lib.mkOption {
      description = ''
        File containing Hyprshell configuration (either a file or text).
        Can be used instead of generated config via settings.
      '';
      type = nullOr (either path lines);
      default = null;
    };

    settings = {
      layerrules = mkOpt "Enable layer rules" bool true;
      launcher = {
        enable = mkOpt "Enable app launcher" bool true;
        width = mkOpt "Launcher width" int 650;
        max_items = mkOpt "Max shown items" int 5;
        animate_launch_ms = mkOpt "Launcher close duration" int 250;
        default_terminal = mkOpt "Default terminal" (nullOr (str)) null;

        plugins = {
          calc = mkOpt "Calculator" bool true;
          shell = mkOpt "Run in Shell" bool false;
          terminal = mkOpt "Run in Terminal" bool true;
          applications = {
            enable = mkOpt "Open applications" bool true;
            cache = mkOpt "Run Cache weeks" int 4;
            execs = mkOpt "Show execs" bool true;
          };
          websearch = {
            enable = mkOpt "Web search" bool true;
            engines =
              mkOpt "Search engines" (listOf (submodule {
                options = {
                  url = mkOpt "Search engine URL" str null;
                  name = mkOpt "Search engine name" str null;
                  key = mkOpt "Key to use for search engine" str null // {
                    apply = key: if (builtins.stringLength key) != 1 then throw "Key must be single character" else key;
                  };
                };
              })) [ ]
              // {
                example = [
                  {
                    url = "https://www.google.com/search?q={}";
                    name = "Google";
                    key = "g";
                  }
                ];
              };
          };
        };

        window = {
          enable = mkOpt "Enable windows (overview, switch)" bool true;
          scale = mkOpt "Scale" float 8.5 // {
            apply = num: if (num >= 0 && num <= 15) then num else throw "Value must be between 0 and 15";
          };
          workspaces_per_row = mkOpt "Workspaces per row" int 5;
          strip_html_from_workspace_title = mkOpt "Strip HTML from workspace title" bool true;
          overview = {
            enable = mkOpt "Enable overview" bool true;
            open = {
              key = mkOpt "Key to open overview" str "tab";
              modifier = mkOpt "Modifier key" (enum [
                "alt"
                "ctrl"
                "super"
                "shift"
              ]) "super";
            };
            navigate = {
              forward = mkOpt "Key to navigate forwards" str "tab";
              reverse = mkOpt "Key to navigate backwards" str "Mod(shift)" // {
                example = "Key(grave)";
              };
            };
            other = {
              filter_by = mkOpt "Filter by" (listOf (enum [
                "same_class"
                "current_monitor"
                "current_workspace"
              ])) [ ];
              hide_filtered = mkOpt "Hide filtered windows" bool false;
            };
          };
          switcher = {
            enable = mkOpt "Enable window switcher" bool true;
            open = {
              modifier = mkOpt "Modifier key" (enum [
                "alt"
                "ctrl"
                "super"
                "shift"
              ]) "alt";
            };
            navigate = {
              forward = mkOpt "Key to navigate forwards" str "tab";
              reverse = mkOpt "Key to navigate backwards" str "Mod(shift)" // {
                example = "Key(grave)";
              };
            };
            other = {
              filter_by = mkOpt "Filter by" (listOf (enum [
                "same_class"
                "current_monitor"
                "current_workspace"
              ])) [ ];
              hide_filtered = mkOpt "Hide filtered windows" bool true;
            };
          };
        };
      };
    };
  };

  config = lib.mkIf cfg.enable ({
    home.packages = [ cfg.package ];

    systemd.user.services.hyprshell = lib.mkIf cfg.systemd.enable {
      Unit = {
        Description = "Starts Hyprshell daemon";
        After = [ cfg.systemd.target ];
      };
      Service = {
        Type = "simple";
        ExecStart = "${lib.getExe cfg.package} run";
        Restart = "on-failure";
      };
      Install.WantedBy = [ cfg.systemd.target ];
    };

    xdg.configFile."hyprshell/config.ron" = {
      # source = if (lib.isPath cfg.configFile || lib.isStorePath cfg.configFile) then cfg.configFile else null;
      text =
        if (builtins.isString cfg.configFile) then
          cfg.configFile
        else
          builtins.toJSON cfg.settings;
    };

    xdg.configFile."hyprshell/style.css" = {
      # source = if (lib.isPath cfg.styleFile || lib.isStorePath cfg.styleFile) then cfg.styleFile else null;
      text = if (builtins.isString cfg.styleFile) then cfg.styleFile else "";
    };
  });
}
