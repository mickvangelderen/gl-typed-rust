use crate::convert::*;

macro_rules! impl_zero_copy_wrap_unwrap_all {
    ($T: ident) => {
        impl_zero_copy_wrap_unwrap_all!($T,
             1,  2,  3,  4,  5,  6,  7,  8,
             9, 10, 11, 12, 13, 14, 15, 16,
            17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        );
    };

    ($T: ident, $($N: expr,)*) => {
        $(
            impl WrapAll for [$T; $N] {
                type Wrapped = [Option<$T>; $N];

                #[inline]
                fn wrap_all(self) -> Self::Wrapped {
                    // Safe because:
                    // 1. all T are valid Option<T>.
                    unsafe {
                        std::mem::transmute(self)
                    }
                }
            }

            impl UnwrapAll for [Option<$T>; $N] {
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
                        if self.iter().all(Option::is_some) {
                            Ok(self.unwrap_all_unchecked())
                        } else {
                            Err(self)
                        }
                    }
                }
            }

            impl WrapAllRef for [$T; $N] {
                type Wrapped = [Option<$T>; $N];

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

            impl UnwrapAllRef for [Option<$T>; $N] {
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
                        if self.iter().all(Option::is_some) {
                            Some(self.unwrap_all_ref_unchecked())
                        } else {
                            None
                        }
                    }
                }
            }

            impl UnwrapAllMut for [Option<$T>; $N] {
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
                        if self.iter().all(Option::is_some) {
                            Some(self.unwrap_all_mut_unchecked())
                        } else {
                            None
                        }
                    }
                }
            }
        )*

        impl WrapAll for Vec<$T> {
            type Wrapped = Vec<Option<$T>>;

            #[inline]
            fn wrap_all(mut self) -> Self::Wrapped {
                unsafe {
                    let wrapped = Vec::from_raw_parts(
                        self.as_mut_ptr() as *mut Option<$T>,
                        self.len(),
                        self.capacity()
                    );
                    std::mem::forget(self);
                    wrapped
                }
            }
        }

        impl UnwrapAll for Vec<Option<$T>> {
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
                    if self.iter().all(Option::is_some) {
                        Ok(self.unwrap_all_unchecked())
                    } else {
                        Err(self)
                    }
                }
            }
        }

        impl WrapAllRef for [$T] {
            type Wrapped = [Option<$T>];

            #[inline]
            fn wrap_all_ref(&self) -> &Self::Wrapped {
                unsafe {
                    std::slice::from_raw_parts(self.as_ptr() as *const Option<$T>, self.len())
                }
            }
        }

        impl UnwrapAllRef for [Option<$T>] {
            type Unwrapped = [$T];

            #[inline]
            unsafe fn unwrap_all_ref_unchecked(&self) -> &Self::Unwrapped {
                std::slice::from_raw_parts(self.as_ptr() as *const $T, self.len())
            }

            #[inline]
            fn unwrap_all_ref(&self) -> Option<&Self::Unwrapped> {
                unsafe {
                    if self.iter().all(Option::is_some) {
                        Some(self.unwrap_all_ref_unchecked())
                    } else {
                        None
                    }
                }
            }
        }

        impl UnwrapAllMut for [Option<$T>] {
            #[inline]
            unsafe fn unwrap_all_mut_unchecked(&mut self) -> &mut Self::Unwrapped {
                std::slice::from_raw_parts_mut(self.as_mut_ptr() as *mut $T, self.len())
            }

            #[inline]
            fn unwrap_all_mut(&mut self) -> Option<&mut Self::Unwrapped> {
                unsafe {
                    if self.iter().all(Option::is_some) {
                        Some(self.unwrap_all_mut_unchecked())
                    } else {
                        None
                    }
                }
            }
        }
    };
}

macro_rules! impl_names {
    ($($Name: ident,)*) => {
        $(
            /// No guarantees are made about the validity of the object this
            /// name represents.
            #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
            #[repr(transparent)]
            pub struct $Name(pub ::std::num::NonZeroU32);

            impl $Name {
                #[inline]
                pub fn new(name: u32) -> Option<Self> {
                    std::num::NonZeroU32::new(name).map($Name)
                }

                #[inline]
                pub const unsafe fn new_unchecked(name: u32) -> Self {
                    $Name(std::num::NonZeroU32::new_unchecked(name))
                }

                #[inline]
                pub fn into_u32(self) -> u32 {
                    self.0.get()
                }
            }

            impl_zero_copy_wrap_unwrap_all!($Name);
        )*
    };
}

impl_names!(
    BufferName,
    FramebufferName,
    ProgramName,
    ShaderName,
    TextureName,
    VertexArrayName,
);

// Even though there is a distinction to be made between an
// Option<FramebufferName> and the default framebuffer, I don't think it is
// likely enough to create the types NonDefaultFramebufferName, FramebufferName
// and the conversion between them for it. Maybe I'll change my mind.
