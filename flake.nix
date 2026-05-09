{
  description = "A beginning of an awesome project bootstrapped with github:bleur-org/templates";

  inputs = {
    nixpkgs.url = "git+https://git.oss.uzinfocom.uz/xinux/nixpkgs?ref=nixos-unstable&shallow=1";

    xinux-lib = {
      url = "git+https://git.oss.uzinfocom.uz/xinux/lib?ref=main&shallow=1";
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
