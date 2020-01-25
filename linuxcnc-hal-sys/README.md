# LinuxCNC HAL Rust bindings

[![CircleCI](https://circleci.com/gh/jamwaffles/linuxcnc-hal-rs.svg?style=shield)](https://circleci.com/gh/jamwaffles/linuxcnc-hal-rs)
[![Crates.io](https://img.shields.io/crates/v/linuxcnc-hal-sys.svg)](https://crates.io/crates/linuxcnc-hal-sys)
[![Docs.rs](https://docs.rs/linuxcnc-hal-sys/badge.svg)](https://docs.rs/linuxcnc-hal-sys)

Provides **non-realtime** Rust bindings for the LinuxCNC `hal` module. Useful for writing drivers for external hardware.

> Note: This only works on Linux (64 bit) currently

## Development

### Setup

[`bindgen`](https://github.com/rust-lang/rust-bindgen) must be set up correctly. Follow the [requirements section of its docs](https://rust-lang.github.io/rust-bindgen/requirements.html).

LinuxCNC is included as a submodule under `./linuxcnc-hal-sys`. It must be compiled for files to be in the right places.

At minimum (on Linux Mint 19.3):

```bash
apt install \
    bwidget \
    intltool \
    kmod \
    libboost-python-dev \
    libglu-dev \
    libgtk2.0-dev \
    libmodbus-dev \
    libtk-img \
    libudev-dev \
    libusb-1.0-0-dev \
    libx11-dev \
    libxinerama-dev \
    libxmu-dev \
    mesa-common-dev \
    python \
    python-tk \
    tclx \
    tk-dev \
    yapps2

cd linuxcnc/src

./autogen.sh

./configure \
  --with-realtime=uspace \
  --enable-non-distributable=yes \
  --disable-userspace-pci \
  --disable-check-runtime-deps

make -j $(nproc)
```

There are likely a bunch of unused dependencies in the above list, but LinuxCNC's `configure` script is really bad at turning features on/off, so this is the list required to just build LCNC.

### Build

```bash
cargo build
```
