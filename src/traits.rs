//! The param traits are unsafe because their associated type Value will be cast
//! to a pointer of some other type.

use crate::symbols;
use crate::types;
pub trait GetShaderivParam: Into<types::GetShaderivParam>
where
    i32: convute::marker::Transmute<Self::Value>,
{
    type Value: convute::marker::Transmute<i32>;
}

impl GetShaderivParam for symbols::CompileStatus {
    type Value = types::UncheckedShaderCompileStatus;
}

impl GetShaderivParam for symbols::InfoLogLength {
    type Value = i32;
}

pub trait GetProgramivParam: Into<types::GetProgramivParam>
where
    i32: convute::marker::Transmute<Self::Value>,
{
    type Value: convute::marker::Transmute<i32>;
}

impl GetProgramivParam for symbols::LinkStatus {
    type Value = types::UncheckedProgramLinkStatus;
}

impl GetProgramivParam for symbols::InfoLogLength {
    type Value = i32;
}

pub trait TexParameteriParam: Into<types::TexParameteriParam> {
    type Target: Into<types::TextureTarget>;
    type Value: convute::marker::Transmute<i32>;
}

impl TexParameteriParam for symbols::DepthStencilTextureMode {
    type Target = types::TextureTarget;
    type Value = types::DepthStencilTextureMode;
}

impl TexParameteriParam for symbols::TextureBaseLevel {
    type Target = types::TextureTarget;
    type Value = i32;
}

impl TexParameteriParam for symbols::TextureMaxLevel {
    type Target = types::TextureTarget;
    type Value = i32;
}

impl TexParameteriParam for symbols::TextureMagFilter {
    type Target = types::TextureTarget;
    type Value = types::TextureMagFilter;
}

impl TexParameteriParam for symbols::TextureMinFilter {
    type Target = types::TextureTarget;
    type Value = types::TextureMinFilter;
}

impl TexParameteriParam for symbols::TextureWrapS {
    type Target = types::TextureTarget;
    type Value = types::TextureWrap;
}

impl TexParameteriParam for symbols::TextureWrapT {
    type Target = types::TextureTarget;
    type Value = types::TextureWrap;
}

impl TexParameteriParam for symbols::TextureWrapR {
    type Target = types::TextureTarget;
    type Value = types::TextureWrap;
}

pub trait TexParameterfParam: Into<types::TexParameterfParam> {
    type Target: Into<types::TextureTarget>;
    type Value: convute::marker::Transmute<f32>;
}

impl TexParameterfParam for symbols::TextureMaxAnisotropy {
    type Target = types::TextureTarget;
    type Value = f32;
}

pub trait SamplerParameteriValue {
    fn to_i32(self) -> i32;
}

impl SamplerParameteriValue for types::TextureMagFilter {
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl SamplerParameteriValue for types::TextureMinFilter {
    fn to_i32(self) -> i32 {
        self as i32
    }
}

impl SamplerParameteriValue for types::TextureWrap {
    fn to_i32(self) -> i32 {
        self as i32
    }
}

pub trait SamplerParameteriParam: Into<types::SamplerParameteri> {
    type Value: SamplerParameteriValue;
}

impl SamplerParameteriParam for symbols::TextureMagFilter {
    type Value = types::TextureMagFilter;
}

impl SamplerParameteriParam for symbols::TextureMinFilter {
    type Value = types::TextureMinFilter;
}

impl SamplerParameteriParam for symbols::TextureWrapS {
    type Value = types::TextureWrap;
}

impl SamplerParameteriParam for symbols::TextureWrapT {
    type Value = types::TextureWrap;
}

impl SamplerParameteriParam for symbols::TextureWrapR {
    type Value = types::TextureWrap;
}
