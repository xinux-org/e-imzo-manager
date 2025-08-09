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

## Building the project

Make sure you have `flatpak` and `flatpak-builder` installed. Then run the commands below. Replace `<application_id>` with the value you entered during project creation. Please note that these commands are just for demonstration purposes. Normally this would be handled by your IDE, such as GNOME Builder or VS Code with the Flatpak extension.

```shell
flatpak install --user org.gnome.Sdk//47 org.gnome.Platform//47  org.freedesktop.Sdk.Extension.rust-stable//23.08 org.freedesktop.Sdk.Extension.llvm18
flatpak-builder --user flatpak_app build-aux/<application_id>.Devel.json
```

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
