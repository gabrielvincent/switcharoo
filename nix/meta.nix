{ pkgs }:
{
  mainProgram = "hyprshell";
  description = "A modern GTK4-based window switcher and application launcher for Hyprland";
  homepage = "https://github.com/h3rmt/hyprshell";
  license = pkgs.lib.licenses.mit;
  platforms = pkgs.hyprland.meta.platforms;
}
