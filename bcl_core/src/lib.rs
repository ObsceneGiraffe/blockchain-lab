//! Bitcoin Lab

// coding conventions
#![forbid(unsafe_code)]
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(dead_code)]
#![deny(unused_imports)]
#![deny(missing_docs)]

#[cfg(target_pointer_width = "16")]
compile_error!("rust-bitcoin cannot be used on 16-bit architectures");

pub mod blockdata;
mod mem;
pub mod num;

pub use blockdata::block::Block;
pub use blockdata::block::BlockHeader;
