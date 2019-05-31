use crate::binding::*;

#[test]
fn bindgen_test_layout___fsid_t() {
    assert_eq!(
        ::std::mem::size_of::<__fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__fsid_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__fsid_t>())).__val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__fsid_t),
            "::",
            stringify!(__val)
        )
    );
}

#[test]
fn bindgen_test_layout_max_align_t() {
    assert_eq!(
        ::std::mem::size_of::<max_align_t>(),
        32usize,
        concat!("Size of: ", stringify!(max_align_t))
    );
    assert_eq!(
        ::std::mem::align_of::<max_align_t>(),
        16usize,
        concat!("Alignment of ", stringify!(max_align_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<max_align_t>())).__clang_max_align_nonce1 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<max_align_t>())).__clang_max_align_nonce2 as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce2)
        )
    );
}

#[test]
fn bindgen_test_layout_blake2b_state__() {
    assert_eq!(
        ::std::mem::size_of::<blake2b_state__>(),
        248usize,
        concat!("Size of: ", stringify!(blake2b_state__))
    );
    assert_eq!(
        ::std::mem::align_of::<blake2b_state__>(),
        8usize,
        concat!("Alignment of ", stringify!(blake2b_state__))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<blake2b_state__>())).h as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(blake2b_state__),
            "::",
            stringify!(h)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<blake2b_state__>())).t as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(blake2b_state__),
            "::",
            stringify!(t)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<blake2b_state__>())).f as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(blake2b_state__),
            "::",
            stringify!(f)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<blake2b_state__>())).buf as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(blake2b_state__),
            "::",
            stringify!(buf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<blake2b_state__>())).buflen as *const _ as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(blake2b_state__),
            "::",
            stringify!(buflen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<blake2b_state__>())).outlen as *const _ as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(blake2b_state__),
            "::",
            stringify!(outlen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<blake2b_state__>())).last_node as *const _ as usize },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(blake2b_state__),
            "::",
            stringify!(last_node)
        )
    );
}

#[test]
fn bindgen_test_layout_blake2b_param__() {
    assert_eq!(
        ::std::mem::size_of::<blake2b_param__>(),
        64usize,
        concat!("Size of: ", stringify!(blake2b_param__))
    );
    assert_eq!(
        ::std::mem::align_of::<blake2b_param__>(),
        1usize,
        concat!("Alignment of ", stringify!(blake2b_param__))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<blake2b_param__>())).digest_length as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(blake2b_param__),
            "::",
            stringify!(digest_length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<blake2b_param__>())).key_length as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(blake2b_param__),
            "::",
            stringify!(key_length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<blake2b_param__>())).fanout as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(blake2b_param__),
            "::",
            stringify!(fanout)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<blake2b_param__>())).depth as *const _ as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(blake2b_param__),
            "::",
            stringify!(depth)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<blake2b_param__>())).leaf_length as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(blake2b_param__),
            "::",
            stringify!(leaf_length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<blake2b_param__>())).node_offset as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(blake2b_param__),
            "::",
            stringify!(node_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<blake2b_param__>())).xof_length as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(blake2b_param__),
            "::",
            stringify!(xof_length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<blake2b_param__>())).node_depth as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(blake2b_param__),
            "::",
            stringify!(node_depth)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<blake2b_param__>())).inner_length as *const _ as usize },
        17usize,
        concat!(
            "Offset of field: ",
            stringify!(blake2b_param__),
            "::",
            stringify!(inner_length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<blake2b_param__>())).reserved as *const _ as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(blake2b_param__),
            "::",
            stringify!(reserved)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<blake2b_param__>())).salt as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(blake2b_param__),
            "::",
            stringify!(salt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<blake2b_param__>())).personal as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(blake2b_param__),
            "::",
            stringify!(personal)
        )
    );
}
