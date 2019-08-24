macro_rules! impl_option_type {
    (
        $Name: ident,
        $OptionName: ident,
        $Raw: ident,
        $from_raw: ident,
        $from_raw_unchecked: ident,
        $to_raw: ident,
        $NONE: expr,
    ) => {
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
        #[repr(transparent)]
        pub struct $Name($Raw);

        impl $Name {
            const NONE: $Raw = $NONE;

            #[inline]
            pub fn $from_raw(val: $Raw) -> Option<Self> {
                match val {
                    none if none == Self::NONE => None,
                    some => Some(Self(some)),
                }
            }

            /// You must guarantee val is in range 0..$Raw::MAX.
            #[inline]
            pub const unsafe fn $from_raw_unchecked(val: $Raw) -> Self {
                Self(val)
            }

            #[inline]
            pub const fn $to_raw(&self) -> $Raw {
                self.0
            }
        }

        /// A more compact representation of Option<UniformLocation>.
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
        #[repr(transparent)]
        pub struct $OptionName(pub $Raw);

        impl $OptionName {
            pub const NONE: Self = Self($NONE);

            #[inline]
            pub fn is_some(&self) -> bool {
                self != &Self::NONE
            }

            #[inline]
            pub fn is_none(&self) -> bool {
                self == &Self::NONE
            }

            #[inline]
            pub const fn $from_raw(val: $Raw) -> Self {
                Self(val)
            }

            #[inline]
            pub const fn $to_raw(&self) -> $Raw {
                self.0
            }

            #[inline]
            pub fn from_option(val: Option<$Name>) -> Self {
                match val {
                    Some(val) => Self::$from_raw(val.$to_raw()),
                    None => Self::NONE,
                }
            }

            #[inline]
            pub fn to_option(&self) -> Option<$Name> {
                $Name::$from_raw(self.0)
            }
        }

        impl Default for $OptionName {
            fn default() -> Self {
                Self::NONE
            }
        }

        impl From<Option<$Name>> for $OptionName {
            #[inline]
            fn from(val: Option<$Name>) -> Self {
                Self::from_option(val)
            }
        }

        impl From<$OptionName> for Option<$Name> {
            #[inline]
            fn from(val: $OptionName) -> Self {
                val.to_option()
            }
        }
    };
}

impl_option_type! {
    AttributeLocation,
    OptionAttributeLocation,
    i32,
    from_i32,
    from_i32_unchecked,
    to_i32,
    -1,
}

impl AttributeLocation {
    #[deprecated]
    #[inline]
    pub fn into_i32(self) -> i32 {
        self.0
    }

    #[inline]
    pub(crate) fn to_u32(&self) -> u32 {
        self.0 as u32
    }
}

impl OptionAttributeLocation {
    #[deprecated]
    #[inline]
    pub fn new(val: i32) -> Self {
        Self(val)
    }
}

impl_option_type! {
    UniformLocation,
    OptionUniformLocation,
    i32,
    from_i32,
    from_i32_unchecked,
    to_i32,
    -1,
}

impl UniformLocation {
    #[deprecated]
    #[inline]
    pub fn into_i32(self) -> i32 {
        self.0
    }
}

impl OptionUniformLocation {
    #[deprecated]
    #[inline]
    pub fn new(val: i32) -> Self {
        Self(val)
    }
}

impl_option_type! {
    UniformBlockIndex,
    OptionUniformBlockIndex,
    u32,
    from_u32,
    from_u32_unchecked,
    to_u32,
    std::u32::MAX,
}

impl UniformBlockIndex {
    #[deprecated]
    #[inline]
    pub fn into_u32(self) -> u32 {
        self.0
    }
}

impl OptionUniformBlockIndex {
    #[deprecated]
    #[inline]
    pub fn new(val: u32) -> Self {
        Self(val)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct VertexArrayBufferBindingIndex(u32);

impl VertexArrayBufferBindingIndex {
    pub const fn from_u32(value: u32) -> Self {
        VertexArrayBufferBindingIndex(value)
    }

    pub const fn to_u32(&self) -> u32 {
        self.0
    }
}
