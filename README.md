# RITOS


For proposal please refer to [here](https://github.com/ExtraSecond/proposal).
For weekly meeting notes please refer to [notes](https://github.com/ExtraSecond/weekly-meeting).


## Install suitable rust toolchain ##


```bash
curl https://sh.rustup.rs -sSf             \
    |                                      \
    sh -s --                               \
    --default-toolchain nightly-2019-12-20 \
    --component rust-src llvm-tools-preview rustfmt rls rust-analysis

source $HOME/.cargo/env
cargo install cargo-xbuild cargo-binutils
```

## Firmware for Raspberry Pi 3 ##

The official firmware for RPi 3 B+:

- [bootcode.bin](https://github.com/raspberrypi/firmware/raw/master/boot/bootcode.bin)
- [fixup.dat](https://github.com/raspberrypi/firmware/raw/master/boot/fixup.dat)
- [start.elf](https://github.com/raspberrypi/firmware/raw/master/boot/start.elf)

## Boot on Raspberry Pi 3 ##

1. Create an `FAT32` partition named `boot` and then copy the files above and `kernel8.img` to the microSD card. And create a `config.txt` file with contents:

```
init_uart_clock=48000000
```

2. Insert the microSD card to RPi 3 and connect the USB series to host PC.

3. Run `screen` on the host PC
```bash
sudo screen /dev/ttyUSB0 230400
```
## Chainboot ##

`make` before `demo_payload_rpi3.img` become useful. Then you can `make chainboot`. Need ruby runtime environment and `colorize`, `ruby-progressbar`, `serialport` gem packages.

## GPIO and PWM driver ##

1. Use `gpio().setup(pin, direction, pull)` to set up GPIO pins and clear the output bit respectively.

2. Write or read the status of GPIO pins by using `gpio().output(pin, value)` and `gpio().input(pin)`