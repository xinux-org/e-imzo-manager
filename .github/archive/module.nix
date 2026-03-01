flake:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  # Shortcuts
  cfg = config.services.e-imzo;
  pkg = flake.packages.${pkgs.stdenv.hostPlatform.system}.default;

  # Single name for everything
  name = "e-imzo";

  args =
    { cfg }:
    let
      id = if cfg.id-card then "--id-card" else "";
    in
    lib.strings.concatStringsSep " " [ id ];

  # Systemd service
  service = lib.mkIf cfg.enable {
    users.users.${name} = {
      description = "E-IMZO service daemon user";
      isSystemUser = true;
      group = name;
    };

    users.groups.${name} = { };

    systemd.user.services.${name} = {
      enable = true;
      description = "E-IMZO, uzbek state web signing service";
      documentation = [ "https://github.com/xinux-org/e-imzo" ];

      after = [
        "network-online.target"
        "graphical.target"
      ];
      wants = [
        "network-online.target"
        "graphical.target"
      ];
      wantedBy = [ "default.target" ];

      serviceConfig = {
        Type = "simple";
        Restart = "always";
        RestartSec = 1;
        ExecStart = "${lib.getBin cfg.package}/bin/e-imzo ${args { inherit cfg; }}";

        # Hardening
        NoNewPrivileges = true;
        SystemCallArchitectures = "native";
      };
    };
  };
in
{
  options = with lib; {
    services.e-imzo = {
      enable = mkEnableOption ''
        Enable E-IMZO background service.
      '';

      id-card = mkOption {
        type = with types; bool;
        default = false;
        description = lib.mdDoc ''
          Enable ID-CARD support for E-IMZO service
        '';
      };

      # user = mkOption {
      #   type = types.str;
      #   default = "e-imzo";
      #   description = "User for running service + accessing keys";
      # };

      # group = mkOption {
      #   type = types.str;
      #   default = "e-imzo";
      #   description = "Group for running service + accessing keys";
      # };

      package = mkOption {
        type = types.package;
        default = pkg;
        description = ''
          E-IMZO package to use with the service.
        '';
      };
    };
  };

  config = lib.mkMerge [ service ];
}
