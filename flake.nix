{
  description = "E-IMZO client for NixOS";

  inputs = {
    # Too old to work with most libraries
    # nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";

    # Perfect!
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    # The flake-utils library
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};

        packages = {
          default = packages.e-imzo;
          e-imzo = pkgs.callPackage ./. {inherit pkgs;};
          e-helper = pkgs.callPackage ./helper {inherit pkgs;};
        };
      in {
        # Output package
        inherit packages;

        # Nix script formatter
        formatter = pkgs.alejandra;

        # Development environment
        devShells.default = import ./shell.nix {inherit pkgs;};
      }
    )
    // {
      # Overlay module
      nixosModules.e-imzo = import ./module.nix self;
    };
}
