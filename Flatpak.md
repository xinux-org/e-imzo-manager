# Flatpak
Instructions regarding Flatpak build and deployment.

## Dependencies
- `flatpak`
- `flatpak-builder`
- `org.gnome.Platform`
- `org.freedesktop.Sdk.Extension.rust-stable`

> The current version of `org.gnome.Platform` is 48.

Install the following dependencies:
```shell
flatpak install --runtime org.gnome.Platform org.freedesktop.Sdk.Extension.rust-stable

# write --system instead if you want for all system users
flatpak install --user org.gnome.Sdk//47 org.gnome.Platform//47
```

#### Development
To build the development version of the app for Flatpak:
```bash
$ flatpak-builder flatpak_build ./build-aux/uz.xinux.EIMZOManager.Devel.json --force-clean
```

#### Release
To build the release version of the app for Flatpak:
```bash
$ flatpak-builder flatpak_build ./build-aux/uz.xinux.EIMZOManager.json --force-clean
```

## Test the build
To verify that the build was successful, run the following:

#### Development
```bash
$ flatpak-builder --user --install --force-clean flatpak_build ./build-aux/uz.xinux.EIMZOManager.Devel.json
$ flatpak run uz.xinux.EIMZOManager.Devel.json
```

#### Release
```bash
$ flatpak-builder --user --install --force-clean flatpak_build ./build-aux/uz.xinux.EIMZOManager.json
$ flatpak run uz.xinux.EIMZOManager.json
```

## Release to Flathub
To make a release to Flathub, run [`flatpak.sh`](scripts/flatpak.sh), take the files and upload them to the new release. 

Once they are uploaded, edit [`uz.xinux.EIMZOManager.json`](https://github.com/flathub/dev.edfloreshz.Done/blob/master/dev.edfloreshz.Done.json) and replace the `url` of the `source` with the new link of the `tar.xz` file uploaded to the release.

Remember to replace `hash` with a newly generated hash for the `tar.xz` file:

```
$ sha256sum done-release.tar.xz
```

```json
"sources" : [
    {
        "type" : "archive",
        "url" : "https://github.com/xinux-org/e-imzo/releases/download/version/done-release.tar.xz", // New download url
        "sha256" : "dcb976ea39287790728399151a9c30926e242a01fa9c68f13ff1d95b48fb2b1f" // New hash
    }
]
```

Then, push changes to https://github.com/flathub/uz.xinux.EIMZOManager.
