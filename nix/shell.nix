{ pkgs, self }:
pkgs.mkShell {
  inputsFrom = [ (pkgs.callPackage ./default.nix { inherit self; }) ];
  buildInputs = with pkgs; [
    cargo
    rustc
    clippy
    rustfmt
    rust-analyzer
  ];
}
