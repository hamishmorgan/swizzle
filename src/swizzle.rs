#[macro_export]
macro_rules! swizzle {
    (@{} $self:ident, $($field:ident),+, @{ ( $($_:ident)+ ) -> ( $($value:ident)+ ) }) => {
        paste::paste! {
            #[doc = "Get a new `" $self "` with the values swizzled: " [<$($value)+>]]
            #[must_use]
            #[inline]
            pub const fn [<$($value)+>](&self) -> Self {
                Self { $($field: self.$value),* }
            }
        }
    };

    (@ { $h:ident $($t:ident)* } $self:ident, $($field:ident),+, @{ ( $head:ident $($tail:ident)* ) -> ( $($a:ident)* ) }) => {
        swizzle!(@{ $($t)* } $self, $($field),+, @{ ( $($field)+ ) -> ( $($a)* $head ) } );
        swizzle!(@{ $h $($t)* } $self, $($field),+, @{ ( $($tail)* ) -> ( $($a)* ) } );
    };

    (@ { $h:ident $($t:ident)* } $self:ident, $($field:ident),+, @{ ( ) -> ( $($a:ident)* ) } ) => {};

    ($self:ident, $($field:ident),+ $(,)? ) => {
        paste::paste! {
            #[doc = "Functions to swizzle a `" $self "`."]
            impl $self {
                swizzle!(@ { $($field)+ } $self, $($field),+, @{ ( $($field)+ ) -> ( ) } );
            }
        }
    };
}

pub use swizzle;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swizzle_struct_1() {
        struct TestStruct {
            a: u8,
        }

        swizzle!(TestStruct, a);

        let s1 = TestStruct { a: 1 };
        let s1_a = s1.a();
        assert_eq!(s1_a.a, 1);
    }

    #[test]
    fn test_swizzle_struct_2() {
        struct TestStruct {
            a: u8,
            b: u8,
        }

        swizzle!(TestStruct, a, b);

        let s2 = TestStruct { a: 1, b: 2 };

        let s2_aa = s2.aa();
        assert_eq!(s2_aa.a, 1);
        assert_eq!(s2_aa.b, 1);

        let s2_bb = s2.bb();
        assert_eq!(s2_bb.a, 2);
        assert_eq!(s2_bb.b, 2);

        let s2_ab = s2.ab();
        assert_eq!(s2_ab.a, 1);
        assert_eq!(s2_ab.b, 2);

        let s2_ba = s2.ba();
        assert_eq!(s2_ba.a, 2);
        assert_eq!(s2_ba.b, 1);
    }

    #[test]
    fn test_swizzle_struct_3() {
        struct TestStruct {
            r: u8,
            g: u8,
            b: u8,
        }

        swizzle!(TestStruct, r, g, b);

        let s3 = TestStruct { r: 1, g: 2, b: 3 };
        let s3_rgb = s3.rgb();
        assert_eq!(s3_rgb.r, 1);
        assert_eq!(s3_rgb.g, 2);
        assert_eq!(s3_rgb.b, 3);

        let s3_bgr = s3.bgr();
        assert_eq!(s3_bgr.r, 3);
        assert_eq!(s3_bgr.g, 2);
        assert_eq!(s3_bgr.b, 1);
    }
}
