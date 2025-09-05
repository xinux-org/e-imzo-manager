<p align="center">
    <img src=".github/assets/header.png" alt="Xinux'es {E-IMZO Manager}">
</p>

<p align="center">
    <h3 align="center">E-IMZO Key Management Application written on Rust/GTK.</h3>
</p>

<p align="center">
    <img align="center" src="https://img.shields.io/github/languages/top/xinux-org/e-imzo?style=flat&logo=nixos&logoColor=5277C3&labelColor=ffffff&color=ffffff" alt="Top Used Language">
    <a href="https://github.com/xinux-org/e-imzo/actions/workflows/test.yml"><img align="center" src="https://img.shields.io/github/actions/workflow/status/uzinfocom-org/instances/test.yml?style=flat&logo=github&logoColor=5277C3&labelColor=ffffff&color=ffffff" alt="Test CI"></a>
</p>

## About

Since NixOS'es nixpkgs has e-imzo service available under the hood, there's no helper or managemental application for e-imzo's key management. We created that very key manager that will help users to easily add, remove and see more details about their key.

## Development

This application has Linux-only dependencies.
```bash
# download dependencies
nix develop 

# Initiate meson environment for the first time. This will generate ./src/config.rs
meson setup build

# build the project
nix build --show--trace

# Go one folder back and run bin outside of nix-shell. Otherwise polkit cannot ask password to create /media/DSKEYS for e-imzo server
cd ..
./e-imzo/result/bin/E-IMZO-Manager

# Optional. Generate translation words from /po/POTFILES.in if needed.
cd ./po
xgettext --directory=.. --files-from=POTFILES.in --from-code=UTF-8 -kgettext -o translations.pot
```

## Building

Building this app requires Linux hosts only with nix. If you are going to only try or use this application, simply run:

```bash
# Call the flake via nix. Do not run it inside nix-shell!
nix run github:xinux-org/e-imzo
```

However, if you were planning to tweak this application or hack, follow the [development](#development) section of this document for more, it includes build steps for development environment.

## Installation
```nix
# add these to your config files or /etc/nixos/configuration.nix 
environment = {
  systemPackages = with pkgs; [
    e-imzo-manager
  ];
};

service.e-imzo.enable = true;
```
## FAQ

These are most frequently asked question for developers and hackers.

### Why not use default.nix for devShell?

There's been cases when we wanted to reproduce totally different behaviors in development environment and production build. This occurs quite a lot lately for some reason and because of that, we tend to keep both shell.nix and default.nix to don't mix things up.

### Will this application work on Windows and MacOS?

Linux is the first-class citizen and we don't primarilary support these platforms as our main target. However, as GTK supports these platforms and rust can compile to both of them, it's possible to tweak this application and make it work for these platforms.

### What about publishing this app to the Flathub?

Our main target stays on NixOS and nix package manager. App hosted on nixpkgs and porting to flathub requires some code change. That means we are open to help mainteiners who really wants to see E-IMZO Manager on another Linux distros. The software on flathub will be community supported because the software developed by Uzinfocom OSS developers is only supported in nixpkgs and does not take responsibility for the version on flathub.

### Flathub development docs?
Refer to [Flatpak.md](./Flatpak.md)
## License

This project is licensed under the CC-BY-4.0 for text documents at [archive](.github/archive) license due to stricted use of [Soliq.uz](https://soliq.uz)'es policy and AGPL for the manager - see the [LICENSE-CCBY](LICENSE-CCBY) and [LICENSE-AGPL](LICENSE-AGPL) files for details.

<p align="center">
    <img src="./.github/assets/footer.png" alt="Xinux'es {E-IMZO Manager}">
</p>
