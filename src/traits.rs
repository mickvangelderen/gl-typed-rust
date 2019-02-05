use crate::enums;
use crate::symbols;

pub trait UncheckedFrom<T>: From<T> {
    unsafe fn unchecked_from(v: T) -> Self;
}

pub trait UncheckedInto<T>: Into<T> {
    unsafe fn unchecked_into(self) -> T;
}

impl<T> UncheckedFrom<T> for T {
    unsafe fn unchecked_from(v: Self) -> Self {
        v
    }
}

impl<F, T> UncheckedInto<F> for T
where
    F: UncheckedFrom<T>,
{
    unsafe fn unchecked_into(self) -> F {
        UncheckedFrom::unchecked_from(self)
    }
}

pub unsafe trait GetShaderivValue: AsMut<i32> {
    type Param: Into<enums::GetShaderivParam>;
}

unsafe impl GetShaderivValue for enums::RawShaderCompileStatus {
    type Param = symbols::CompileStatus;
}

// TODO(mickvangelderen): Implement remaining GetShaderivValue types.
