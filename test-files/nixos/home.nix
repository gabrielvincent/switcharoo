{ lib, inputs, ... }:
{
  home-manager = {
    useGlobalPkgs = true;
    extraSpecialArgs = { inherit inputs; };
    users.enrico = import ./enrico.nix;
  };
}
