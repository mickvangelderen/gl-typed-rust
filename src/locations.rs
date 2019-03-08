use crate::convert::*;

/// Guaranteed to be in range 0..i32::MAX.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct AttributeLocation(u32);

impl AttributeLocation {
    #[inline]
    pub fn new(val: i32) -> Option<Self> {
        if val >= 0 {
            Some(AttributeLocation(val as u32))
        } else {
            None
        }
    }

    /// You must guarantee val is in range 0..i32::MAX.
    pub unsafe fn new_unchecked(val: i32) -> Self {
        AttributeLocation(val as u32)
    }

    #[inline]
    pub fn into_u32(self) -> u32 {
        self.0
    }
}

/// A more compact representation of Option<AttributeLocation>.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct OptionAttributeLocation(pub i32);

impl OptionAttributeLocation {
    #[inline]
    pub fn new(val: i32) -> Self {
        OptionAttributeLocation(val)
    }

    #[inline]
    pub fn into_i32(self) -> i32 {
        self.0
    }

    #[inline]
    fn is_some(&self) -> bool {
        Option::<AttributeLocation>::from(*self).is_some()
    }
}

impl From<OptionAttributeLocation> for Option<AttributeLocation> {
    #[inline]
    fn from(val: OptionAttributeLocation) -> Self {
        AttributeLocation::new(val.into_i32())
    }
}

/// Guaranteed to be in range 0..i32::MAX.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct UniformLocation(i32);

impl UniformLocation {
    #[inline]
    pub fn new(val: i32) -> Option<Self> {
        if val >= 0 {
            Some(UniformLocation(val))
        } else {
            None
        }
    }

    /// You must guarantee val is in range 0..i32::MAX.
    pub unsafe fn new_unchecked(val: i32) -> Self {
        UniformLocation(val)
    }

    #[inline]
    pub fn into_i32(self) -> i32 {
        self.0
    }
}

/// A more compact representation of Option<UniformLocation>.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct OptionUniformLocation(pub i32);

impl OptionUniformLocation {
    #[inline]
    pub fn new(val: i32) -> Self {
        OptionUniformLocation(val)
    }

    #[inline]
    pub fn into_i32(self) -> i32 {
        self.0
    }

    #[inline]
    fn is_some(&self) -> bool {
        Option::<UniformLocation>::from(*self).is_some()
    }
}

impl From<OptionUniformLocation> for Option<UniformLocation> {
    #[inline]
    fn from(val: OptionUniformLocation) -> Self {
        UniformLocation::new(val.into_i32())
    }
}

