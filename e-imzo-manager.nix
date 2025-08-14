{
  stdenv,
  lib,
  libiconv,
  fetchFromGitHub,
  gcc,
  llvmPackages,
  # nativeBuildInputs
  appstream,
  appstream-glib,
  cargo,
  clippy,
  desktop-file-utils,
  gettext,
  git,
  meson,
  ninja,
  pkg-config,
  polkit,
  rustc,
  rustPlatform,
  wrapGAppsHook4,
  # buildInputs
  gdk-pixbuf,
  glib,
  gnome-desktop,
  adwaita-icon-theme,
  gtk4,
  libadwaita,
  libgweather,
  openssl,
  parted,
  vte-gtk4,
}: let
  # Helpful nix function
  getLibFolder = pkg: "${pkg}/lib";
in
  stdenv.mkDerivation rec {
    pname = "E-IMZO-Manager";
    version = "0.1.0";

    src = fetchFromGitHub {
      owner = "xinux-org";
      repo = "e-imzo";
      rev = version;
      hash = "sha256-gV644xWN/wy1SgBlAU/C78Xypt84NVktT85jXS4eWtY=";
    };

    cargoDeps = rustPlatform.fetchCargoVendor {
      inherit pname version src;
      hash = "sha256-rulWG4L/uN6+JBk+SzC0y57Pdw5N0Q1dJlpXGVo+vbQ=";
  };

    # Compile time dependencies
    nativeBuildInputs = [
      appstream
      appstream-glib
      cargo
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
    buildInputs = [
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
    NIX_LDFLAGS = "-L${(getLibFolder libiconv)}";
    LD_LIBRARY_PATH = lib.makeLibraryPath [
      gcc
      llvmPackages.llvm
    ];

    meta = with lib; {
      homepage = "https://github.com/xinux-org";
      description = "E-IMZO for uzbek state web key signing on Linux";
      license = with lib.licenses; [asl20 mit];
      platforms = platforms.linux;
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
