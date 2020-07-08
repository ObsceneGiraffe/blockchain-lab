//! Tests for the block module

// use bcl_core::blockdata::block;
// use bcl_core::num::uint;

#[test]
fn test_difficulty_le_extraction() {
    let bits = 0x1b0404cb_u32;
    let bits_le = bits.to_le();
    let head = (bits_le >> 24) & 0xff;
    let tail = bits_le & 0x00ffffff;
    assert_eq!(head, 0x1b);
    assert_eq!(tail, 0x0404cb);
}

#[test]
fn test_difficulty() {
    // For example, if the packed target in the block is 0x1b0404cb (stored in little-endian order: cb 04 04 1b), the hexadecimal target is
    // 0x0404cb * 2**(8*(0x1b - 3)) = 0x00000000000404CB000000000000000000000000000000000000000000000000
    // let res = block::difficulty(0x1b0404cb);
    // let byte_arr = [
    //     0x00000000000404CB0000000000000000_u128,
    //     0x00000000000000000000000000000000_u128,
    // ];
    // assert_eq!(res, uint::U256::from_u128_array(byte_arr));
}
