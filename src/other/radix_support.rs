use std::ops::{BitAnd, BitOrAssign, Shl, Shr};

/// A fixed-width converted key. Only unsigned integer primitives implement this.
pub trait Unsigned:
    sealed::Sealed
    + Copy
    + Ord
    + BitAnd<Output = Self>
    + BitOrAssign
    + Shl<Output = Self>
    + Shr<Output = Self>
{
    const BITS: usize;
    fn from_usize(value: usize) -> Self;
    fn to_usize(self) -> usize;
    fn to_u128(self) -> u128;
}

mod sealed {
    pub trait Sealed {}
}

macro_rules! unsigned {
    ($($ty:ty),* $(,)?) => {$(
        impl sealed::Sealed for $ty {}
        impl Unsigned for $ty {
            const BITS: usize = <$ty>::BITS as usize;
            #[inline]
            fn from_usize(value: usize) -> Self { value as Self }
            #[inline]
            fn to_usize(self) -> usize { self as usize }
            #[inline]
            fn to_u128(self) -> u128 { self as u128 }
        }
        impl Key for $ty {
            type Value = Self;
            #[inline]
            fn key(&self) -> Self { *self }
        }
    )*};
}

/// A record sortable by a converted unsigned key; payloads need not be Copy or Clone.
///
/// `key` must return a deterministic key matching the record's `Ord`, including
/// equality. The backend preserves input order for equal converted keys.
/// If extraction panics, the panic resumes and the order is unspecified. For a
/// record with a destructor the adapters also restore the slice to a
/// permutation of the original records, so nothing is lost or dropped twice;
/// records without a destructor provide memory safety only, which is why they
/// skip the recovery bookkeeping.
pub trait Key {
    type Value: Unsigned;
    const SUPPORTED: bool = true;
    fn key(&self) -> Self::Value;
}

unsigned!(u8, u16, u32, u64, u128, usize);

macro_rules! signed {
    ($(($signed:ty, $unsigned:ty)),* $(,)?) => {$(
        impl Key for $signed {
            type Value = $unsigned;
            #[inline]
            fn key(&self) -> Self::Value {
                (*self as $unsigned) ^ (1 << (<$unsigned>::BITS - 1))
            }
        }
    )*};
}

signed!(
    (i8, u8),
    (i16, u16),
    (i32, u32),
    (i64, u64),
    (i128, u128),
    (isize, usize)
);

impl Key for bool {
    type Value = u8;
    fn key(&self) -> u8 {
        u8::from(*self)
    }
}

impl Key for char {
    type Value = u32;
    fn key(&self) -> u32 {
        *self as u32
    }
}

impl Key for () {
    type Value = u8;
    fn key(&self) -> u8 {
        0
    }
}

// Preserve the original converted float ordering, which agrees with total_cmp.
// Focused backend tests sort primitive floats directly; the harness's generic
// sort entry point still requires Ord.
macro_rules! float {
    ($(($float:ty, $unsigned:ty)),* $(,)?) => {$(
        impl Key for $float {
            type Value = $unsigned;
            #[inline]
            fn key(&self) -> Self::Value {
                let bits = self.to_bits();
                let sign = 1 << (<$unsigned>::BITS - 1);
                if bits & sign != 0 { !bits } else { bits ^ sign }
            }
        }
    )*};
}

float!((f32, u32), (f64, u64));

impl Key for sort_test_tools::ffi_types::F128 {
    type Value = u64;
    fn key(&self) -> u64 {
        self.sort_key().key()
    }
}

impl<T: Key + Copy> Key for std::cell::Cell<T> {
    type Value = T::Value;
    const SUPPORTED: bool = T::SUPPORTED;
    fn key(&self) -> Self::Value {
        self.get().key()
    }
}
impl Key for sort_test_tools::ffi_types::FFIOneKibiByte {
    type Value = u64;
    fn key(&self) -> u64 {
        self.sort_key().key()
    }
}

/// Matches partial_cmp on non-NaN values, including equal signed zeros.
pub fn partial_f32_key(value: f32) -> u32 {
    if value == 0.0 {
        0.0f32.key()
    } else {
        value.key()
    }
}
pub fn partial_f64_key(value: f64) -> u64 {
    if value == 0.0 {
        0.0f64.key()
    } else {
        value.key()
    }
}

impl<A: Key, B: Key> Key for (A, B) {
    type Value = u128;
    const SUPPORTED: bool = A::SUPPORTED && B::SUPPORTED && A::Value::BITS + B::Value::BITS <= 128;
    fn key(&self) -> u128 {
        assert!(Self::SUPPORTED, "combined radix key exceeds 128 bits");
        (self.0.key().to_u128() << B::Value::BITS) | self.1.key().to_u128()
    }
}

// All direct adapters use the same key contract and capability check. Unsupported
// operations remain explicit; benchmark registration can omit unsupported types.
macro_rules! radix_adapter {
    ($name:expr, $algorithm:expr) => {
        sort_impl!(
            $name,
            fn supports_type<T>() -> bool
            where
                T: Ord,
            {
                <T as RadixDispatch>::supported()
            },
            fn supports_sort_by() -> bool {
                false
            }
        );
        trait RadixDispatch: Sized {
            fn supported() -> bool;
            fn sort(data: &mut [Self]);
        }
        impl<T> RadixDispatch for T {
            default fn supported() -> bool {
                false
            }
            default fn sort(_data: &mut [Self]) {
                panic!("Type not supported");
            }
        }
        impl<T: crate::other::radix_support::Key> RadixDispatch for T {
            fn supported() -> bool {
                <T as crate::other::radix_support::Key>::SUPPORTED
            }
            fn sort(data: &mut [Self]) {
                sort_keyed(data);
            }
        }
        /// Sort explicitly keyed records; payloads need no Ord/Clone/Copy impl.
        pub fn sort_keyed<T: crate::other::radix_support::Key>(data: &mut [T]) {
            assert!(T::SUPPORTED, "Type not supported");
            ($algorithm)(data);
        }
        pub fn supports_type<T>() -> bool {
            <T as RadixDispatch>::supported()
        }
        pub fn sort<T: Ord>(data: &mut [T]) {
            <T as RadixDispatch>::sort(data);
        }
        pub fn sort_by<T, F: FnMut(&T, &T) -> std::cmp::Ordering>(_data: &mut [T], _compare: F) {
            panic!("sort_by not supported");
        }
    };
}
