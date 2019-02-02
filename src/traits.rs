use crate::enums;
use crate::symbols;

pub trait ShaderKind: Into<enums::ShaderKind> + Copy + Sized {}
impl<T: Into<enums::ShaderKind> + Copy + Sized> ShaderKind for T {}

pub trait CompileStatus: Into<enums::CompileStatus> + Copy + Sized {}
impl<T: Into<enums::CompileStatus> + Copy + Sized> CompileStatus for T {}

pub trait UncompiledCompileStatus: CompileStatus {
    const UNCOMPILED: Self;
}

impl UncompiledCompileStatus for enums::CompileStatus {
    const UNCOMPILED: Self = enums::CompileStatus::Uncompiled;
}

impl UncompiledCompileStatus for symbols::Uncompiled {
    const UNCOMPILED: Self = symbols::Uncompiled;
}

pub trait GetShaderivParam: Into<enums::GetShaderivParam> + Copy + Sized {}
impl<T: Into<enums::GetShaderivParam> + Copy + Sized> GetShaderivParam for T {}

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

fn x<KI, KO: From<KI>>(k: KI) -> KO {
    From::from(k)
}
