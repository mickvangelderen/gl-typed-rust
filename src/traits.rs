use crate::enums;
use crate::symbols;

pub trait ShaderKind: Into<enums::ShaderKind> + Copy + Sized {}

pub trait CompileStatus: Into<enums::CompileStatus> + Copy + Sized {}

pub trait UncompiledCompileStatus: CompileStatus {
    const UNCOMPILED: Self;
}

pub trait GetShaderivParam: Into<enums::GetShaderivParam> + Copy + Sized {}

pub trait GetShaderivValue: Sized {
    type Param: GetShaderivParam;

    fn as_i32_mut(&mut self) -> &mut i32;
}

impl GetShaderivValue for i32 {
    type Param = enums::GetShaderivParam;

    fn as_i32_mut(&mut self) -> &mut i32 {
        self
    }
}

impl GetShaderivValue for enums::CompileStatus {
    type Param = symbols::CompileStatus;

    fn as_i32_mut(&mut self) -> &mut i32 {
        unsafe { &mut *(self as *mut enums::CompileStatus as *mut i32) }
    }
}

// TODO(mickvangelderen): Implement remaining GetShaderivValue types.
