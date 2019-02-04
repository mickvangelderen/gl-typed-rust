use crate::enums;
use crate::symbols;
use crate::traits;

/// The name of a shader, without the kind.
impl_name!(ShaderName);

/// A generic shader object for which we may know the kind at compile-time.
#[derive(Debug)]
pub struct Shader<K, S> {
    kind: K,
    name: ShaderName,
    status: S,
}

#[derive(Debug)]
pub enum ShaderKind<C> {
    Compute(Shader<symbols::Compute, C>),
    Vertex(Shader<symbols::Vertex, C>),
    TessControl(Shader<symbols::TessControl, C>),
    TessEvaluation(Shader<symbols::TessEvaluation, C>),
    Geometry(Shader<symbols::Geometry, C>),
    Fragment(Shader<symbols::Fragment, C>),
}

#[derive(Debug)]
pub enum CompileStatus<K> {
    Uncompiled(Shader<K, symbols::Uncompiled>),
    Compiled(Shader<K, symbols::Compiled>),
}

impl<K, S> Shader<K, S> {
    /// Does not verify whether name is of the given kind and status.
    #[inline]
    pub unsafe fn from_raw_parts(kind: K, name: ShaderName, status: S) -> Self {
        Shader { kind, name, status }
    }

    #[inline]
    pub fn into_raw_parts(self) -> (K, ShaderName, S) {
        let Shader { kind, name, status } = self;
        (kind, name, status)
    }

    #[inline]
    pub fn without_kind(self) -> Shader<symbols::Unknown, S> {
        let Shader { name, status, .. } = self;
        Shader {
            kind: symbols::Unknown,
            name,
            status,
        }
    }

    #[inline]
    pub fn without_status(self) -> Shader<K, symbols::Unknown> {
        let Shader { kind, name, .. } = self;
        Shader {
            kind,
            name,
            status: symbols::Unknown,
        }
    }

    #[inline]
    pub fn kind(&self) -> &K {
        &self.kind
    }

    /// Be careful, allows changing the kind to an incorrect value.
    #[inline]
    pub unsafe fn set_kind(&mut self, kind: K) {
        self.kind = kind
    }

    #[inline]
    pub fn name(&self) -> &ShaderName {
        &self.name
    }

    /// Be careful, allows changing the name to an incorrect value.
    #[inline]
    pub unsafe fn set_name(&mut self, name: ShaderName) {
        self.name = name;
    }

    #[inline]
    pub fn status(&self) -> &S {
        &self.status
    }

    /// Be careful, allows changing the status to an incorrect value.
    #[inline]
    pub unsafe fn set_status(&mut self, status: S) {
        self.status = status;
    }
}

impl<K: traits::ShaderKind, S> Shader<K, S> {
    #[inline]
    pub fn determine_kind(self) -> ShaderKind<S> {
        unsafe {
            let (kind, name, status) = self.into_raw_parts();
            match kind.into() {
                enums::ShaderKind::Compute => {
                    ShaderKind::Compute(Shader::from_raw_parts(symbols::Compute, name, status))
                }
                enums::ShaderKind::Vertex => {
                    ShaderKind::Vertex(Shader::from_raw_parts(symbols::Vertex, name, status))
                }
                enums::ShaderKind::TessControl => ShaderKind::TessControl(Shader::from_raw_parts(
                    symbols::TessControl,
                    name,
                    status,
                )),
                enums::ShaderKind::TessEvaluation => ShaderKind::TessEvaluation(
                    Shader::from_raw_parts(symbols::TessEvaluation, name, status),
                ),
                enums::ShaderKind::Geometry => {
                    ShaderKind::Geometry(Shader::from_raw_parts(symbols::Geometry, name, status))
                }
                enums::ShaderKind::Fragment => {
                    ShaderKind::Fragment(Shader::from_raw_parts(symbols::Fragment, name, status))
                }
            }
        }
    }
}

impl<K, S: traits::CompileStatus> Shader<K, S> {
    #[inline]
    pub fn determine_status(
        self,
    ) -> CompileStatus<K> {
        unsafe {
            let (kind, name, status) = self.into_raw_parts();
            match status.into() {
                enums::CompileStatus::Uncompiled => {
                    CompileStatus::Uncompiled(Shader::from_raw_parts(kind, name, symbols::Uncompiled))
                }
                enums::CompileStatus::Compiled => {
                    CompileStatus::Compiled(Shader::from_raw_parts(kind, name, symbols::Compiled))
                }
            }
        }
    }
}
