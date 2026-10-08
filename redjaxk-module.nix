{ config, lib, ... }:

let
  cfg = config.services.redjaxk-agent;
in
{
  options.services.redjaxk-agent = {
    enable = lib.mkEnableOption "Redjaxk-Agent";

    package = lib.mkOption {
      type = lib.types.package;
      description = "Package containing redjaxk-agent";
    };

    environment = lib.mkOption {
      type = lib.types.attrsOf lib.types.str;
      default = { };
      description = "Environment variables for redjaxk-agnent";
    };
  };

  config = lib.mkIf cfg.enable {
    assertions = [
      {
        assertion = cfg.environment ? REDJAXX_NODE_ID && cfg.environment.REDJAXX_NODE_ID != "";
        message = "services.redjaxk-agent.environment.REDJAXK_NODE_ID must be set.";
      }
    ];
    home.packages = [
      cfg.package
    ];

    systemd.user.services.redjaxk-agent = {
      Unit.Description = "Redjaxk Agent";
      Service = {
        ExecStart = "${cfg.package}/bin/redjaxk-agent";
        Restart = "on-failure";

        Environment = lib.mapAttrsToList (name: value: "${name}=${value}") cfg.environment;
      };
      Install.WantedBy = [ "default.target" ];
    };
  };
}
