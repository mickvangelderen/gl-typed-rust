use crate::num::NonMinusOneI32;
use std::marker::PhantomData;

#[repr(transparent)]
pub struct AttributeLocation(NonMinusOneI32);

impl AttributeLocation {
    #[inline]
    pub fn from_raw(raw: i32) -> Option<Self> {
        NonMinusOneI32::new(raw).map(AttributeLocation)
    }

    #[inline]
    pub const unsafe fn from_raw_unchecked(raw: i32) -> Self {
        AttributeLocation(NonMinusOneI32::new_unchecked(raw))
    }

    #[inline]
    pub fn as_u32(&self) -> u32 {
        self.0.get() as u32
    }
}

#[repr(transparent)]
pub struct UniformLocation<T>(NonMinusOneI32, PhantomData<*const T>);

impl<T> UniformLocation<T> {
    #[inline]
    pub fn from_raw(raw: i32) -> Option<Self> {
        NonMinusOneI32::new(raw).map(|loc| UniformLocation(loc, PhantomData))
    }

    #[inline]
    pub const unsafe fn from_raw_unchecked(raw: i32) -> Self {
        UniformLocation(NonMinusOneI32::new_unchecked(raw), PhantomData)
    }

    #[inline]
    pub fn as_i32(&self) -> i32 {
        self.0.get()
    }
}
