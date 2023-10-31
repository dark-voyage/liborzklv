{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "liborzklv";

  buildInputs = with pkgs; [
    pkgs.clang
    pkgs.cmake
    pkgs.glibc
    pkgs.ninja
    pkgs.valgrind
  ];

  shellHook = ''
    echo "Welcome to liborzklv!"
  '';
}