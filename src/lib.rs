#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[allow(clippy::all)]
mod binding;
mod blake2b;

pub use crate::blake2b::{blake2b, Blake2b, Blake2bBuilder};
