{
  pkgs,
  inputs,
  ...
}:
let
  # Helpful nix function
  lib = pkgs.lib;

  # Manifest via Cargo.toml
  manifest = (pkgs.lib.importTOML ../../Cargo.toml).package;

  craneLib = inputs.crane.mkLib pkgs;

  commonBuildInputs = with pkgs; [
    gtk4
    libadwaita
    openssl
    polkit
  ];

  commonNativeBuildInputs = with pkgs; [
    desktop-file-utils
    gettext
    git
    meson
    ninja
    pkg-config
    wrapGAppsHook4
  ];

  cargoArtifacts = craneLib.buildDepsOnly {
    src = craneLib.cleanCargoSource ../..;
    strictDeps = true;

    nativeBuildInputs = commonNativeBuildInputs;
    buildInputs = commonBuildInputs;
  };
in
craneLib.buildPackage {
  pname = manifest.name;
  version = manifest.version;
  strictDeps = true;

  src = pkgs.lib.cleanSource ../..;

  inherit cargoArtifacts;

  nativeBuildInputs = commonNativeBuildInputs;
  buildInputs = commonBuildInputs;

  configurePhase = ''
    mesonConfigurePhase
    runHook postConfigure
  '';

  buildPhase = ''
    runHook preBuild
    ninjaBuildPhase
    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall
    mesonInstallPhase
    runHook postInstall
  '';

  doNotPostBuildInstallCargoBinaries = true;
  checkPhase = false;

  meta = {
    homepage = manifest.homepage;
    description = manifest.description;
    license = with lib.licenses; [ agpl3Plus ];
    platforms = lib.platforms.linux;
    teams = [ lib.teams.uzinfocom ];
  };
}
