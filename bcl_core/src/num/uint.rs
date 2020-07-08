//! Bitcoin uses a unsigned 265 integer as to store hashes

// use crate::mem;
// use std::ptr;

/// the byte len of a 256 uint
pub const U256_BYTE_LEN: usize = 256 / 8;

/// An unsigned 256 bit integer
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct U256(pub [u8; U256_BYTE_LEN]);

impl U256 {
    /// the length in bytes of a unsigned 256 bit integer
    pub const fn byte_len() -> usize {
        U256_BYTE_LEN
    }

    // pub fn from_u128_array(byte_arr: [u128; 2]) -> U256 {
    //     let byte_arr_be = [byte_arr[0].to_be(), byte_arr[1].to_be()];
    //     let byte_slice = unsafe { mem::any_as_u8_slice(&byte_arr_be) };
    //     U256(mem::clone_into_array(byte_slice))
    // }

    // pub fn from_u64_array(byte_arr: [u64; 4]) -> U256 {
    //     let byte_arr_be = [
    //         byte_arr[0].to_be(),
    //         byte_arr[1].to_be(),
    //         byte_arr[2].to_be(),
    //         byte_arr[3].to_be(),
    //     ];
    //     let byte_slice = unsafe { mem::any_as_u8_slice(&byte_arr_be) };
    //     U256(mem::clone_into_array(byte_slice))
    // }

    // pub fn from_u8_slice(slice: &[u8]) -> U256 {
    //     let mut result: U256 = Default::default();
    //     unsafe {
    //         ptr::copy_nonoverlapping(slice.as_ptr(), result.as_mut_ptr(), U256_BYTE_LEN);
    //     }
    //     result
    // }

    /// get this int as a mutable byte pointer
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self as *mut U256 as *mut u8
    }
}
