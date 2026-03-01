{
  description = "A beginning of an awesome project bootstrapped with github:bleur-org/templates";

  inputs = {
    # Stable for keeping thins clean
    # # Fresh and new for testing
    nixpkgs.url = "github:xinux-org/nixpkgs/nixos-unstable";

    crane.url = "github:ipetkov/crane";

    xinux-lib = {
      url = "github:xinux-org/lib/main";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.xinux-lib.mkFlake {
      inherit inputs;
      alias.packages.default = "e-imzo-manager";
      alias.shells.default = "e-imzo-manager";
      src = ./.;
      hydraJobs = inputs.self.packages.x86_64-linux;
    };
}
