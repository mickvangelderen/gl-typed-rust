//! The param traits are unsafe because their associated type Value will be cast
//! to a pointer of some other type.

use crate::enums;
use crate::symbols;

pub unsafe trait TransmuteMarker<T> {}

unsafe impl<T> TransmuteMarker<T> for T {}

//Not symmetric
pub trait Transmute<T> {
    fn from(t: T) -> Self;
    fn into(self) -> T;
    fn as_ref(&self) -> &T;
    fn as_mut(&mut self) -> &mut T;
}

/// A reimplementation of the `transmute` function, avoiding problems
/// when the compiler can't prove equal sizes.
#[inline]
#[doc(hidden)]
pub unsafe fn transmute<A, B>(a: A) -> B {
    let b = ::core::ptr::read(&a as *const A as *const B);
    ::core::mem::forget(a);
    b
}


impl<M, T> Transmute<T> for M
where
    M: TransmuteMarker<T> + Sized,
    T: Sized,
{
    #[inline]
    fn from(t: T) -> Self {
        unsafe { transmute(t) }
    }

    #[inline]
    fn into(self) -> T {
        unsafe { transmute(self) }
    }

    #[inline]
    fn as_ref(&self) -> &T {
        unsafe { &*(self as *const Self as *const T) }
    }

    #[inline]
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *(self as *mut Self as *mut T) }
    }
}

pub unsafe trait GetShaderivParam: Into<enums::GetShaderivParam> {
    type Value: Transmute<i32>;
}

unsafe impl GetShaderivParam for symbols::CompileStatus {
    type Value = enums::RawShaderCompileStatus;
}

unsafe impl GetShaderivParam for symbols::InfoLogLength {
    type Value = i32;
}

pub unsafe trait GetProgramivParam: Into<enums::GetProgramivParam> {
    type Value: Transmute<i32>;
}

unsafe impl GetProgramivParam for symbols::LinkStatus {
    type Value = enums::RawProgramLinkStatus;
}

unsafe impl GetProgramivParam for symbols::InfoLogLength {
    type Value = i32;
}
