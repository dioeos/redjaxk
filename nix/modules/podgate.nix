{ config, lib, ... }:

let
  cfg = config.services.redjaxk-podgate;
in
{
  options.services.redjaxk-podgate = {
    enable = lib.mkEnableOption "Redjaxk-Podgate";

    package = lib.mkOption {
      type = lib.types.package;
      description = "Package containing redjaxk-podgate";
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = [
      cfg.package
    ];

    systemd.user.services.redjaxk-podgate = {
      Unit.Description = "Redjaxk Podgate";
      Service = {
        ExecStart = "${cfg.package}/bin/redjaxk-podgate";
        Restart = "on-failure";

        Install.WantedBy = [ "default.target" ];
      };
    };
  };
}
