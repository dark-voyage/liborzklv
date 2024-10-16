{ pkgs ? import <nixpkgs> {} }:
let
  getLibFolder = pkg: "${pkg}/lib";
  getFramwork = pkg: "${pkg}/Library/Frameworks";
  darwinOptions = if pkgs.stdenv.isDarwin then ''
    -F${(getFramwork pkgs.darwin.apple_sdk.frameworks.Security)}
    -F${(getFramwork pkgs.darwin.apple_sdk.frameworks.CoreFoundation)}
    -F${(getFramwork pkgs.darwin.apple_sdk.frameworks.CoreServices)}
    -F${(getFramwork pkgs.darwin.apple_sdk.frameworks.SystemConfiguration)}
  '' else "";
in
pkgs.stdenv.mkDerivation {
  name = "orzklv";

  nativeBuildInputs = with pkgs; [
  # LLVM & GCC
  gcc
  cmake
  gnumake
  pkg-config
  llvmPackages.llvm
  llvmPackages.clang

  # Hail the Nix
  nixd
  nixpkgs-fmt

  # Launch scripts
  just

  # Rust
  rustc
  cargo
  clippy
  rustfmt
  cargo-watch
  rust-analyzer
  ];

  buildInputs = with pkgs; [
    openssl
    darwin.apple_sdk.frameworks.Security
    darwin.apple_sdk.frameworks.CoreServices
    darwin.apple_sdk.frameworks.CoreFoundation
    darwin.apple_sdk.frameworks.SystemConfiguration
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
  NIX_LDFLAGS = "-L${(getLibFolder pkgs.libiconv)} ${darwinOptions}";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    (getLibFolder pkgs.gcc)
    (getLibFolder pkgs.libiconv)
    (getLibFolder pkgs.llvmPackages.llvm)
  ];
}
