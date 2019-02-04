use crate::enums;
use crate::symbols;

pub trait FromUnchecked<T>: From<T> {
    unsafe fn from_unchecked(v: T) -> Self;
}

pub trait IntoUnchecked<T>: Into<T> {
    unsafe fn into_unchecked(self) -> T;
}

impl<T> FromUnchecked<T> for T {
    unsafe fn from_unchecked(v: Self) -> Self {
        v
    }
}

impl<F, T> IntoUnchecked<F> for T
where
    F: FromUnchecked<T>,
{
    unsafe fn into_unchecked(self) -> F {
        FromUnchecked::from_unchecked(self)
    }
}

pub unsafe trait GetShaderivValue: AsMut<i32> {
    type Param: Into<enums::GetShaderivParam>;
}

unsafe impl GetShaderivValue for enums::RawShaderCompileStatus {
    type Param = symbols::CompileStatus;
}

// TODO(mickvangelderen): Implement remaining GetShaderivValue types.

