{
  gnutar,
  jre,
  curl,
  pkgs ? import <nixpkgs> {},
  ...
}: let
  lib = pkgs.lib;

  exec = pkgs.writeShellScript "e-imzo" ''
    # Change working directory to script
    cd "$(dirname "$0")/../lib"

    # Start the damned server
    ${pkgs.jre8} -Xms512M -Xmx2048M -jar ../lib/E-IMZO.jar
  '';
in
  pkgs.stdenv.mkDerivation rec {
    pname = "e-imzo";
    version = "4.64";

    src = pkgs.fetchurl {
      url = "https://dls.yt.uz/E-IMZO-v${version}.tar.gz";
      hash = "sha256-ej99PJrO9ufJ8+VlC/HpfvS/bGBtKqUWcsRyiZRlU4c=";
    };

    buildInputs = with pkgs; [
      # The main hero
      jre8

      # Just in case for networking
      curl
    ];

    installPhase = ''
      # Executables folder
      mkdir -p $out/bin

      # Library folder
      mkdir -p $out/lib

      # Copy java thingies to lib
      cp -r ./lib $out/lib/
      cp ./E-IMZO.jar $out/lib/
      cp ./E-IMZO.pem $out/lib/
      cp ./truststore.jks $out/lib/

      # Copy the shell script to bin
      cp -r "${exec}" $out/bin/e-imzo
    '';

    meta = with lib; {
      homepage = "https://e-imzo.soliq.uz";
      description = "E-IMZO for uzbek web key signing.";
      licencse = lib.licenses.unfree;
      platforms = with platforms; linux ++ darwin;
      # mainProgram = "e-imzo";
      maintainers = [
        {
          name = "Sokhibjon Orzikulov";
          email = "sakhib@orzklv.uz";
          handle = "orzklv";
          github = "orzklv";
          githubId = 54666588;
          keys = [
            {
              fingerprint = "00D2 7BC6 8707 0683 FBB9  137C 3C35 D3AF 0DA1 D6A8";
            }
          ];
        }
      ];
    };
  }
