use crate::binding::{
    blake2b_constant_BLAKE2B_KEYBYTES, blake2b_constant_BLAKE2B_OUTBYTES,
    blake2b_constant_BLAKE2B_PERSONALBYTES, blake2b_constant_BLAKE2B_SALTBYTES, blake2b_final,
    blake2b_init_key_with_param, blake2b_init_param, blake2b_param, blake2b_state, blake2b_update,
};
use core::ffi::c_void;
use core::mem::MaybeUninit;

pub struct Blake2b {
    pub(crate) state: MaybeUninit<blake2b_state>,
}

impl Blake2b {
    pub fn uninit() -> Self {
        Blake2b {
            state: MaybeUninit::<blake2b_state>::uninit(),
        }
    }
}

pub struct Blake2bBuilder {
    pub(crate) param: blake2b_param,
    pub(crate) key_len: usize,
    pub(crate) key: [u8; blake2b_constant_BLAKE2B_KEYBYTES as usize],
}

impl Blake2bBuilder {
    /// In reality, most projects only use one or two blake2b hashers,
    /// which means there is not need to rebuild Blake2bBuilder again
    /// and again. This const function allows one to create a Rust
    /// constant builder, and use it across the project. Right now we
    /// only provide new_with_personal variant, since this is widely
    /// used in Nervos CKB.
    pub const fn new_with_personal(
        out_len: usize,
        personal: [u8; blake2b_constant_BLAKE2B_PERSONALBYTES as usize],
    ) -> Self {
        assert!(out_len >= 1 && out_len <= blake2b_constant_BLAKE2B_OUTBYTES as usize);
        let param = blake2b_param {
            digest_length: out_len as u8,
            key_length: 0,
            fanout: 1,
            depth: 1,
            leaf_length: 0,
            node_offset: 0,
            xof_length: 0,
            node_depth: 0,
            inner_length: 0,
            reserved: [0u8; 14usize],
            salt: [0u8; blake2b_constant_BLAKE2B_SALTBYTES as usize],
            personal,
        };

        let key_len = 0;
        let key = [0u8; blake2b_constant_BLAKE2B_KEYBYTES as usize];

        Blake2bBuilder {
            param,
            key_len,
            key,
        }
    }

    pub fn new(out_len: usize) -> Self {
        Self::new_with_personal(
            out_len,
            [0u8; blake2b_constant_BLAKE2B_PERSONALBYTES as usize],
        )
    }

    pub fn salt(mut self, salt: &[u8]) -> Blake2bBuilder {
        let len = salt.len();
        assert!(len <= blake2b_constant_BLAKE2B_SALTBYTES as usize);

        unsafe {
            ::core::ptr::copy_nonoverlapping(salt.as_ptr(), self.param.salt.as_mut_ptr(), len);
        }
        self
    }

    pub fn personal(mut self, personal: &[u8]) -> Blake2bBuilder {
        let len = personal.len();
        assert!(len <= blake2b_constant_BLAKE2B_PERSONALBYTES as usize);

        unsafe {
            ::core::ptr::copy_nonoverlapping(
                personal.as_ptr(),
                self.param.personal.as_mut_ptr(),
                len,
            );
        }
        self
    }

    pub fn key(mut self, key: &[u8]) -> Blake2bBuilder {
        let key_len = key.len();
        assert!(key_len <= blake2b_constant_BLAKE2B_KEYBYTES as usize);
        self.param.key_length = key_len as u8;
        self.key_len = key_len;

        unsafe {
            ::core::ptr::copy_nonoverlapping(key.as_ptr(), self.key.as_mut_ptr(), key_len);
        }
        self
    }

    pub fn build(self) -> Blake2b {
        let mut blake2b = Blake2b::uninit();
        self.build_from_ref(&mut blake2b);
        blake2b
    }

    /// Combined with new_with_personal, this can use the same builder
    /// to efficiently generate multiple different hashers.
    pub fn build_from_ref(&self, blake2b: &mut Blake2b) {
        if self.key_len == 0 {
            unsafe {
                blake2b_init_param(
                    blake2b.state.as_mut_ptr(),
                    &self.param as *const blake2b_param,
                );
            }
        } else {
            unsafe {
                blake2b_init_key_with_param(
                    blake2b.state.as_mut_ptr(),
                    &self.param as *const blake2b_param,
                    self.key.as_ptr() as *const c_void,
                    self.key_len,
                );
            }
        }
    }
}

impl Blake2b {
    pub fn update(&mut self, data: &[u8]) {
        unsafe {
            blake2b_update(
                self.state.as_mut_ptr(),
                data.as_ptr() as *const c_void,
                data.len(),
            );
        }
    }

    pub fn finalize(mut self, dst: &mut [u8]) {
        self.finalize_from_ref(dst);
    }

    /// This is the same as finalize, but allows one to call the function
    /// via a mutable reference, which can result in better performance
    /// if one can manually control the times this function is invoked.
    pub fn finalize_from_ref(&mut self, dst: &mut [u8]) {
        unsafe {
            blake2b_final(
                self.state.as_mut_ptr(),
                dst.as_mut_ptr() as *mut c_void,
                dst.len(),
            );
        }
    }
}

pub fn blake2b(key: &[u8], data: &[u8], dst: &mut [u8]) {
    let mut blake2b = Blake2bBuilder::new(dst.len()).key(key).build();
    blake2b.update(data);
    blake2b.finalize(dst)
}

#[cfg(test)]
mod tests {
    use super::Blake2bBuilder;
    use faster_hex::{hex_decode, hex_string};
    use serde_derive::Deserialize;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;
    use std::string::String;
    use std::vec;
    use std::vec::Vec;

    #[derive(Deserialize, Debug)]
    struct TestItem {
        outlen: usize,
        out: String,
        input: String,
        key: String,
        salt: String,
        personal: String,
    }

    #[test]
    fn test_full() {
        let test_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/test.json");

        let f = File::open(test_path).unwrap();
        let reader = BufReader::new(f);
        let tests: Vec<TestItem> = serde_json::from_reader(reader).unwrap();

        for test in tests {
            let mut hash = vec![0u8; test.outlen];
            let mut blake2b = Blake2bBuilder::new(test.outlen)
                .key(&unhex(test.key.as_bytes()))
                .personal(&unhex(test.personal.as_bytes()))
                .salt(&unhex(test.salt.as_bytes()))
                .build();
            blake2b.update(&unhex(test.input.as_bytes()));
            blake2b.finalize(&mut hash);
            assert_eq!(hex_string(&hash), test.out);
        }
    }

    fn unhex(src: &[u8]) -> Vec<u8> {
        let len = src.len() / 2;
        let mut ret = vec![0u8; len];
        if !src.is_empty() {
            hex_decode(src, &mut ret).unwrap();
        }
        ret
    }
}
