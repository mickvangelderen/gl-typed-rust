#[macro_export]
macro_rules! impl_name {
    ($Name:ident) => {
        #[derive(Debug)]
        #[repr(transparent)]
        pub struct $Name(::std::num::NonZeroU32);

        impl $Name {
            #[inline]
            pub unsafe fn from_u32(name: u32) -> Option<Self> {
                std::num::NonZeroU32::new(name).map($Name)
            }

            #[inline]
            pub const unsafe fn from_u32_unchecked(name: u32) -> Self {
                $Name(std::num::NonZeroU32::new_unchecked(name))
            }

            /// Converts the name into a number without dropping it.
            #[inline]
            pub unsafe fn into_u32(self) -> u32 {
                std::mem::ManuallyDrop::new(self).as_u32()
            }

            #[inline]
            pub fn as_u32(&self) -> u32 {
                self.0.get()
            }
        }

        impl Drop for $Name {
            #[cold]
            fn drop(&mut self) {
            }
        }
    };
}

