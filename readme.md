# RasPiRus_OS

### Building

Build command:

Default `cargo build` target is arm for raspberry pi.
 
*   arm: `cargo build --target target_configs/arm-raspirus_os.json`
*   x86_64: `cargo build --target target_configs/x86_64-raspirus_os.json`


Rust version:  
`rustc 1.52.0-nightly (94736c434 2021-02-27)`

`nightly` is required for unstable build options. If this changes, I can update to a release version in the `rust-toolchain` file.

target:  
`rustup target add armv7r-none-eabihf`

gcc:  
`arm-none-eabi-gcc`


##### Notes

Got a working target json by running the following:  
`rustc -Z unstable-options --print target-spec-json --target armv7r-none-eabihf`

This dumps a json config for that target arch which I could copy.