{ pkgs, ... }:
pkgs.mkShell {
  packages = with pkgs; [
    # Nix
    nixd
    statix
    deadnix
    nixfmt
    just
    just-lsp

    # Rust
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer
    cargo-watch
    cargo-expand
    openssl

    # Gnome related
    gtk4
    meson
    ninja
    pango
    polkit
    gettext
    pkg-config
    libadwaita
    gnome-desktop
    wrapGAppsHook4
    desktop-file-utils
    gobject-introspection
    rustPlatform.bindgenHook
  ];

  # Set Environment Variables
  RUST_BACKTRACE = "full";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
