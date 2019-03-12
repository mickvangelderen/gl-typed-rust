/// Guaranteed to be in range 0..i32::MAX.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct AttributeLocation(u32);

impl AttributeLocation {
    #[inline]
    pub fn new(val: i32) -> Option<Self> {
        if val >= 0 {
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
    #[inline]
    pub fn new(val: i32) -> Self {
        OptionAttributeLocation(val)
    }

    #[inline]
    pub fn into_option(self) -> Option<AttributeLocation> {
        AttributeLocation::new(self.0)
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
    #[inline]
    pub fn new(val: i32) -> Option<Self> {
        if val >= 0 {
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
    #[inline]
    pub fn new(val: i32) -> Self {
        OptionUniformLocation(val)
    }

    #[inline]
    pub fn into_option(self) -> Option<UniformLocation> {
        UniformLocation::new(self.0)
    }
}

unsafe impl convute::marker::Transmute<OptionUniformLocation> for UniformLocation {}
unsafe impl convute::marker::TryTransmute<UniformLocation> for OptionUniformLocation {
    #[inline]
    fn can_transmute(&self) -> bool {
        self.into_option().is_some()
    }
}
