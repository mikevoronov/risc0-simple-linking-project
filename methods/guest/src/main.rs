#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental


use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

extern "C" { 
    fn addxx(a: i32, b :i32) -> i32;
    //fn randomx_get_flags() -> u32;
}

#[no_mangle]
pub extern "C" fn _sbrk(shift: u32) -> u32 {
    0
}

pub fn main() {
    let result = unsafe { addxx(1, 2) };
    println!("{}", result);


    env::commit(&result);
}
