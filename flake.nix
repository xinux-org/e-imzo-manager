{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    {
      flake-utils,
      naersk,
      nixpkgs,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk { };

        app = naersk'.buildPackage rec {
          pname = "e-imzo-manager";
          src = ./.;

          nativeBuildInputs = commonNativeBuildInputs;
          buildInputs = commonBuildInputs;

          preFixup = ''
                    gappsWrapperArgs+=(
                      # The icon theme is hardcoded.
                      --prefix XDG_DATA_DIRS : "$XDG_ICON_DIRS"
                    )

                    for f in $(find $out/bin/ $out/libexec/ -type f -executable); do
              wrapProgram "$f" \
                --prefix GIO_EXTRA_MODULES : "${pkgs.lib.getLib pkgs.dconf}/lib/gio/modules" \
                --prefix XDG_DATA_DIRS : "$out/share" \
                --prefix XDG_DATA_DIRS : "$out/share/gsettings-schemas/${pname}" \
                --prefix XDG_DATA_DIRS : "${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}" \
                --prefix XDG_DATA_DIRS : "${pkgs.hicolor-icon-theme}/share" \
                --prefix GI_TYPELIB_PATH : "${
                  pkgs.lib.makeSearchPath "lib/girepository-1.0" (
                    with pkgs;
                    [
                      pango
                      json-glib
                    ]
                  )
                }"
            done
          '';
        };

        commonBuildInputs = with pkgs; [
          gtk4
          libadwaita
          openssl
          pango
          pkg-config
          graphene
          wrapGAppsHook4
          gtk4
          glib
          libadwaita
          desktop-file-utils
          # polkit
        ];

        commonNativeBuildInputs = with pkgs; [
          desktop-file-utils
          gettext
          git
          meson
          ninja
          pkg-config
          wrapGAppsHook4
          pango
          graphene
          glib
          gobject-introspection
          cairo
          gtk4
          libadwaita
          gdk-pixbuf
        ];

      in
      {
        # For `nix build` & `nix run`:
        packages.default = app.overrideAttrs (p: {
          SOME_ENV_VAR = "yes";
        });

        # For `nix develop` (optional, can be skipped):
        devShell = import ./shells/e-imzo-manager/default.nix { inherit pkgs; };
        formatter = pkgs.nixfmt-tree;
      }
    );
}
