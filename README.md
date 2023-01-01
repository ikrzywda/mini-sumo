# Mini sumo robot firmware

## Rust project template for rp pico

https://github.com/rp-rs/rp2040-project-template


## General setup

* Make sure all rust sdk is installed in `.cargo` directory - otherwise provided target
    might not work
* GDB && OpenOcd setup: https://kresna.dev/getting-picoprobe-openocd-gdb-to-work-with-rust-on-mac-m1/
    - TL;DR:
```bash
    brew tap armmbed/homebrew-formulae
    brew install arm-none-eabi-gcc
    brew install libtool automake libusb wget pkg-config gcc texinfo

    export PATH="/opt/homebrew/opt/texinfo/bin:$PATH"
    git clone https://github.com/raspberrypi/openocd.git --branch rp2040 --depth=1
    # inside cloned directory:
    ./bootstrap
    ./configure --enable-picoprobe --disable-werror
    make -j4   # if failing make sure texinfo PATH is exported
    sudo make install  # to install openocd
```

* Rp Pico debug wiring && picoprobe firmware:
    - https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf (page 64)
    - https://datasheets.raspberrypi.com/soft/picoprobe.uf2

* Rust project deps: https://github.com/rp-rs/rp2040-project-template
    - TL;DR:
```bash
    rustup target add thumbv6m-none-eabi
    cargo install flip-link
    cargo install probe-run
    cargo install elf2uf2-rs --locked
```

## Usage

### Initiation

* `scripts/start-openocd.sh` to start openocd session (first terminal)
* `scripts/run.sh` start gdb, connect to remote session (set to port `3333`) and 
    enable semihosting

### Quickstart in gdb

* `load` - load binary
* `continue` - start the program


## Misc error handling

> **failing to connect / upload to target**
 make sure target pico is powered (either by usb or `VSYS` from debug pico)
