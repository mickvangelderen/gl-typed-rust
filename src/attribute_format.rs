use crate::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ComponentCount {
    P1,
    P2,
    P3,
    P4,
}

impl ComponentCount {
    #[inline]
    pub fn to_u32(&self) -> u32 {
        match *self {
            Self::P1 => 1,
            Self::P2 => 2,
            Self::P3 => 3,
            Self::P4 => 4,
        }
    }

    #[inline]
    pub fn to_i32(&self) -> i32 {
        match *self {
            Self::P1 => 1,
            Self::P2 => 2,
            Self::P3 => 3,
            Self::P4 => 4,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub enum AttributeFormatF {
    F16(ComponentCount),
    F32(ComponentCount),
    Fixed16_16(ComponentCount),
    I8(ComponentCount, bool),
    U8(ComponentCount, bool),
    I16(ComponentCount, bool),
    U16(ComponentCount, bool),
    I32(ComponentCount, bool),
    U32(ComponentCount, bool),
    I_2_10_10_10_REV,
    U_2_10_10_10_REV,
    U_10F_11F_11F_REV,
}

impl AttributeFormatF {
    #[inline]
    pub fn byte_size(&self) -> u32 {
        match *self {
            Self::I8(c, _) | Self::U8(c, _) => c.to_u32(),
            Self::I16(c, _) | Self::U16(c, _) | Self::F16(c) => c.to_u32() * 2,
            Self::I32(c, _) | Self::U32(c, _) | Self::F32(c) | Self::Fixed16_16(c) => {
                c.to_u32() * 4
            }
            Self::I_2_10_10_10_REV | Self::U_2_10_10_10_REV => 4,
            Self::U_10F_11F_11F_REV => 3,
        }
    }

    #[inline]
    fn should_normalize(&self) -> bool {
        match *self {
            Self::I8(_, n)
            | Self::U8(_, n)
            | Self::I16(_, n)
            | Self::U16(_, n)
            | Self::I32(_, n)
            | Self::U32(_, n) => n,
            Self::F16(_)
            | Self::F32(_)
            | Self::Fixed16_16(_)
            | Self::I_2_10_10_10_REV
            | Self::U_2_10_10_10_REV
            | Self::U_10F_11F_11F_REV => false,
        }
    }

    #[inline]
    fn component_count(&self) -> ComponentCount {
        match *self {
            Self::F16(c)
            | Self::F32(c)
            | Self::Fixed16_16(c)
            | Self::I8(c, _)
            | Self::U8(c, _)
            | Self::I16(c, _)
            | Self::U16(c, _)
            | Self::I32(c, _)
            | Self::U32(c, _) => c,
            Self::I_2_10_10_10_REV | Self::U_2_10_10_10_REV => ComponentCount::P4,
            Self::U_10F_11F_11F_REV => ComponentCount::P3,
        }
    }

    #[inline]
    fn component_type(&self) -> u32 {
        match *self {
            Self::F16(_) => gl::HALF_FLOAT,
            Self::F32(_) => gl::FLOAT,
            Self::Fixed16_16(_) => gl::FIXED,
            Self::I8(_, _) => gl::BYTE,
            Self::U8(_, _) => gl::UNSIGNED_BYTE,
            Self::I16(_, _) => gl::SHORT,
            Self::U16(_, _) => gl::UNSIGNED_SHORT,
            Self::I32(_, _) => gl::INT,
            Self::U32(_, _) => gl::UNSIGNED_INT,
            Self::I_2_10_10_10_REV => gl::INT_2_10_10_10_REV,
            Self::U_2_10_10_10_REV => gl::UNSIGNED_INT_2_10_10_10_REV,
            Self::U_10F_11F_11F_REV => gl::UNSIGNED_INT_10F_11F_11F_REV,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub enum AttributeFormatI {
    I8(ComponentCount),
    U8(ComponentCount),
    I16(ComponentCount),
    U16(ComponentCount),
    I32(ComponentCount),
    U32(ComponentCount),
}

impl AttributeFormatI {
    #[inline]
    pub fn byte_size(&self) -> u32 {
        match *self {
            Self::I8(c) | Self::U8(c) => c.to_u32(),
            Self::I16(c) | Self::U16(c) => c.to_u32() * 2,
            Self::I32(c) | Self::U32(c) => c.to_u32() * 4,
        }
    }

    #[inline]
    fn component_count(&self) -> ComponentCount {
        match *self {
            Self::I8(c)
            | Self::U8(c)
            | Self::I16(c)
            | Self::U16(c)
            | Self::I32(c)
            | Self::U32(c) => c,
        }
    }

    #[inline]
    fn component_type(&self) -> u32 {
        match *self {
            Self::I8(_) => gl::BYTE,
            Self::U8(_) => gl::UNSIGNED_BYTE,
            Self::I16(_) => gl::SHORT,
            Self::U16(_) => gl::UNSIGNED_SHORT,
            Self::I32(_) => gl::INT,
            Self::U32(_) => gl::UNSIGNED_INT,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub enum AttributeFormatL {
    F64(ComponentCount),
}

impl AttributeFormatL {
    #[inline]
    pub fn byte_size(&self) -> u32 {
        match *self {
            Self::F64(c) => c.to_u32() * 8,
        }
    }

    #[inline]
    fn component_count(&self) -> ComponentCount {
        match *self {
            Self::F64(c) => c,
        }
    }

    #[inline]
    fn component_type(&self) -> u32 {
        match *self {
            Self::F64(_) => gl::DOUBLE,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AttributeFormat {
    F(AttributeFormatF),
    I(AttributeFormatI),
    L(AttributeFormatL),
}

impl AttributeFormat {
    pub fn byte_size(&self) -> u32 {
        match *self {
            Self::F(a) => a.byte_size(),
            Self::I(a) => a.byte_size(),
            Self::L(a) => a.byte_size(),
        }
    }
}

impl From<AttributeFormatF> for AttributeFormat {
    fn from(value: AttributeFormatF) -> Self {
        Self::F(value)
    }
}

impl From<AttributeFormatI> for AttributeFormat {
    fn from(value: AttributeFormatI) -> Self {
        Self::I(value)
    }
}

impl From<AttributeFormatL> for AttributeFormat {
    fn from(value: AttributeFormatL) -> Self {
        Self::L(value)
    }
}

impl Gl {
    #[inline]
    pub unsafe fn vertex_array_attrib_format(
        &self,
        vertex_array_name: VertexArrayName,
        location: AttributeLocation,
        format: AttributeFormat,
        offset: u32,
    ) {
        match format {
            AttributeFormat::F(format) => {
                self.vertex_array_attrib_format_f(vertex_array_name, location, format, offset)
            }
            AttributeFormat::I(format) => {
                self.vertex_array_attrib_format_i(vertex_array_name, location, format, offset)
            }
            AttributeFormat::L(format) => {
                self.vertex_array_attrib_format_l(vertex_array_name, location, format, offset)
            }
        }
    }

    #[inline]
    pub unsafe fn vertex_array_attrib_format_f(
        &self,
        vertex_array_name: VertexArrayName,
        location: AttributeLocation,
        format: AttributeFormatF,
        offset: u32,
    ) {
        self.gl.VertexArrayAttribFormat(
            vertex_array_name.to_u32(),
            location.to_u32(),
            format.component_count().to_i32(),
            format.component_type(),
            format.should_normalize() as u8,
            offset,
        );
    }

    #[inline]
    pub unsafe fn vertex_array_attrib_format_i(
        &self,
        vertex_array_name: VertexArrayName,
        location: AttributeLocation,
        format: AttributeFormatI,
        offset: u32,
    ) {
        self.gl.VertexArrayAttribIFormat(
            vertex_array_name.to_u32(),
            location.to_u32(),
            format.component_count().to_i32(),
            format.component_type(),
            offset,
        );
    }

    #[inline]
    pub unsafe fn vertex_array_attrib_format_l(
        &self,
        vertex_array_name: VertexArrayName,
        location: AttributeLocation,
        format: AttributeFormatL,
        offset: u32,
    ) {
        self.gl.VertexArrayAttribLFormat(
            vertex_array_name.to_u32(),
            location.to_u32(),
            format.component_count().to_i32(),
            format.component_type(),
            offset,
        );
    }
}
