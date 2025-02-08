flake: {
  config,
  lib,
  pkgs,
  ...
}: let
  # Shortcuts
  cfg = config.services.e-imzo;
  pkg = flake.packages.${pkgs.stdenv.hostPlatform.system}.default;

  args = {cfg}: let
    id =
      if cfg.id-card
      then "--id-card"
      else "";
  in
    lib.strings.concatStringsSep " " [id];

  # Systemd service
  service = lib.mkIf cfg.enable {
    users.users.${cfg.user} = {
      description = "E-IMZO service daemon user";
      isSystemUser = true;
      group = cfg.group;
    };

    users.groups.${cfg.group} = {};

    systemd.services.e-imzo = {
      description = "E-IMZO, uzbek state web signing service";
      documentation = ["https://github.com/xinux-org/e-imzo"];

      after = ["network-online.target"];
      wants = ["network-online.target"];
      wantedBy = ["multi-user.target"];

      serviceConfig = {
        User = cfg.user;
        Group = cfg.group;
        Restart = "always";
        ExecStart = "${lib.getBin cfg.package}/bin/e-imzo ${args {inherit cfg;}}";
        StateDirectory = "/media";
        StateDirectoryMode = "0750";

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
        MemoryDenyWriteExecute = true;
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

      user = mkOption {
        type = types.str;
        default = "e-imzo";
        description = "User for running service + accessing keys";
      };

      group = mkOption {
        type = types.str;
        default = "e-imzo";
        description = "Group for running service + accessing keys";
      };

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
