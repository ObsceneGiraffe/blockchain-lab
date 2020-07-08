//! A set of memory management helpers

// use std::convert::AsMut;

// pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
//     ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
// }

// pub fn clone_into_array<A, T>(slice: &[T]) -> A
// where
//     A: Sized + Default + AsMut<[T]>,
//     T: Clone,
// {
//     let mut a = Default::default();
//     <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
//     a
// }

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_uint_construction() {
        // let expected_uint = 0x40302010_u32;
        // let split = [0x4030_u16.to_be(), 0x2010_u16.to_be()];
        // let byte_arr = unsafe { any_as_u8_slice(&split) };
        // let byte_arr_fixed: [u8; 4] = clone_into_array(byte_arr);
        // let uint = u32::from_be_bytes(byte_arr_fixed);
        // assert_eq!(byte_arr, [0x40, 0x30, 0x20, 0x10]);
        // assert_eq!(uint, expected_uint);
    }
}
