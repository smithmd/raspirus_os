# RasPiRus_OS

### Building

Build command:

Default `cargo build` target is arm for raspberry pi.
 
*   arm: `cargo build --target target_configs/arm-raspirus_os.json`
*   x86_64: `cargo build --target target_configs/x86_64-raspirus_os.json`

Strip symbols and create kernel img file:
`rust-objcopy --strip-all -O binary target/arm-raspirus_os/release/raspirus_os kernel8.img`

Rust version:  
`rustc 1.52.0-nightly (94736c434 2021-02-27)`

`nightly` is required for unstable build options. If this changes, I can update to a release version in the `rust-toolchain` file.

target:  
`rustup target add armv7r-none-eabihf`

gcc:  
`arm-none-eabi-gcc`



### QEMU

`qemu-system-aarch64 -M raspi3 -d in_asm  -kernel kernel8.img`

###### Note: QEMU only supports up to raspberry pi 3

##### Notes

Got a working target json by running the following:  
`rustc -Z unstable-options --print target-spec-json --target armv7r-none-eabihf`

This dumps a json config for that target arch which I could copy.