// NOTE(ZERO): We want Option<NonMinusOneI32> and friends to not take extra
// space. In the future we will be able to specify -1 as the None value:
// https://github.com/rust-lang/rust/issues/49137#issuecomment-408202688. For
// now we remap the sentinel value to 0 so we can use NonZeroU32.

use std::fmt;
use std::num::NonZeroU32;

// Taken from https://doc.rust-lang.org/beta/src/core/num/mod.rs.html#12
macro_rules! impl_nonzero_fmt {
    ( ( $( $Trait: ident ),+ ) for $Ty: ident ) => {
        $(
            impl fmt::$Trait for $Ty {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    self.get().fmt(f)
                }
            }
        )+
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct NonMinusOneI32(NonZeroU32);

impl NonMinusOneI32 {
    /// Create a non-minus-one without checking the value.
    ///
    /// # Safety
    ///
    /// The value must not be minus one.
    #[inline]
    pub const unsafe fn new_unchecked(n: i32) -> Self {
        NonMinusOneI32(NonZeroU32::new_unchecked(n.wrapping_add(1) as u32))
    }

    /// Create a non-minus-one if the given value is not minus one.
    #[inline]
    pub fn new(n: i32) -> Option<Self> {
        NonZeroU32::new(n.wrapping_add(1) as u32).map(NonMinusOneI32)
    }

    /// Returns the value as a primitive type.
    #[inline]
    pub fn get(self) -> i32 {
        (self.0.get() as i32).wrapping_sub(1)
    }
}

impl_nonzero_fmt! {
    (Debug, Display, Binary, Octal, LowerHex, UpperHex) for NonMinusOneI32
}

#[allow(non_camel_case_types)]
pub union b32 {
    u: u32,
    i: i32,
}

impl From<u32> for b32 {
    fn from(val: u32) -> Self {
        b32 { u: val }
    }
}

impl From<b32> for u32 {
    fn from(val: b32) -> Self {
        unsafe {
            val.u
        }
    }
}

impl From<i32> for b32 {
    fn from(val: i32) -> Self {
        b32 { i: val }
    }
}

impl From<b32> for i32 {
    fn from(val: b32) -> Self {
        unsafe {
            val.i
        }
    }
}
