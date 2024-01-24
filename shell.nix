{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    nativeBuildInputs = [ 
      pkgs.buildPackages.gcc
      pkgs.buildPackages.bintools
    ];
}
