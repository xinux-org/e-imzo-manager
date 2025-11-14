{
  pkgs ? let
    lock = (builtins.fromJSON (builtins.readFile ./flake.lock)).nodes.nixpkgs.locked;
    nixpkgs = fetchTarball {
      url = "https://github.com/nixos/nixpkgs/archive/${lock.rev}.tar.gz";
      sha256 = lock.narHash;
    };
  in
    import nixpkgs {overlays = [];},
  ...
}: let
  # Helpful nix function
  lib = pkgs.lib;

  # Manifest via Cargo.toml
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
  pkgs.stdenv.mkDerivation {
    pname = manifest.name;
    version = manifest.version;

    # Your govnocodes
    src = pkgs.lib.cleanSource ./.;

    cargoDeps = pkgs.rustPlatform.importCargoLock {
      lockFile = ./Cargo.lock;
    };

    # Compile time dependencies
    nativeBuildInputs = with pkgs; [
      meson
      ninja
      pkg-config
      cargo
      rustPlatform.cargoSetupHook
      rustc
      desktop-file-utils
      wrapGAppsHook4
    ];

    # Runtime dependencies which will be shipped
    # with nix package
    buildInputs = with pkgs; [
      gdk-pixbuf
      glib
      gnome-desktop
      adwaita-icon-theme
      gtk4
      libadwaita
      openssl
      rustPlatform.bindgenHook
      polkit
    ];

    meta = {
      homepage = manifest.homepage;
      description = manifest.description;
      license = with lib.licenses; [agpl3Plus];
      platforms = lib.platforms.linux;
      teams = [lib.teams.uzinfocom];
    };
  }