macro_rules! impl_zero_copy_wrap_unwrap_all {
    ($T: ident, $OT: ident) => {
        impl_zero_copy_wrap_unwrap_all!($T, $OT,
             1,  2,  3,  4,  5,  6,  7,  8,
             9, 10, 11, 12, 13, 14, 15, 16,
            17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        );
    };

    ($T: ident, $OT: ident, $($N: expr,)*) => {
        $(
            impl WrapAll for [$T; $N] {
                type Wrapped = [$OT; $N];

                #[inline]
                fn wrap_all(self) -> Self::Wrapped {
                    // Safe because:
                    // 1. all T are valid Option<T>.
                    unsafe {
                        std::mem::transmute(self)
                    }
                }
            }

            impl UnwrapAll for [$OT; $N] {
                type Unwrapped = [$T; $N];

                #[inline]
                unsafe fn unwrap_all_unchecked(self) -> Self::Unwrapped {
                    std::mem::transmute(self)
                }

                #[inline]
                fn unwrap_all(self) -> Result<Self::Unwrapped, Self> {
                    // Safe because:
                    // 1. we ensure that all Option<TextureName> are Some(TextureName), and
                    // 2. all Some(TextureName) are valid TextureName.
                    unsafe {
                        if self.iter().all(|val| $T::new(val.into_i32()).is_some()) {
                            Ok(self.unwrap_all_unchecked())
                        } else {
                            Err(self)
                        }
                    }
                }
            }

            impl WrapAllRef for [$T; $N] {
                type Wrapped = [$OT; $N];

                #[inline]
                fn wrap_all_ref(&self) -> &Self::Wrapped {
                    // Safe because:
                    // 1. all T are valid Option<T>, and
                    // 2. we cannot write to a T as if it were an Option<T> through an immutable reference.
                    unsafe {
                        &*(self.as_ptr() as *const Self::Wrapped)
                    }
                }
            }

            impl UnwrapAllRef for [$OT; $N] {
                type Unwrapped = [$T; $N];

                #[inline]
                unsafe fn unwrap_all_ref_unchecked(&self) -> &Self::Unwrapped {
                    &*(self.as_ptr() as *const Self::Unwrapped)
                }

                #[inline]
                fn unwrap_all_ref(&self) -> Option<&Self::Unwrapped> {
                    // Safe because:
                    // 1. we ensure that all Option<T> are Some(T), and
                    // 2. all Some(T) are valid T.
                    unsafe {
                        if self.iter().all(|val| $T::new(val.into_i32()).is_some()) {
                            Some(self.unwrap_all_ref_unchecked())
                        } else {
                            None
                        }
                    }
                }
            }

            impl UnwrapAllMut for [$OT; $N] {
                #[inline]
                unsafe fn unwrap_all_mut_unchecked(&mut self) -> &mut Self::Unwrapped {
                    &mut *(self.as_mut_ptr() as *mut Self::Unwrapped)
                }

                #[inline]
                fn unwrap_all_mut(&mut self) -> Option<&mut Self::Unwrapped> {
                    // Safe because:
                    // 1. we ensure that all Option<T> are Some(T), and
                    // 2. all Some(T) are valid T, and
                    // 3. vice versa.
                    unsafe {
                        if self.iter().all(|val| $T::new(val.into_i32()).is_some()) {
                            Some(self.unwrap_all_mut_unchecked())
                        } else {
                            None
                        }
                    }
                }
            }
        )*

        impl WrapAll for Vec<$T> {
            type Wrapped = Vec<$OT>;

            #[inline]
            fn wrap_all(mut self) -> Self::Wrapped {
                unsafe {
                    let wrapped = Vec::from_raw_parts(
                        self.as_mut_ptr() as *mut $OT,
                        self.len(),
                        self.capacity()
                    );
                    std::mem::forget(self);
                    wrapped
                }
            }
        }

        impl UnwrapAll for Vec<$OT> {
            type Unwrapped = Vec<$T>;

            #[inline]
            unsafe fn unwrap_all_unchecked(mut self) -> Self::Unwrapped {
                let wrapped = Vec::from_raw_parts(
                    self.as_mut_ptr() as *mut $T,
                    self.len(),
                    self.capacity()
                );
                std::mem::forget(self);
                wrapped
            }

            #[inline]
            fn unwrap_all(self) -> Result<Self::Unwrapped, Self> {
                unsafe {
                    if self.iter().all(|val| $T::new(val.into_i32()).is_some()) {
                        Ok(self.unwrap_all_unchecked())
                    } else {
                        Err(self)
                    }
                }
            }
        }

        impl WrapAllRef for [$T] {
            type Wrapped = [$OT];

            #[inline]
            fn wrap_all_ref(&self) -> &Self::Wrapped {
                unsafe {
                    std::slice::from_raw_parts(self.as_ptr() as *const $OT, self.len())
                }
            }
        }

        impl UnwrapAllRef for [$OT] {
            type Unwrapped = [$T];

            #[inline]
            unsafe fn unwrap_all_ref_unchecked(&self) -> &Self::Unwrapped {
                std::slice::from_raw_parts(self.as_ptr() as *const $T, self.len())
            }

            #[inline]
            fn unwrap_all_ref(&self) -> Option<&Self::Unwrapped> {
                unsafe {
                    if self.iter().all(|val| $T::new(val.into_i32()).is_some()) {
                        Some(self.unwrap_all_ref_unchecked())
                    } else {
                        None
                    }
                }
            }
        }

        impl UnwrapAllMut for [$OT] {
            #[inline]
            unsafe fn unwrap_all_mut_unchecked(&mut self) -> &mut Self::Unwrapped {
                std::slice::from_raw_parts_mut(self.as_mut_ptr() as *mut $T, self.len())
            }

            #[inline]
            fn unwrap_all_mut(&mut self) -> Option<&mut Self::Unwrapped> {
                unsafe {
                    if self.iter().all($OT::is_some) {
                        Some(self.unwrap_all_mut_unchecked())
                    } else {
                        None
                    }
                }
            }
        }
    };
}

impl_zero_copy_wrap_unwrap_all!(AttributeLocation, OptionAttributeLocation);
impl_zero_copy_wrap_unwrap_all!(UniformLocation, OptionUniformLocation);
