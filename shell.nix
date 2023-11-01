{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "liborzklv";

  buildInputs = with pkgs; [
    pkgs.clang
    pkgs.cmake
    pkgs.ninja
    pkgs.lldb
  ];

  shellHook = ''
    echo "Welcome to liborzklv!"
  '';
}