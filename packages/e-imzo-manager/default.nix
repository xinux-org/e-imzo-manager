{
  pkgs,
  lib,
  ...
}:
let
  # Manifest via Cargo.toml
  manifest = (pkgs.lib.importTOML ../../Cargo.toml).package;
in
pkgs.stdenv.mkDerivation {
  pname = manifest.name;
  version = manifest.version;

  src = pkgs.lib.cleanSource ../..;
  cargoDeps = pkgs.rustPlatform.importCargoLock {
    lockFile = ../../Cargo.lock;
  };

  nativeBuildInputs = with pkgs; [
    rustc
    cargo
    appstream
    appstream-glib
    desktop-file-utils
    gettext
    meson
    ninja
    pkg-config
    polkit
    wrapGAppsHook4
    rustPlatform.cargoSetupHook
  ];

  buildInputs = with pkgs; [
    gtk4
    gnome-desktop
    libadwaita
    openssl
  ];

  meta = {
    homepage = manifest.homepage;
    description = manifest.description;
    license = with lib.licenses; [ agpl3Plus ];
    platforms = lib.platforms.linux;
    maintainers = with lib.maintainers; [
      bahrom04
      bemeritus
      orzklv
    ];
  };
}
