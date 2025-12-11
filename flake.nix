{
  description = "A beginning of an awesome project bootstrapped with github:bleur-org/templates";

  inputs = {
    # Stable for keeping thins clean
    # # Fresh and new for testing
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

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
    })
    // {
      # Hydra CI jobs
      hydraJobs = {
        packages = self.packages.x86_64-linux.default;
      };
    };
}
