flake: {
  config,
  lib,
  pkgs,
  ...
}: let
  # Shortcuts
  cfg = config.services.e-imzo;
  pkg = flake.packages.${pkgs.stdenv.hostPlatform.system}.default;

  # Single name for everything
  name = "e-imzo";

  args = {cfg}: let
    id =
      if cfg.id-card
      then "--id-card"
      else "";
  in
    lib.strings.concatStringsSep " " [id];

  # Systemd service
  service = lib.mkIf cfg.enable {
    users.users.${name} = {
      description = "E-IMZO service daemon user";
      isSystemUser = true;
      group = name;
    };

    users.groups.${name} = {};

    systemd.user.services.${name} = {
      enable = true;
      description = "E-IMZO, uzbek state web signing service";
      documentation = ["https://github.com/xinux-org/e-imzo"];

      after = ["network-online.target" "graphical.target"];
      wants = ["network-online.target" "graphical.target"];
      wantedBy = ["default.target"];

      serviceConfig = {
        Type = "simple";
        # User = cfg.user;
        # Group = cfg.group;
        Restart = "always";
        ExecStart = "${lib.getBin cfg.package}/bin/e-imzo ${args {inherit cfg;}}";
        # StateDirectory = cfg.user;
        # StateDirectoryMode = "0750";

        # Hardening
        CapabilityBoundingSet = [
          "AF_NETLINK"
          "AF_INET"
          "AF_INET6"
        ];
        DeviceAllow = ["/dev/stdin r"];
        DevicePolicy = "strict";
        IPAddressAllow = "localhost";
        LockPersonality = true;
        # MemoryDenyWriteExecute = true;
        NoNewPrivileges = true;
        PrivateDevices = true;
        PrivateTmp = true;
        PrivateUsers = true;
        ProtectClock = true;
        ProtectControlGroups = true;
        ProtectHome = true;
        ProtectHostname = true;
        ProtectKernelLogs = true;
        ProtectKernelModules = true;
        ProtectKernelTunables = true;
        ProtectSystem = "strict";
        ReadOnlyPaths = ["/"];
        RemoveIPC = true;
        RestrictAddressFamilies = [
          "AF_NETLINK"
          "AF_INET"
          "AF_INET6"
        ];
        RestrictNamespaces = true;
        RestrictRealtime = true;
        RestrictSUIDSGID = true;
        SystemCallArchitectures = "native";
        SystemCallFilter = [
          "@system-service"
          "~@privileged"
          "~@resources"
          "@pkey"
        ];
        UMask = "0027";
      };

      # preStart = ''
      #   if [ ! -d /media ]; then
      #     mkdir -p /media;
      #   fi
      # '';
    };
  };
in {
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

  config = lib.mkMerge [service];
}
