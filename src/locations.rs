/// Guaranteed to be in range 0..i32::MAX.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct AttributeLocation(u32);

impl AttributeLocation {
    const NONE: i32 = -1;

    #[inline]
    fn is_some(val: i32) -> bool {
        val >= 0
    }

    #[inline]
    fn is_none(val: i32) -> bool {
        !Self::is_some(val)
    }

    #[inline]
    pub fn new(val: i32) -> Option<Self> {
        if Self::is_some(val) {
            Some(AttributeLocation(val as u32))
        } else {
            None
        }
    }

    /// You must guarantee val is in range 0..i32::MAX.
    pub const unsafe fn new_unchecked(val: i32) -> Self {
        AttributeLocation(val as u32)
    }

    #[inline]
    pub fn into_u32(self) -> u32 {
        self.0
    }
}

/// A more compact representation of Option<AttributeLocation>.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct OptionAttributeLocation(pub i32);

impl OptionAttributeLocation {
    pub const NONE: Self = OptionAttributeLocation(AttributeLocation::NONE);

    #[inline]
    pub fn is_some(&self) -> bool {
        AttributeLocation::is_some(self.0)
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        AttributeLocation::is_none(self.0)
    }

    #[inline]
    pub fn new(val: i32) -> Self {
        OptionAttributeLocation(val)
    }

    #[inline]
    pub fn into_option(self) -> Option<AttributeLocation> {
        AttributeLocation::new(self.0)
    }
}

impl From<OptionAttributeLocation> for Option<AttributeLocation> {
    #[inline]
    fn from(val: OptionAttributeLocation) -> Self {
        val.into_option()
    }
}

impl Default for OptionAttributeLocation {
    fn default() -> Self {
        Self::NONE
    }
}

/// Guaranteed to be in range 0..i32::MAX.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct UniformLocation(i32);

impl UniformLocation {
    const NONE: i32 = -1;

    #[inline]
    fn is_some(val: i32) -> bool {
        val >= 0
    }

    #[inline]
    fn is_none(val: i32) -> bool {
        !Self::is_some(val)
    }

    #[inline]
    pub fn new(val: i32) -> Option<Self> {
        if Self::is_some(val) {
            Some(UniformLocation(val))
        } else {
            None
        }
    }

    /// You must guarantee val is in range 0..i32::MAX.
    pub const unsafe fn new_unchecked(val: i32) -> Self {
        UniformLocation(val)
    }

    #[inline]
    pub fn into_i32(self) -> i32 {
        self.0
    }
}

/// A more compact representation of Option<UniformLocation>.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct OptionUniformLocation(pub i32);

impl OptionUniformLocation {
    pub const NONE: Self = OptionUniformLocation(UniformLocation::NONE);

    #[inline]
    pub fn is_some(&self) -> bool {
        UniformLocation::is_some(self.0)
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        UniformLocation::is_none(self.0)
    }

    #[inline]
    pub fn new(val: i32) -> Self {
        OptionUniformLocation(val)
    }

    #[inline]
    pub fn into_option(self) -> Option<UniformLocation> {
        UniformLocation::new(self.0)
    }
}

impl From<OptionUniformLocation> for Option<UniformLocation> {
    #[inline]
    fn from(val: OptionUniformLocation) -> Self {
        val.into_option()
    }
}

impl Default for OptionUniformLocation {
    fn default() -> Self {
        Self::NONE
    }
}

/// Guaranteed to be in range 0..u32::MAX - 1.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct UniformBlockIndex(u32);

impl UniformBlockIndex {
    const NONE: u32 = std::u32::MAX;

    #[inline]
    fn is_some(val: u32) -> bool {
        val < Self::NONE
    }

    #[inline]
    fn is_none(val: u32) -> bool {
        !Self::is_some(val)
    }

    #[inline]
    pub fn new(val: u32) -> Option<Self> {
        if Self::is_some(val) {
            Some(UniformBlockIndex(val))
        } else {
            None
        }
    }

    /// You must guarantee val is in range 0..u32::MAX.
    pub const unsafe fn new_unchecked(val: u32) -> Self {
        UniformBlockIndex(val)
    }

    #[inline]
    pub fn into_u32(self) -> u32 {
        self.0
    }
}

/// A more compact representation of Option<UniformLocation>.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct OptionUniformBlockIndex(pub u32);

impl OptionUniformBlockIndex {
    pub const NONE: Self = OptionUniformBlockIndex(UniformBlockIndex::NONE);

    #[inline]
    pub fn is_some(&self) -> bool {
        UniformBlockIndex::is_some(self.0)
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        UniformBlockIndex::is_none(self.0)
    }

    #[inline]
    pub fn new(val: u32) -> Self {
        OptionUniformBlockIndex(val)
    }

    #[inline]
    pub fn into_option(self) -> Option<UniformBlockIndex> {
        UniformBlockIndex::new(self.0)
    }
}

impl From<OptionUniformBlockIndex> for Option<UniformBlockIndex> {
    #[inline]
    fn from(val: OptionUniformBlockIndex) -> Self {
        val.into_option()
    }
}

impl Default for OptionUniformBlockIndex {
    fn default() -> Self {
        Self::NONE
    }
}

#[derive(Debug, Copy, Clone)]
pub struct VertexArrayBufferBindingIndex(u32);

impl VertexArrayBufferBindingIndex {
    pub const fn from_u32(value: u32) -> Self {
        VertexArrayBufferBindingIndex(value)
    }

    pub const fn to_u32(self) -> u32 {
        self.0
    }
}
