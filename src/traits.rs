use crate::enums;
use crate::symbols;

pub unsafe trait GetShaderivValue: AsMut<i32> {
    type Param: Into<enums::GetShaderivParam>;
}

unsafe impl GetShaderivValue for enums::RawShaderCompileStatus {
    type Param = symbols::CompileStatus;
}

// TODO(mickvangelderen): Implement remaining GetShaderivValue types.
