# RasPiRus_OS

### Building

Build command:
 
*   Arm: `cargo build --target target_configs/arm-raspirus_os.json`
*   x86_64: `cargo build --target target_configs/x86_64-raspirus_os.json`


Rust version:  
`rustc 1.52.0-nightly (94736c434 2021-02-27)`

target:  
`rustup target add armv7r-none-eabihf`

gcc:  
`arm-none-eabi-gcc`