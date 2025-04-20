{ pkgs, self }:
pkgs.mkShell {
  inputsFrom = [ (pkgs.callPackage ./default.nix { inherit self; }) ];
  buildInputs = with pkgs; [
    rust-analyzer
    rustfmt
    clippy
  ];
}