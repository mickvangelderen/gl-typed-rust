//! The param traits are unsafe because their associated type Value will be cast
//! to a pointer of some other type.

use crate::symbols;
use crate::types;
use crate::convert::*;

pub unsafe trait GetShaderivParam: Into<types::GetShaderivParam> {
    type Value: Transmute<i32>;
}

unsafe impl GetShaderivParam for symbols::CompileStatus {
    type Value = types::UncheckedShaderCompileStatus;
}

unsafe impl GetShaderivParam for symbols::InfoLogLength {
    type Value = i32;
}

pub unsafe trait GetProgramivParam: Into<types::GetProgramivParam> {
    type Value: Transmute<i32>;
}

unsafe impl GetProgramivParam for symbols::LinkStatus {
    type Value = types::UncheckedProgramLinkStatus;
}

unsafe impl GetProgramivParam for symbols::InfoLogLength {
    type Value = i32;
}

pub unsafe trait TexParameteriParam: Into<types::TexParameteriParam> {
    type Target: Into<types::TextureTarget>;
    type Value: Into<i32>;
}

unsafe impl TexParameteriParam for symbols::DepthStencilTextureMode {
    type Target = types::TextureTarget;
    type Value = types::DepthStencilTextureMode;
}

unsafe impl TexParameteriParam for symbols::TextureBaseLevel {
    type Target = types::TextureTarget;
    type Value = i32;
}

unsafe impl TexParameteriParam for symbols::TextureMaxLevel {
    type Target = types::TextureTarget;
    type Value = i32;
}

unsafe impl TexParameteriParam for symbols::TextureMagFilter {
    type Target = types::TextureTarget;
    type Value = types::TextureMagFilter;
}

unsafe impl TexParameteriParam for symbols::TextureMinFilter {
    type Target = types::TextureTarget;
    type Value = types::TextureMinFilter;
}

unsafe impl TexParameteriParam for symbols::TextureWrapS {
    type Target = types::TextureTarget;
    type Value = types::TextureWrap;
}

unsafe impl TexParameteriParam for symbols::TextureWrapT {
    type Target = types::TextureTargetGE2D;
    type Value = types::TextureWrap;
}

unsafe impl TexParameteriParam for symbols::TextureWrapR {
    type Target = types::TextureTargetGE3D;
    type Value = types::TextureWrap;
}
