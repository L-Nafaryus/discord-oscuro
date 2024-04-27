{
    description = "Oscuro - a fancy discord bot";

    nixConfig = {
        extra-substituters = [ "https://bonfire.cachix.org" ];
        extra-trusted-public-keys = [ "bonfire.cachix.org-1:mzAGBy/Crdf8NhKail5ciK7ZrGRbPJJobW6TwFb7WYM=" ];
    };

    inputs = {
        bonfire = {
            url = "github:L-Nafaryus/bonfire";
        };
    };

    outputs = { self, bonfire, ... }:
    let
        nixpkgs = bonfire.inputs.nixpkgs;
        forAllSystems = nixpkgs.lib.genAttrs [ "x86_64-linux" ];
        nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; });
    in
    {
        packages = forAllSystems (system: 
        let 
            pkgs = nixpkgsFor.${system};
            crane-lib = bonfire.inputs.crane.lib.${system};

            src = pkgs.lib.cleanSourceWith {
                src = ./.;
                filter = path: type: (crane-lib.filterCargoSources path type);
            };

            common = {
                inherit src;
                pname = "oscuro";
                version = "0.1.0";
                strictDeps = true;
            };

            cargoArtifacts = crane-lib.buildDepsOnly common;
        in {
            oscuro = crane-lib.buildPackage (common // { inherit cargoArtifacts; });

            default = self.packages.${system}.oscuro;
        });

        devShells = forAllSystems (system: 
        let 
            pkgs = nixpkgsFor.${system};
            bonfire-pkgs = bonfire.packages.${system};
            fenix-pkgs = bonfire.inputs.fenix.packages.${system};
            crane-lib = bonfire.inputs.crane.lib.${system};
        in {
            default = pkgs.mkShell {
                buildInputs = [ 
                    fenix-pkgs.complete.toolchain 
                    bonfire-pkgs.cargo-shuttle 
                    pkgs.cargo-release
                ];
            };
        });

        nixosModules = {
            oscuro = { config, lib, pkgs, ... }:
            with lib;
            let 
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
                        wantedBy = [ "multi-user.target" ];
                        after = [ "network.target" ];

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

            default = self.nixosModules.oscuro;
        };

        nixosConfigurations.oscuro = nixpkgs.lib.nixosSystem {
            description = "Oscuro";
            system = "x86_64-linux";
            modules = [
                self.nixosModules.oscuro
                ({ pkgs, ... }: {
                    boot.isContainer = true;

                    networking.hostName = "oscuro";
                    networking.useDHCP = false;

                    services.oscuro = {
                        enable = true;
                        discordToken = null;
                    };

                    system.stateVersion = "24.05";
                })
            ];
        };
    };

}
