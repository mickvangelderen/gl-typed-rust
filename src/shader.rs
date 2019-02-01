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
    pub fn kind(&self) -> &K {
        &self.kind
    }

    /// Be careful, allows changing the kind to an incorrect value.
    #[inline]
    pub unsafe fn kind_mut(&mut self) -> &mut K {
        &mut self.kind
    }

    #[inline]
    pub fn name(&self) -> &ShaderName {
        &self.name
    }

    /// Be careful, allows changing the name to an incorrect value.
    #[inline]
    pub unsafe fn name_mut(&mut self) -> &mut ShaderName {
        &mut self.name
    }

    #[inline]
    pub fn status(&self) -> &S {
        &self.status
    }

    /// Be careful, allows changing the status to an incorrect value.
    #[inline]
    pub unsafe fn status_mut(&mut self) -> &mut S {
        &mut self.status
    }
}

impl<K, S: traits::CompileStatus> Shader<K, S> {
    #[inline]
    pub fn into_compiled(
        self,
    ) -> Result<Shader<K, symbols::Compiled>, Shader<K, symbols::Uncompiled>> {
        unsafe {
            let (kind, name, status) = self.into_raw_parts();
            match status.into() {
                enums::CompileStatus::Uncompiled => {
                    Err(Shader::from_raw_parts(kind, name, symbols::Uncompiled))
                }
                enums::CompileStatus::Compiled => {
                    Ok(Shader::from_raw_parts(kind, name, symbols::Compiled))
                }
            }
        }
    }
}
