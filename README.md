# libpkgbuild

A library for building AcaciaLinux packages from packagebuilds implementing the [packagebuild spec](https://wiki.acacialinux.org/pkgbuild).

# Directories

libpkgbuild operates in its root as specified in its configuration and does not write anything outside that.

There are some subdirectories that are used:

- `environments` This directory contains subdirectories that contain the root filesystems for the different build environments.

- `cache` This directory contains all the cached files, such as the `leaf` cache.
  
  - `leaf` The directory where leaf can cache its files.
  
  - `overlay_work` The `work` directory used by `overlayfs`.
  
  - `overlay_upper` The `upper` directory used by `overlayfs`.

- `build` The directory that gets composed by the current environment and additional packages installed using `overlayfs`. This is the root where the package is built.

- `target` The target directory used to prepare packages. This will contain the raw package directories to install into. This gets mapped into the build directory using `bind` mounts.

## Mounts

libpkgbuild will construct two main mounts for the build process:

`/build` and `/target/<package name>`.

#### /build

This mount is of type `overlay` that will be composed of the following subdirectories:

- `lower`: `environments/<environment>` using the selected environment for the build.

- `work`: `cache/overlay_work` - Gets disposed after build.

- `upper`: `cache/overlay_upper` - Gets disposed after build: The builder is only interested in the resulting destdir.

#### /target/[package_name]

This is the package directory that the install script installs into. It contains the `data` directory. It gets mounted into the `build` directory and exposed by the builder under the `PKG_ROOT` environment variable. The builder has full authority on where this directory resides, but it will normally be mounted at `/target` and the variable will then be constructed from that.