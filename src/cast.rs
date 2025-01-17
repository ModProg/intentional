/// A trait for casting a type to another type using the `as` operator.
pub trait CastInto<To> {
    /// Returns `self as To`.
    fn cast_into(self) -> To;
}

/// A trait for casting a type from another type using the `as` operator.
pub trait CastFrom<From> {
    /// Returns `from as Self`.
    fn from_cast(from: From) -> Self;
}

impl<A, B> CastInto<A> for B
where
    A: CastFrom<B>,
{
    #[inline]
    fn cast_into(self) -> A {
        A::from_cast(self)
    }
}

impl<A> CastFrom<A> for A {
    #[inline]
    fn from_cast(from: A) -> Self {
        from
    }
}

/// Allows casting from this type to other types using
/// [`CastFrom`]/[`CastInto`].
pub trait Cast: Sized {
    /// Casts `self` to the `To` type. This may be a lossy operation.
    fn cast<To: CastFrom<Self>>(self) -> To;
}

impl<A> Cast for A {
    #[inline]
    fn cast<To: CastFrom<Self>>(self) -> To {
        To::from_cast(self)
    }
}

macro_rules! impl_cast_between {
    ($a:ident, $b:ident) => {
        impl CastFrom<$b> for $a {
            #[inline]
            fn from_cast(from: $b) -> Self {
                from as $a
            }
        }
        impl CastFrom<$a> for $b {
            #[inline]
            fn from_cast(from: $a) -> Self {
                from as $b
            }
        }
    };
    ($a:ident, [$($b:ident),+]) => {
        $(
            impl_cast_between!($a, $b);
        )+
    };
    ($a:ident, $($b:ident),+) => {
        impl_cast_between!($a, [$($b),+]);
        impl_cast_between!($($b),+);
    }
}

impl_cast_between!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
