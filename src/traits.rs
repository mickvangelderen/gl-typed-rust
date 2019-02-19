//! The param traits are unsafe because their associated type Value will be cast
//! to a pointer of some other type.

use crate::symbols;
use crate::types;

pub unsafe trait TransmuteMarker<T> {}

unsafe impl<T> TransmuteMarker<T> for T {}

unsafe impl TransmuteMarker<u32> for i32 {}
unsafe impl TransmuteMarker<i32> for u32 {}

pub trait FromExt<T> {
    fn from(t: T) -> Self;
}

pub trait IntoExt<T> {
    fn into(self) -> T;
}

impl<T> FromExt<T> for T {
    #[inline]
    fn from(t: Self) -> Self {
        t
    }
}

impl<Old, New> IntoExt<New> for Old
where
    New: FromExt<Old>,
{
    #[inline]
    fn into(self) -> New {
        FromExt::from(self)
    }
}

impl FromExt<u32> for i32 {
    #[inline]
    fn from(n: u32) -> i32 {
        n as i32
    }
}

impl FromExt<i32> for u32 {
    #[inline]
    fn from(n: i32) -> u32 {
        n as u32
    }
}

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

pub unsafe trait GetShaderivParam: Into<types::GetShaderivParam> {
    type Value: Transmute<i32>;
}

unsafe impl GetShaderivParam for symbols::CompileStatus {
    type Value = types::RawShaderCompileStatus;
}

unsafe impl GetShaderivParam for symbols::InfoLogLength {
    type Value = i32;
}

pub unsafe trait GetProgramivParam: Into<types::GetProgramivParam> {
    type Value: Transmute<i32>;
}

unsafe impl GetProgramivParam for symbols::LinkStatus {
    type Value = types::RawProgramLinkStatus;
}

unsafe impl GetProgramivParam for symbols::InfoLogLength {
    type Value = i32;
}

pub unsafe trait TexParameteriParam: Into<types::TexParameteriParam> {
    type Target: Into<types::TextureTarget>;
    type Value: IntoExt<i32>;
}

unsafe impl TexParameteriParam for symbols::DepthStencilTextureMode {
    type Target = types::TextureTarget;
    type Value = types::DepthStencilTextureMode;
}

unsafe impl TexParameteriParam for symbols::TextureBaseLevel {
    type Target = types::TextureTarget;
    type Value = u32;
}

unsafe impl TexParameteriParam for symbols::TextureMagFilter {
    type Target = types::TextureTarget;
    type Value = types::TextureMagFilter;
}

unsafe impl TexParameteriParam for symbols::TextureMinFilter {
    type Target = types::TextureTarget;
    type Value = types::TextureMinFilter;
}

unsafe impl TexParameteriParam for symbols::TextureWrapS {
    type Target = types::TextureTarget;
    type Value = types::TextureWrap;
}

unsafe impl TexParameteriParam for symbols::TextureWrapT {
    type Target = types::TextureTargetGE2D;
    type Value = types::TextureWrap;
}

unsafe impl TexParameteriParam for symbols::TextureWrapR {
    type Target = types::TextureTargetGE3D;
    type Value = types::TextureWrap;
}
