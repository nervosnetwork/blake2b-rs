#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate blake2b_rs;

fuzz_target!(|data: &[u8]| {
    let mut hash = [0u8; 64];
    blake2b_rs::blake2b(&[], data, &mut hash);
});
