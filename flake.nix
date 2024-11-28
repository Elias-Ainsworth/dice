{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
  };

  outputs =
    {
      self,
      nixpkgs,
      devenv,
      # systems,
      ...
    }@inputs:
    let
      forEachSystem =
        function:
        nixpkgs.lib.genAttrs [ "x86_64-linux" ] (system: function nixpkgs.legacyPackages.${system});
    in
    {
      devShells = forEachSystem (pkgs: {
        default = devenv.lib.mkShell {
          inherit inputs pkgs;
          modules = [
            {
              # https://devenv.sh/reference/options/
              dotenv.disableHint = true;

              packages =
                with pkgs;
                [
                  cargo-edit
                  pkg-config
                  dioxus-cli
                ]
                ++ [
                  atk
                  cairo
                  dbus
                  gdk-pixbuf
                  glib
                  gtk3
                  libappindicator
                  libsoup_3
                  openssl_3
                  pango
                  webkitgtk_4_1
                  xdotool
                ];

              env = {
                XDG_DATA_DIRS = "${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}";
                GIO_MODULE_DIR = "${pkgs.glib-networking}/lib/gio/modules/";
                # FIXME: fix lag on wayland?
                # https://github.com/tauri-apps/tauri/issues/7354#issuecomment-1620910100
                # WEBKIT_DISABLE_COMPOSITING_MODE = 1;
              };

              languages.rust.enable = true;
            }
          ];
        };
      });

      packages = forEachSystem (pkgs: rec {
        dice = pkgs.callPackage ./package.nix {
          version =
            if self ? "shortRev" then
              self.shortRev
            else
              nixpkgs.lib.replaceStrings [ "-dirty" ] [ "" ] self.dirtyShortRev;
        };
        default = dice;
      });
    };
}
