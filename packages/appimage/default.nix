# example: https://github.com/bpftrace/bpftrace/blob/336bb4a2042767942f1b36368270c7b68a126e58/flake.nix#L267
#
{
  pkgs,
  inputs,
  ...
}:
let
  # Manifest via Cargo.toml
  manifest = (pkgs.lib.importTOML ../../Cargo.toml).package;
in
pkgs.appimageTools.wrapType2 {
  name = manifest.name;
  pname = manifest.name;
  src = ../..;
  version = manifest.version;

  extraPkgs =
    pkgs: with pkgs; [
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

      gtk4
      gnome-desktop
      libadwaita
      openssl
    ];
}
# inputs.nix-appimage.lib.${pkgs.stdenv.hostPlatform.system}.mkAppImage {
#   program = manifest.name;
#   name = "${manifest.name}.AppImage";

#   squashfsTools = with pkgs; [
#     rustc
#     cargo
#     appstream
#     appstream-glib
#     desktop-file-utils
#     gettext
#     meson
#     ninja
#     pkg-config
#     polkit
#     wrapGAppsHook4
#     rustPlatform.cargoSetupHook

#     gtk4
#     gnome-desktop
#     libadwaita
#     openssl
#   ];
# }
