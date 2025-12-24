{
  description = "A beginning of an awesome project bootstrapped with github:bleur-org/templates";

  inputs = {
    # # Fresh and new for testing
    # nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";

    # Stable for keeping thins clean
    nixpkgs.url = "github:xinux-org/nixpkgs/nixos-25.11";

    crane.url = "github:ipetkov/crane";

    # The flake-utils library
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    crane,
    ...
  }:
  # @ inputs
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in rec {
      # Nix script formatter
      formatter = pkgs.alejandra;

      # Development environment
      devShells.default = import ./shell.nix {inherit pkgs;};

      # Output package
      packages.default = import ./. {inherit crane pkgs;};
      
      # When you need hash for new release
      # packages.default = pkgs.callPackage ./package.nix {};
    })
    // {
      # Hydra CI jobs
      hydraJobs = {
        packages = self.packages.x86_64-linux.default;
      };
    };
}
