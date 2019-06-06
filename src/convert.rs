pub trait CastFrom<T> {
    fn cast_from(value: T) -> Self;
}

impl<T> CastFrom<T> for T {
    #[inline]
    fn cast_from(value: T) -> Self {
        value
    }
}

pub trait CastInto<T> {
    fn cast_into(self) -> T;
}

impl<F, I> CastInto<I> for F where I: CastFrom<F> {
    #[inline]
    fn cast_into(self) -> I {
        I::cast_from(self)
    }
}

macro_rules! impl_casts {
    ($(
        $A: ty => $B: ty,
    )*) => {
        $(
            impl CastFrom<$A> for $B {
                #[inline]
                fn cast_from(value: $A) -> Self {
                    value as Self
                }
            }
        )*
    };
}

impl_casts!(
    u32 => i32,
    i32 => u32,
);
