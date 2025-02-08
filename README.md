# E-IMZO

Documentation is still WIP, patience my friends!!!

## Options

```nix
# In your flake.nix
{
  inputs.e-imzo.url = "github:xinux-org/e-imzo";
}

# Somewhere in your nix configs
{
  imports = [inputs.e-imzo.nixosModules.e-imzo];

  # Here are available options
  services.e-imzo = {
    # Enable Toggle
    # => Mandatory
    enable = true;

    # ID Card support (experimental)
    # => Optional
    id-card = false;

    # User for launching service
    # => Optional
    user = "negir";

    # Group of user for launching service
    # => Optional
    group = "negirlar";

    # E-IMZO custom package
    # => Optional
    package = pkgs.<?>;
  };
}
```
