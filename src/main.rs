#![feature(asm)]
#![feature(global_asm)]
#![no_std]  // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod bsp;
mod cpu;
mod panic_wait;