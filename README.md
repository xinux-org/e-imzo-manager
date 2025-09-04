<p align="center">
    <img src=".github/assets/header.png" alt="Xinux'es {E-IMZO Manager}">
</p>

<p align="center">
    <h3 align="center">E-IMZO Key Management Application written on Rust/GTK.</h3>
</p>

<p align="center">
    <img align="center" src="https://img.shields.io/github/languages/top/xinux-org/e-imzo?style=flat&logo=nixos&logoColor=ffffff&labelColor=242424&color=242424" alt="Top Used Language">
    <a href="https://github.com/xinux-org/e-imzo/actions/workflows/test.yml"><img align="center" src="https://img.shields.io/github/actions/workflow/status/uzinfocom-org/instances/test.yml?style=flat&logo=github&logoColor=ffffff&labelColor=242424&color=242424" alt="Test CI"></a>
</p>

## About

Since NixOS'es nixpkgs has e-imzo service available under the hood, there's no helper or managemental application for e-imzo's key management. We created that very key manager that will help users to easily add, remove and see more details about their key.

## Development

This application has Linux-only dependencies that would require additional flags to run on MacOS with nix package manager.

```
# Do not run it inside nix-shell!
nix run github:xinux-org/e-imzo

# Open interactive GTK Debugger
export GTK_DEBUG=interactive

# Initiate meson environment
meson setup build
meson compile -C build
./build/src/E-IMZO-Manager

# Generate translation words from /po/POTFILES.in
xgettext --directory=.. --files-from=POTFILES.in --from-code=UTF-8 -kgettext -o translations.pot
```

## Building

Building this app requires Linux hosts only. If you are going to only try or use this application, simply run:

```bash
# Call the flake via nix
nix run github:xinux-org/e-imzo
```

However, if you were planning to tweak this application or hack, follow the [development](#development) section of this document for more, it includes build steps for development environment.

## Installation

_WIP_

## License

This project is licensed under the CC-BY-4.0 for text documents at [archive](.github/archive) license due to stricted use of [Soliq.uz](https://soliq.uz)'es policy and AGPL for the manager - see the [LICENSE-CCBY](LICENSE-CCBY) and [LICENSE-AGPL](LICENSE-AGPL) files for details.

<p align="center">
    <img src="./.github/assets/footer.png" alt="Xinux'es {E-IMZO Manager}">
</p>
