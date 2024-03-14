{
  description = "Basic rust template";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane = { url = "github:ipetkov/crane"; inputs.nixpkgs.follows = "nixpkgs"; };
    fenix = { url = "github:nix-community/fenix"; inputs.nixpkgs.follows = "nixpkgs"; inputs.rust-analyzer-src.follows = ""; };
  };

  outputs = inputs @ { self, nixpkgs, crane, fenix, ... }:
    let
      forAllSystems = nixpkgs.lib.genAttrs [ "x86_64-linux" ];
      nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; });
    in
    {
      packages = forAllSystems (system: {
        oscuro =
          let
            pkgs = nixpkgsFor.${system};
            cranelib = crane.lib.${system};
          in
          cranelib.buildPackage {
            src = cranelib.cleanCargoSource (cranelib.path ./.);
            strictDeps = true;

            buildInputs = [ ];
          };

        default = self.packages.${system}.oscuro;
      });

      checks = forAllSystems (system: {
        inherit (self.packages.${system}.oscuro);

        oscuro-fmt = let cranelib = crane.lib.${system}; in
          cranelib.cargoFmt {
            src = cranelib.cleanCargoSource (cranelib.path ./.);
          };
      });

      apps = forAllSystems (system: {
        default = {
          type = "app";
          program = "${self.packages.${system}.oscuro}/bin/oscuro";
        };
      });

      devShells = forAllSystems (system: {
        default = let pkgs = nixpkgsFor.${system}; in pkgs.mkShell {
          nativeBuildInputs = [ fenix.packages.${system}.complete.toolchain pkgs.cargo-shuttle ];
        };
      });
    };

}
