{ pkgs ? import <nixpkgs> {} }:

let
  libraries = with pkgs; [
    webkitgtk_4_1
    gtk3
    cairo
    gdk-pixbuf
    glib
    dbus
    openssl_3
    libGL
  ];

  packages = with pkgs; [
    pkg-config
    dbus
    openssl_3
    glib
    gtk3
    libsoup
    webkitgtk_4_1
    appimagekit
    curl
    clang
    llvmPackages.bintools
    bun
    sbclPackages.cl-rsvg2
    cargo
    rustc
    rust-analyzer
    clippy
  ];

in
pkgs.mkShell {
  buildInputs = packages;

  shellHook = ''
    export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
    export WEBKIT_DISABLE_COMPOSITING_MODE=1

    bun install
  '';
}
