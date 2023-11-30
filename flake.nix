{
  description = "Bun + Tauri 2 flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    chaotic.url = "github:chaotic-cx/nyx/nyxpkgs-unstable";
  };

  outputs = { self, nixpkgs, chaotic }: let
    system = "x86_64-linux";
    overlayedNixpkgs = import nixpkgs {
      system = system;
      overlays = [ chaotic.overlays.default ];
    };
    pkgs = overlayedNixpkgs.pkgs;

    # Import the libopenraw definition
    libopenraw = pkgs.callPackage ./libopenraw.nix {};

  in {
    defaultPackage.${system} = with pkgs; let
      commonLibs = [
        llvmPackages_15.clangUseLLVM
        llvm_15
        stdenv
        webkitgtk_4_1
        webkitgtk
        libthai
        zlib
        gtk3
        cairo
        gdk-pixbuf
        glib
        dbus
        openssl_3
        libGL
        xorg.libX11
        xorg.libxcb
        libgpg-error
        fontconfig
        freetype
        harfbuzz
        fribidi
        cups
        colord
        pango
        wayland
        libcanberra-gtk3
        appmenu-gtk3-module
        mesa
        libdrm
        xorg.libSM
        xorg.libICE
        libopenraw
        libjpeg8
        libavif
        librsvg
        libjxl
        libheif
        libwmf
        expat
        libpng
        p11-kit
        gmp
        libtiff
        libffi
        util-linux
        libxml2
        gnutls
        libsoup
        sbclPackages.cl-rsvg2
        libclang
        tesseract
        leptonica
      ];

      additionalBuildInputs = [
        pkg-config
        appimagekit
        clang
        curl
        llvmPackages.bintools
        bun
        rustc
        cargo
        clippy
        rust-analyzer
      ];

      in mkShell {
        buildInputs = commonLibs ++ additionalBuildInputs;

        shellHook = ''
          export LD_LIBRARY_PATH=${pkgs.stdenv.cc.cc.lib}/lib/:$LD_LIBRARY_PATH
          export LD_LIBRARY_PATH=${lib.makeLibraryPath commonLibs}:$LD_LIBRARY_PATH
          export LIBCLANG_PATH=${pkgs.libclang.lib}/lib

          bun install
        '';
      };
  };
}
