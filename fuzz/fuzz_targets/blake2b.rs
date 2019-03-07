#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate blake2b;

fuzz_target!(|data: &[u8]| {
    let mut hash = [0u8; 64];
    blake2b::blake2b(&[], data, &mut hash);
});
