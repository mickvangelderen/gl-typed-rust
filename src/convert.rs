pub trait WrapAll: Sized {
    type Wrapped: UnwrapAll;

    fn wrap_all(self) -> Self::Wrapped;
}

pub trait UnwrapAll: Sized {
    type Unwrapped: WrapAll;

    unsafe fn unwrap_all_unchecked(self) -> Self::Unwrapped;

    fn unwrap_all(self) -> Result<Self::Unwrapped, Self>;
}

pub trait WrapAllRef {
    type Wrapped: UnwrapAllRef + ?Sized;

    fn wrap_all_ref(&self) -> &Self::Wrapped;
}

pub trait UnwrapAllRef {
    type Unwrapped: WrapAllRef + ?Sized;

    unsafe fn unwrap_all_ref_unchecked(&self) -> &Self::Unwrapped;

    fn unwrap_all_ref(&self) -> Option<&Self::Unwrapped>;
}

pub trait UnwrapAllMut: UnwrapAllRef {
    unsafe fn unwrap_all_mut_unchecked(&mut self) -> &mut Self::Unwrapped;

    fn unwrap_all_mut(&mut self) -> Option<&mut Self::Unwrapped>;
}

/// Unsafe because from and into musn't do anything. Must guarantee that Self
/// values can be written to through a *mut T pointer.
pub unsafe trait Transmute<T>: Sized {
    fn transmute_from(val: T) -> Self;
    fn transmute_into(self) -> T;
    fn transmute_as_ref(&self) -> &T;
    fn transmute_as_mut(&mut self) -> &mut T;
}

unsafe impl<T> Transmute<T> for T {
    fn transmute_from(val: Self) -> Self {
        val
    }

    fn transmute_into(self) -> Self {
        self
    }

    fn transmute_as_ref(&self) -> &Self {
        self
    }

    fn transmute_as_mut(&mut self) -> &mut Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    #[repr(transparent)]
    struct UncheckedStatus(u32);

    unsafe impl Transmute<u32> for UncheckedStatus {
        #[inline]
        fn transmute_from(val: u32) -> Self {
            UncheckedStatus(val)
        }

        #[inline]
        fn transmute_into(self) -> u32 {
            self.0
        }

        #[inline]
        fn transmute_as_ref(&self) -> &u32 {
            &self.0
        }

        #[inline]
        fn transmute_as_mut(&mut self) -> &mut u32 {
            &mut self.0
        }
    }

    impl UncheckedStatus {
        #[inline]
        fn check(self) -> Result<Status, Self> {
            match self {
                UncheckedStatus(0) => Ok(Status::Good),
                UncheckedStatus(1) => Ok(Status::Bad),
                other => Err(other),
            }
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    #[repr(u32)]
    enum Status {
        Good = 0,
        Bad = 1,
    }

    #[test]
    fn do_from() {
        let mut us = UncheckedStatus(0);
        {
            let r: &mut u32 = us.transmute_as_mut();
            *r = 2;
        }
        let cs = us.check();
        assert!(cs.is_err());
        // let r: &Status = us.transmute_as_ref();
    }
}
