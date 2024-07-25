{
  description = "Oscuro is a fancy multibot";

  nixConfig = {
    extra-substituters = [
      "https://cache.elnafo.ru"
      "https://bonfire.cachix.org"
    ];
    extra-trusted-public-keys = [
      "cache.elnafo.ru:j3VD+Hn+is2Qk3lPXDSdPwHJQSatizk7V82iJ2RP1yo="
      "bonfire.cachix.org-1:mzAGBy/Crdf8NhKail5ciK7ZrGRbPJJobW6TwFb7WYM="
    ];
  };

  inputs = {
    bonfire = {
      url = "github:L-Nafaryus/bonfire";
      inputs = {
        oscuro.follows = "";
      };
    };
    nixpkgs.follows = "bonfire/nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    bonfire,
    ...
  }: let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
    lib = pkgs.lib;
    fenixPkgs = bonfire.inputs.fenix.packages.x86_64-linux;
    craneLib = (bonfire.inputs.crane.mkLib pkgs).overrideToolchain fenixPkgs.complete.toolchain;
  in {
    packages.x86_64-linux = rec {
      oscuro = let
        common = {
          pname = "oscuro";
          version = "0.1.0";

          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = path: type: (craneLib.filterCargoSources path type);
          };

          strictDeps = true;

          nativeBuildInputs = [pkgs.pkg-config];

          buildInputs = [pkgs.openssl];
        };

        cargoArtifacts = craneLib.buildDepsOnly common;
      in
        craneLib.buildPackage (common // {inherit cargoArtifacts;});

      default = oscuro;
    };

    hydraJobs = {
      packages = self.packages;
    };

    devShells.x86_64-linux.default = pkgs.mkShell {
      nativeBuildInputs = [pkgs.pkg-config];
      buildInputs = [
        fenixPkgs.complete.toolchain
        pkgs.cargo-release
        pkgs.openssl
      ];
      LD_LIBRARY_PATH = lib.makeLibraryPath [pkgs.openssl];
    };

    nixosModules = rec {
      oscuro = {
        config,
        lib,
        pkgs,
        ...
      }:
        with lib; let
          cfg = config.services.oscuro;
          opt = options.services.oscuro;
          pkg = self.packages.${pkgs.system}.oscuro;
          configFile = pkgs.writeText "config.toml" ''
            discord_token = "#discord_token#"
          '';
        in {
          options.services.oscuro = {
            enable = mkEnableOption "Enables the Oscuro bot";

            package = mkPackageOption pkgs "oscuro" {};

            dataDir = mkOption {
              type = types.path;
              default = "/var/lib/oscuro";
              description = lib.mdDoc "Directory to store Oscuro files";
            };

            discordToken = mkOption {
              type = types.nullOr types.str;
              default = null;
              example = "Bot TOKENTOKENTOKEN";
            };

            discordTokenFile = mkOption {
              type = types.nullOr types.str;
              default = null;
              example = "/var/lib/secrets/oscuro/discord_token";
            };
          };

          config = mkIf cfg.enable {
            assertions = [
              {
                assertion = cfg.discordToken != null || cfg.discordTokenFile != null;
                message = "Discord token must be set. Use `services.oscuro.discordToken` or `services.oscuro.discordTokenFile`.";
              }
            ];

            users.users.oscuro = {
              description = "Oscuro bot service user";
              home = cfg.dataDir;
              createHome = true;
              isSystemUser = true;
              group = "oscuro";
            };
            users.groups.oscuro = {};

            systemd.services.oscuro = {
              description = "Oscuro";
              wantedBy = ["multi-user.target"];
              after = ["network.target"];

              serviceConfig = {
                Restart = "always";
                ExecStart = "${pkg}/bin/oscuro";
                User = "oscuro";
                WorkingDirectory = cfg.dataDir;
              };

              preStart = let
                runConfig = "${cfg.dataDir}/config.toml";
                replaceSecret = "${pkgs.replace-secret}/bin/replace-secret";
              in ''
                cp -f '${configFile}' '${runConfig}'
                chmod u+w '${runConfig}'

                ${lib.optionalString (cfg.discordTokenFile != null) ''
                  ${replaceSecret} '#discord_token#' '${cfg.discordTokenFile}' '${runConfig}'
                ''}
                ${lib.optionalString (cfg.discordToken != null) ''
                  sed -i 's/#discord_token#/${cfg.discordToken}/g' '${runConfig}'
                ''}

              '';
            };
          };
        };

      default = oscuro;
    };

    nixosConfigurations.oscuro = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        self.nixosModules.oscuro
        ({pkgs, ...}: {
          boot.isContainer = true;

          networking.hostName = "oscuro";
          networking.useDHCP = false;

          services.oscuro = {
            enable = true;
            discordToken = ""; # insert token
          };

          system.stateVersion = "24.05";
        })
      ];
    };
  };
}
