# Flatpak
Instructions regarding Flatpak build and deployment.

## Dependencies
- `org.gnome.Platform`
- `org.freedesktop.Sdk.Extension.rust-stable`

> The current version of `org.gnome.Platform` is 45.

Install the following dependencies:
```
$ flatpak install --runtime org.gnome.Platform org.freedesktop.Sdk.Extension.rust-stable
```

## Build

#### Development
To build the development version of the app for Flatpak:
```bash
$ flatpak-builder flatpak_build ./build-aux/org.xinux.EIMZOManager.Devel.json
```

#### Release
To build the release version of the app for Flatpak:
```bash
$ flatpak-builder flatpak_build ./build-aux/org.xinux.EIMZOManager.json
```

## Test the build
To verify that the build was successful, run the following:

#### Development
```bash
$ flatpak-builder --user --install --force-clean flatpak_build ./build-aux/org.xinux.EIMZOManager.Devel.json
$ flatpak run org.xinux.EIMZOManager.Devel.json
```

#### Release
```bash
$ flatpak-builder --user --install --force-clean flatpak_build ./build-aux/org.xinux.EIMZOManager.json
$ flatpak run org.xinux.EIMZOManager.json
```

## Release to Flathub
To make a release to Flathub, run [`flatpak.sh`](scripts/flatpak.sh), take the files and upload them to the new release. 

Once they are uploaded, edit [`org.xinux.EIMZOManager.json`](https://github.com/flathub/dev.edfloreshz.Done/blob/master/dev.edfloreshz.Done.json) and replace the `url` of the `source` with the new link of the `tar.xz` file uploaded to the release.

Remember to replace `hash` with a newly generated hash for the `tar.xz` file:

```
$ sha256sum done-release.tar.xz
```

```json
"sources" : [
    {
        "type" : "archive",
        "url" : "https://github.com/done-devs/done/releases/download/version/done-release.tar.xz", // New download url
        "sha256" : "dcb976ea39287790728399151a9c30926e242a01fa9c68f13ff1d95b48fb2b1f" // New hash
    }
]
```

Then, push changes to https://github.com/flathub/dev.edfloreshz.Done.

## Development
```
# do not run it inside nix-shell
nix run github:xinux-org/e-imzo

export GTK_DEBUG=interactive

meson setup build
meson compile -C build
./build/src/gtk-rust-template

# generate translation words from /po/POTFILES.in
xgettext --directory=.. --files-from=POTFILES.in --from-code=UTF-8 -kgettext -o messages.pot
```


# GTK + Rust + Relm4 + Meson + Flatpak = <3

> This is a fork of [gtk-rust-template](https://gitlab.gnome.org/World/Rust/gtk-rust-template) that adapts the code for Relm4 while trying to change as little as possible.

A boilerplate template to get started with GTK, Rust, Meson, Flatpak made for GNOME. It can be adapted for other desktop environments like elementary.

<div align="center">

![Main window](data/resources/screenshots/screenshot1.png "Main window")
</div>

## Building the project

Make sure you have `flatpak` and `flatpak-builder` installed. Then run the commands below. Replace `<application_id>` with the value you entered during project creation. Please note that these commands are just for demonstration purposes. Normally this would be handled by your IDE, such as GNOME Builder or VS Code with the Flatpak extension.

```shell
flatpak install --user org.gnome.Sdk//47 org.gnome.Platform//47  org.freedesktop.Sdk.Extension.rust-stable//23.08 org.freedesktop.Sdk.Extension.llvm18
flatpak-builder --user flatpak_app build-aux/<application_id>.Devel.json
```

## Running the project

Once the project is build, run the command below. Replace `<application_id>` and `<project_name>` with the values you entered during project creation. Please note that these commands are just for demonstration purposes. Normally this would be handled by your IDE, such as GNOME Builder or VS Code with the Flatpak extension.

```shell
flatpak-builder --run flatpak_app build-aux/<application_id>.Devel.json <project_name>
```

## Translations with Gettext

The template uses `gettext` as a framework for translations using [`gettext-rs`](https://github.com/gettext-rs/gettext-rs). The basic files for this can be found in the `po` folder.
While meson will take care of building the translations the extraction and translation itself has to be done manually.

### Extracting translatable strings

First of all you have to have `gettext` installed on your system. With that you then are able to use `xgettext` as following to extract the translatable strings:

```shell
xgettext --package-name=<project_name> --package-version=main --msgid-bugs-address=https://github.com/<project_name>/<project_name>/issues --files-from=po/POTFILES.in --output=po/<project_name>.pot
```

Note that you might need to update the `po/POTFILES.in` file to reflect the files of your process. This describes where `xgettext` is going to search for strings to translate.

### Translating the translatable strings

To translate the strings you need to use po files. Tools like Poedit allow you to generate these from the `po/<project_name>.pot` file.
It also allows you to sync the `po/<project_name>.pot` when you rerun `xgettext`.

When adding a po file also make sure to add the language code to `po/LINGUAS`.

## Community

Join the GNOME and gtk-rs community!

- [Matrix chat](https://matrix.to/#/#rust:gnome.org): chat with other developers using gtk-rs
- [Discourse forum](https://discourse.gnome.org/tag/rust): topics tagged with `rust` on the GNOME forum.
- [GNOME circle](https://circle.gnome.org/): take inspiration from applications and libraries already extending the GNOME ecosystem.

## Credits

- [Podcasts](https://gitlab.gnome.org/World/podcasts)
- [Shortwave](https://gitlab.gnome.org/World/Shortwave)
