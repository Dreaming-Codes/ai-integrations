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
    in {
    defaultPackage.${system} = with pkgs; mkShell {
      buildInputs = [
        stdenv
        pkg-config
        dbus
        openssl_3
        glib
        gtk3
        libsoup
        webkitgtk_4_1
        webkitgtk
        zlib
        libthai
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
        appmenu-gtk3-module
        mesa
        libdrm
        xorg.libSM
        xorg.libICE
      ];

      shellHook = ''
        export LD_LIBRARY_PATH=${lib.makeLibraryPath [
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
        ]}:$LD_LIBRARY_PATH
        export LD_LIBRARY_PATH=${stdenv.cc.cc.lib}/lib/:$LD_LIBRARY_PATH
        export WEBKIT_DISABLE_COMPOSITING_MODE=1
        bun install
      '';
    };
  };
}
