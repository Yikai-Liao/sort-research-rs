#![allow(dead_code, unused_macros)]
#[path = "../vendor/radix_support.rs"]
pub mod radix_support;
mod key { pub use crate::radix_support::{Key, Unsigned}; }
#[path = "../vendor/block_core.rs"]
pub mod block_core;
pub mod other {
    pub use crate::radix_support;
    pub mod rust_block_lsd { pub use crate::block_core as core; }
}
#[path = "../vendor/tessera.rs"]
mod tessera;

use radix_support::Key;

macro_rules! integer_row {
    ($name:ident,$ty:ty,$unsigned:ty,$func:ident) => {
        #[repr(C)]
        pub struct $name { value: $ty, index: u32 }
        impl Key for $name {
            type Value = $unsigned;
            #[inline] fn key(&self) -> $unsigned { self.value.key() }
        }
        #[no_mangle]
        pub unsafe extern "C" fn $func(data: *mut $name, len: usize) {
            tessera::sort(std::slice::from_raw_parts_mut(data,len));
        }
    };
}
integer_row!(I32Row,i32,u32,tessera_i32);
integer_row!(I64Row,i64,u64,tessera_i64);
integer_row!(I128Row,i128,u128,tessera_i128);

macro_rules! float_row {
    ($name:ident,$ty:ty,$unsigned:ty,$func:ident) => {
        #[repr(C)]
        pub struct $name { value: $ty, index: u32 }
        impl Key for $name {
            type Value = $unsigned;
            #[inline] fn key(&self) -> $unsigned {
                // Match StarRocks comparator: NaNs and signed zeros equal +0.
                let v=if self.value.is_nan() || self.value==0.0 {0.0} else {self.value};
                v.key()
            }
        }
        #[no_mangle]
        pub unsafe extern "C" fn $func(data: *mut $name, len: usize) {
            tessera::sort(std::slice::from_raw_parts_mut(data,len));
        }
    };
}
float_row!(F32Row,f32,u32,tessera_f32);
float_row!(F64Row,f64,u64,tessera_f64);

#[no_mangle]
pub unsafe extern "C" fn tessera_u64(data: *mut u64,len:usize) {
    tessera::sort(std::slice::from_raw_parts_mut(data,len));
}
