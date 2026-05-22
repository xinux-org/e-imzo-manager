# Bootstrap everythign e-imzo needs
#
# Legacy nix-build
# {
#   lib,
#   writeShellApplication,
#   ncurses,
# }
#
# Flake only support
{
  pkgs ? import <nixpkgs> { },
  ...
}:
let
  lib = pkgs.lib;
in
(pkgs.writeShellApplication {
  name = "e-helper";
  runtimeInputs = with pkgs; [
    ncurses
    xdg-utils
  ];
  text = builtins.readFile ./e-helper.sh;
})
// {
  meta = with lib; {
    homepage = "https://github.com/xinux-org/e-imzo";
    description = "E-IMZO initial bootstrapper for further progression.";
    licenses = licenses.mit;
    platforms = platforms.all;
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
