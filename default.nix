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
  getLibFolder = pkg: "${pkg}/lib";

  # Manifest via Cargo.toml
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
  pkgs.stdenv.mkDerivation {
    # Package related things automatically
    # obtained from Cargo.toml, so you don't
    # have to do everything manually
    pname = manifest.name;
    version = manifest.version;

    # Your govnocodes
    src = pkgs.lib.cleanSource ./.;

    cargoDeps = pkgs.rustPlatform.importCargoLock {
      lockFile = ./Cargo.lock;
      # Use this if you have dependencies from git instead
      # of crates.io in your Cargo.toml
      # outputHashes = {
      #   # Sha256 of the git repository, doesn't matter if it's monorepo
      #   "example-0.1.0" = "sha256-80EwvwMPY+rYyti8DMG4hGEpz/8Pya5TGjsbOBF0P0c=";
      # };
    };

    # Compile time dependencies
    nativeBuildInputs = with pkgs; [
      appstream
      appstream-glib
      cargo
      rust-analyzer
      clippy
      desktop-file-utils
      gettext
      git
      meson
      ninja
      pkg-config
      polkit
      rustc
      rustPlatform.cargoSetupHook
      wrapGAppsHook4
    ];

    # Runtime dependencies which will be shipped
    # with nix package
    buildInputs = with pkgs; [
      desktop-file-utils
      gdk-pixbuf
      glib
      gnome-desktop
      adwaita-icon-theme
      gtk4
      libadwaita
      libgweather
      openssl
      parted
      rustPlatform.bindgenHook
      vte-gtk4
    ];

    # Compiler LD variables
    NIX_LDFLAGS = "-L${(getLibFolder pkgs.libiconv)}";
    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
      pkgs.gcc
      pkgs.libiconv
      pkgs.llvmPackages.llvm
    ];

    meta = with lib; {
      homepage = manifest.homepage;
      description = manifest.description;
      # https://github.com/NixOS/nixpkgs/blob/master/lib/licenses.nix
      license = with lib.licenses; [asl20 mit];
      platforms = with platforms; linux ++ darwin;
      maintainers = [
        {
          name = "Xinux Developers";
          email = "support@floss.uz";
          handle = "orzklv";
          github = "xinux-org";
        }
      ];
    };
  }
