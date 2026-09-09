{
  description = "chess-raw — Tauri v2 + Svelte development shell";

  inputs = {
    # Pinned to the SAME revision as nixos-config/flake.lock (nixpkgs input).
    # Everything below is therefore already in the local store: entering this
    # shell downloads nothing. Bump this rev when you bump the system flake.
    nixpkgs.url = "github:NixOS/nixpkgs/331800de5053fcebacf6813adb5db9c9dca22a0c";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };

      # The C libraries the Tauri `-sys` crates look for through pkg-config.
      # glib-sys, gobject-sys, gtk-sys, soup3-sys, javascriptcore-rs-sys,
      # libdbus-sys and webkit2gtk-sys all fail their build scripts without
      # these. They are needed at build time AND at run time.
      tauriLibs = with pkgs; [
        webkitgtk_4_1
        gtk3
        libsoup_3
        glib
        cairo
        pango
        gdk-pixbuf
        atk
        harfbuzz
        librsvg
        dbus
        openssl
      ];
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          pkg-config
          gobject-introspection
        ];

        buildInputs = tauriLibs;

        # pkg-config finds the build-time metadata through PKG_CONFIG_PATH,
        # which mkShell sets from buildInputs. The two exports below cover
        # run time: the dynamic loader and the GSettings schemas.
        shellHook = ''
          export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath tauriLibs}''${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
          export XDG_DATA_DIRS="''${GSETTINGS_SCHEMAS_PATH:+$GSETTINGS_SCHEMAS_PATH:}$XDG_DATA_DIRS"
        '';
      };
    };
}
