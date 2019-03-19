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
    pub unsafe fn new_unchecked(val: i32) -> Self {
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

unsafe impl convute::marker::Transmute<OptionAttributeLocation> for AttributeLocation {}
unsafe impl convute::marker::TryTransmute<AttributeLocation> for OptionAttributeLocation {
    #[inline]
    fn can_transmute(&self) -> bool {
        self.into_option().is_some()
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
    pub unsafe fn new_unchecked(val: i32) -> Self {
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

unsafe impl convute::marker::Transmute<OptionUniformLocation> for UniformLocation {}
unsafe impl convute::marker::TryTransmute<UniformLocation> for OptionUniformLocation {
    #[inline]
    fn can_transmute(&self) -> bool {
        self.into_option().is_some()
    }
}
