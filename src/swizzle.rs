/// Macro for generating swizzle functions of structs.
///
/// The `swizzle!` macro generates functions that rearranged the fields in any combination. Swizzles
/// can also be used to convert between different types, with different numbers/names of attributes.
/// This is particularly useful for mathematical operations, graphics programming, and data manipulation.
///
/// # How it works
///
/// The macro generates all possible combinations of field values. For a struct with `n` fields,
/// it creates `n^n` different swizzle functions. Each function returns a new instance of the struct
/// with the field values arranged according to the function name.
///
/// # Basic Usage
///
/// ```rust
/// use swizzle::swizzle;
///
/// struct MyStruct {
///     a: u8,
///     b: u8,
///     c: u8,
/// }
///
/// impl MyStruct {
///     swizzle!(MyStruct { a, b, c });
/// }
///
/// let s = MyStruct { a: 1, b: 2, c: 3 };
/// let s_swizzled = s.abc(); // MyStruct { a: 1, b: 2, c: 3 }
/// ```
///
/// # Parameters
///
/// - `StructName`: The name of the struct to implement swizzle functions for
/// - `field1, field2, ...`: The field names to generate swizzle combinations for
///
/// # Generated Functions
///
/// For each possible combination of field names, the macro generates a function named after
/// that combination. For example, with fields `a`, `b`, `c`:
///
/// - `aaa()` → returns struct with all fields set to the value of `a`
/// - `abc()` → returns struct with fields set to `a`, `b`, `c` respectively
/// - `cba()` → returns struct with fields set to `c`, `b`, `a` respectively
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use swizzle::swizzle;
///
/// struct Vec2 {
///     x: f32,
///     y: f32,
/// }
///
/// impl Vec2 {
///     swizzle!(Vec2 { x, y });
/// }
///
/// let v = Vec2 { x: 1.0, y: 2.0 };
/// let v_swapped = v.yx();  // Vec2 { x: 2.0, y: 1.0 }
/// let v_repeated = v.xx(); // Vec2 { x: 1.0, y: 1.0 }
/// ```
///
/// ## 3D Vector Example
///
/// ```rust
/// use swizzle::swizzle;
///
/// struct Vec3 {
///     x: f32,
///     y: f32,
///     z: f32,
/// }
///
/// impl Vec3 {
///     swizzle!(Vec3 { x, y, z });
/// }
///
/// let v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
/// let v_xy = v.xyy();      // Vec3 { x: 1.0, y: 2.0, z: 2.0 }
/// let v_reverse = v.zyx(); // Vec3 { x: 3.0, y: 2.0, z: 1.0 }
/// ```
///
/// ## Color Example
///
/// ```rust
/// use swizzle::swizzle;
///
/// struct Color {
///     r: u8,
///     g: u8,
///     b: u8,
///     a: u8,
/// }
///
/// impl Color {
///     swizzle!(Color { r, g, b, a });
/// }
///
/// let c = Color { r: 255, g: 128, b: 64, a: 255 };
/// let c_bgr = c.bgrb();    // Color { r: 64, g: 128, b: 255, a: 64 }
/// let c_grayscale = c.rrrr(); // Color { r: 255, g: 255, b: 255, a: 255 }
/// ```
///
/// # Performance
///
/// All generated functions are marked as `#[inline]` and `#[must_use]` for optimal performance.
/// The functions are const functions, allowing them to be used in const contexts.
///
/// # Limitations
///
/// - Field names must be valid Rust identifiers
/// - All fields must be of the same type or types that can be copied
/// - The macro generates a lot of functions for structs with many fields (n^n functions). It's extremely slow for n>5 though it doesn't stop you from trying.
///
/// # Use Cases
///
/// - **Graphics Programming**: Swizzling vector components (xy, yx, xyz, etc.)
/// - **Data Manipulation**: Reordering struct fields for different data views
/// - **Mathematical Operations**: Creating variations of mathematical objects
/// - **API Design**: Providing convenient access patterns for struct data
///
/// # Generated Function Naming Convention
///
/// Function names are created by concatenating the field names in the order specified.
/// For example, with fields `a`, `b`, `c`:
///
/// - `a` → `a()`
/// - `ab` → `ab()`
/// - `abc` → `abc()`
/// - `cba` → `cba()`
/// - `aaa` → `aaa()`
///
/// The function name directly corresponds to the field values that will be used
/// in the returned struct, in the same order.
#[macro_export]
macro_rules! swizzle {

    // Simple case to generate a single swizzle function. Also the terminal case for the more complex invocations.
    // ```
    // swizzle!(Vec2 {x: x, y: y}) =>
    //     pub const fn xx(&self) -> Vec2 { Vec2 { x: x, y: y } }
    // ```
    (
        $dst_type:path {
            $( $dst_attr:ident: $src_attr:ident ),*
            $(,)?
        }
    ) => {
        paste::paste! {
            #[doc = "Create an instance of `" ]
            #[doc = stringify!( $dst_type ) ]
            #[doc = "` with the values swizzled: ["  [< $($src_attr)+ >] "]" ]
            #[must_use]
            #[inline]
            pub const fn [< $($src_attr)+ >](&self) -> $dst_type {
                $dst_type { $($dst_attr: self.$src_attr),* }
            }
        }
    };

    // Case for a swizzle function that creates new instances of it's own type with all
    // combinations of attributes.
    // ```
    // swizzle!(Vec2 {x, y}) =>
    //     pub const fn xx(&self) -> Vec2 { Vec2 { x: self.x, y: self.x } }
    //     pub const fn xy(&self) -> Vec2 { Vec2 { x: self.x, y: self.y } }
    //     pub const fn yx(&self) -> Vec2 { Vec2 { x: self.y, y: self.x } }
    //     pub const fn yy(&self) -> Vec2 { Vec2 { x: self.y, y: self.y } }
    // ```
    (
        $dst_type:path {
            $( $attr:ident ),*
            $(,)?
        }
    ) => {
        // Before we can generate the swizzle functions we need to build out the list of source
        // attributes, which is a copy of the destination attributes for each destination
        // attribute.
        swizzle!(
            $dst_type;
            @bld { $( $attr ),* }
            @src { }
            @dst { $( $attr ),* }
            @out { }
        );
    };

    // Recursive case for building out the list of source attributes.
    (
        $dst_type:path;
        @bld { $head:ident $(, $tail:ident )* $(,)? }
        @src { $( ( $( $src_attr:ident ),* ) ),* $(,)? }
        @dst { $( $dst_attr:ident ),* $(,)? }
        @out { }
    ) => {
        // Recurse on self with @bld reduced by one and @src extended by one set of @dst attributes.
        swizzle!(
            $dst_type;
            @bld { $( $tail ),* }
            @src {
                    ( $( $dst_attr ),* )
                $(, ( $( $src_attr ),* ) )*
            }
            @dst { $( $dst_attr ),* }
            @out { }
        );
    };

    // Terminal case for building out the list of source attributes. @bld is empty.
    (
        $dst_type:path;
        @bld { $(,)? }
        @src { $( ( $( $src_attr:ident ),* ) ),* $(,)? }
        @dst { $( $dst_attr:ident ),* $(,)? }
        @out { }
    ) => {
        // Call the main generation function with the final lists.
        swizzle!(
            $dst_type;
            @src { $( ( $( $src_attr ),* ) ),*} ;
            @dst { $( $dst_attr ),* } ;
            @out { };
        );
    };

    // Case for generating multiple swizzle functions where the destination type is created with
    // attributes set to all combinations of the given source attributes. This method can be used
    // for converting between different types, with different numbers/names of attributes.
    // ```
    // impl Vec3 {
    //   swizzle!(Vec2 {x: (x, y, z), y: (x, y, z)}) =>
    //     pub const fn xx(&self) -> Vec2 { Vec2 { x: self.x, y: self.x } }
    //     pub const fn xy(&self) -> Vec2 { Vec2 { x: self.x, y: self.y } }
    //     pub const fn xz(&self) -> Vec2 { Vec2 { x: self.x, y: self.z } }
    //     pub const fn yx(&self) -> Vec2 { Vec2 { x: self.y, y: self.x } }
    //     pub const fn yy(&self) -> Vec2 { Vec2 { x: self.y, y: self.y } }
    //     pub const fn yz(&self) -> Vec2 { Vec2 { x: self.y, y: self.z } }
    //     pub const fn zx(&self) -> Vec2 { Vec2 { x: self.z, y: self.x } }
    //     pub const fn zy(&self) -> Vec2 { Vec2 { x: self.z, y: self.y } }
    //     pub const fn zz(&self) -> Vec2 { Vec2 { x: self.z, y: self.z } }
    // }
    // ```
    (
        $dst_type:path {
            $(
                $dst_attr:ident: (
                    $( $src_attr:ident ),+
                    $(,)?
                )
            ),+
            $(,)?
        }
    ) => {
        // Reorganize the parameters in the form necessary for the main generation function.
        swizzle!(
            $dst_type;
            @src { $( ( $($src_attr),+ ) ),+ };
            @dst { $( $dst_attr ),+ };
            @out { };
        );
    };

    // Main recursive case for generating the swizzle functions.
    (
        $dst_type:path;
        @src{
                (
                    $src_attr_head_head:ident
                    $( , $src_attr_head_tail:ident )* $(,)?
                )
            $(, ( $( $src_attr_tail:ident ),+ $(,)? ) )*
            $(,)?
        };
        @dst{
                $dst_attr_head:ident
            $(, $dst_attr_tail:ident )*
            $(,)?
        };
        @out{
            $( $out_dst:ident: $out_src:ident ),*
            $(,)?
        };
    ) => {
        swizzle!(
            $dst_type;
            @src {
                ( $($src_attr_head_tail),* )
                $( , ( $( $src_attr_tail ),* ) )*
            };
            @dst {
                $dst_attr_head
                $( , $dst_attr_tail )*
            };
            @out {
                $( $out_dst: $out_src , )*
            };
        );
        swizzle!(
            $dst_type;
            @src {
                $( ( $( $src_attr_tail ),* ) ),*
            };
            @dst {
                $( $dst_attr_tail ),*
            };
            @out {
                $( $out_dst: $out_src , )*
                $dst_attr_head: $src_attr_head_head
            };
        );
    };

    // Terminal case for when a list of source attributes is empty.
    (
        $dst_type:path;
        @src{
            ( )
            $(, ( $( $src_attr_tail:ident ),+ $(,)? ) )*
            $(,)?
        };
        @dst{
                $dst_attr_head:ident
            $(, $dst_attr_tail:ident )*
            $(,)?
        };
        @out{
            $( $out_dst:ident: $out_src:ident ),*
            $(,)?
        };
    ) => {
    };


    // Terminal generation case.
    (
        $dst_type:path;
        @src{ $(,)? };
        @dst{ $(,)? };
        @out{ $( $out_dst:ident: $out_src:ident ),+ $(,)? };
    ) => {
        swizzle!($dst_type { $( $out_dst: $out_src ),+ });
    };

}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use core::f32;

    #[test]
    fn test_swizzle_struct_1_field() {
        struct TestStruct {
            a: u8,
        }

        impl TestStruct {
            swizzle!(TestStruct { a });
        }

        let s1 = TestStruct { a: 1 };
        let s1_a = s1.a();
        assert_eq!(s1_a.a, 1);
    }

    #[test]
    fn test_swizzle_struct_2_fields() {
        #[derive(Debug, PartialEq)]
        struct TestStruct {
            a: u8,
            b: u8,
        }

        impl TestStruct {
            swizzle!(TestStruct { a, b });
        }

        let s2 = TestStruct { a: 1, b: 2 };

        let aa = s2.aa();
        assert_eq!(aa.a, 1);
        assert_eq!(aa.b, 1);

        let bb = s2.bb();
        assert_eq!(bb.a, 2);
        assert_eq!(bb.b, 2);

        let ab = s2.ab();
        assert_eq!(ab.a, 1);
        assert_eq!(ab.b, 2);

        let ba = s2.ba();
        assert_eq!(ba.a, 2);
        assert_eq!(ba.b, 1);

        assert_eq!(s2.ba().ba(), s2);
    }

    #[test]
    fn test_swizzle_struct_3_fields() {
        struct TestStruct {
            a: u8,
            b: u8,
            c: u8,
        }

        impl TestStruct {
            swizzle!(TestStruct { a, b, c });
        }

        let s3 = TestStruct { a: 1, b: 2, c: 3 };

        let aaa = s3.aaa();
        assert_eq!((aaa.a, aaa.b, aaa.c), (1, 1, 1));

        let aab = s3.aab();
        assert_eq!((aab.a, aab.b, aab.c), (1, 1, 2));

        let aac = s3.aac();
        assert_eq!((aac.a, aac.b, aac.c), (1, 1, 3));

        let aba = s3.aba();
        assert_eq!((aba.a, aba.b, aba.c), (1, 2, 1));

        let abb = s3.abb();
        assert_eq!((abb.a, abb.b, abb.c), (1, 2, 2));

        let abc = s3.abc();
        assert_eq!((abc.a, abc.b, abc.c), (1, 2, 3));

        let aca = s3.aca();
        assert_eq!((aca.a, aca.b, aca.c), (1, 3, 1));

        let acb = s3.acb();
        assert_eq!((acb.a, acb.b, acb.c), (1, 3, 2));

        let acc = s3.acc();
        assert_eq!((acc.a, acc.b, acc.c), (1, 3, 3));

        let baa = s3.baa();
        assert_eq!((baa.a, baa.b, baa.c), (2, 1, 1));

        let bab = s3.bab();
        assert_eq!((bab.a, bab.b, bab.c), (2, 1, 2));

        let bac = s3.bac();
        assert_eq!((bac.a, bac.b, bac.c), (2, 1, 3));

        let bba = s3.bba();
        assert_eq!((bba.a, bba.b, bba.c), (2, 2, 1));

        let bbb = s3.bbb();
        assert_eq!((bbb.a, bbb.b, bbb.c), (2, 2, 2));

        let bbc = s3.bbc();
        assert_eq!((bbc.a, bbc.b, bbc.c), (2, 2, 3));

        let bca = s3.bca();
        assert_eq!((bca.a, bca.b, bca.c), (2, 3, 1));

        let bcb = s3.bcb();
        assert_eq!((bcb.a, bcb.b, bcb.c), (2, 3, 2));

        let bcc = s3.bcc();
        assert_eq!((bcc.a, bcc.b, bcc.c), (2, 3, 3));

        let caa = s3.caa();
        assert_eq!((caa.a, caa.b, caa.c), (3, 1, 1));

        let cab = s3.cab();
        assert_eq!((cab.a, cab.b, cab.c), (3, 1, 2));

        let cac = s3.cac();
        assert_eq!((cac.a, cac.b, cac.c), (3, 1, 3));

        let cba = s3.cba();
        assert_eq!((cba.a, cba.b, cba.c), (3, 2, 1));

        let cbb = s3.cbb();
        assert_eq!((cbb.a, cbb.b, cbb.c), (3, 2, 2));

        let cbc = s3.cbc();
        assert_eq!((cbc.a, cbc.b, cbc.c), (3, 2, 3));

        let cca = s3.cca();
        assert_eq!((cca.a, cca.b, cca.c), (3, 3, 1));

        let ccb = s3.ccb();
        assert_eq!((ccb.a, ccb.b, ccb.c), (3, 3, 2));

        let ccc = s3.ccc();
        assert_eq!((ccc.a, ccc.b, ccc.c), (3, 3, 3));
    }

    #[test]
    fn test_swizzle_struct_4_fields() {
        struct TestStruct {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        impl TestStruct {
            swizzle!(TestStruct { a, b, c, d });
        }

        let s4 = TestStruct {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
        };

        // Test some key combinations (testing all 256 would be excessive)

        let aaaa = s4.aaaa();
        assert_eq!((aaaa.a, aaaa.b, aaaa.c, aaaa.d), (1, 1, 1, 1));

        let abcd = s4.abcd();
        assert_eq!((abcd.a, abcd.b, abcd.c, abcd.d), (1, 2, 3, 4));

        let dcba = s4.dcba();
        assert_eq!((dcba.a, dcba.b, dcba.c, dcba.d), (4, 3, 2, 1));

        let aabb = s4.aabb();
        assert_eq!((aabb.a, aabb.b, aabb.c, aabb.d), (1, 1, 2, 2));

        let abab = s4.abab();
        assert_eq!((abab.a, abab.b, abab.c, abab.d), (1, 2, 1, 2));

        let baba = s4.baba();
        assert_eq!((baba.a, baba.b, baba.c, baba.d), (2, 1, 2, 1));

        let cccc = s4.cccc();
        assert_eq!((cccc.a, cccc.b, cccc.c, cccc.d), (3, 3, 3, 3));

        let dddd = s4.dddd();
        assert_eq!((dddd.a, dddd.b, dddd.c, dddd.d), (4, 4, 4, 4));

        let aabc = s4.aabc();
        assert_eq!((aabc.a, aabc.b, aabc.c, aabc.d), (1, 1, 2, 3));

        let bcda = s4.bcda();
        assert_eq!((bcda.a, bcda.b, bcda.c, bcda.d), (2, 3, 4, 1));

        let cdab = s4.cdab();
        assert_eq!((cdab.a, cdab.b, cdab.c, cdab.d), (3, 4, 1, 2));

        let dabc = s4.dabc();
        assert_eq!((dabc.a, dabc.b, dabc.c, dabc.d), (4, 1, 2, 3));

        let abba = s4.abba();
        assert_eq!((abba.a, abba.b, abba.c, abba.d), (1, 2, 2, 1));

        let acca = s4.acca();
        assert_eq!((acca.a, acca.b, acca.c, acca.d), (1, 3, 3, 1));

        let bccb = s4.bccb();
        assert_eq!((bccb.a, bccb.b, bccb.c, bccb.d), (2, 3, 3, 2));

        let adda = s4.adda();
        assert_eq!((adda.a, adda.b, adda.c, adda.d), (1, 4, 4, 1));
    }

    #[test]
    fn test_swizzle_struct_with_different_field_names() {
        struct CustomStruct {
            first: u8,
            second: u8,
            third: u8,
        }

        impl CustomStruct {
            swizzle!(CustomStruct {
                first,
                second,
                third
            });
        }

        let s = CustomStruct {
            first: 10,
            second: 20,
            third: 30,
        };

        let fff = s.firstfirstfirst();
        assert_eq!(fff.first, 10);
        assert_eq!(fff.second, 10);
        assert_eq!(fff.third, 10);

        let fst = s.firstsecondthird();
        assert_eq!(fst.first, 10);
        assert_eq!(fst.second, 20);
        assert_eq!(fst.third, 30);

        let tsf = s.thirdsecondfirst();
        assert_eq!(tsf.first, 30);
        assert_eq!(tsf.second, 20);
        assert_eq!(tsf.third, 10);
    }

    #[test]
    fn test_swizzle_struct_5_fields() {
        struct TestStruct {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
            e: u8,
        }

        impl TestStruct {
            swizzle!(TestStruct { a, b, c, d, e });
        }

        let s5 = TestStruct {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
            e: 5,
        };

        let aaaaa = s5.aaaaa();
        assert_eq!(
            (aaaaa.a, aaaaa.b, aaaaa.c, aaaaa.d, aaaaa.e),
            (1, 1, 1, 1, 1)
        );

        // Test sequential patterns
        let abcde = s5.abcde();
        assert_eq!(
            (abcde.a, abcde.b, abcde.c, abcde.d, abcde.e),
            (1, 2, 3, 4, 5)
        );

        let edcba = s5.edcba();
        assert_eq!(
            (edcba.a, edcba.b, edcba.c, edcba.d, edcba.e),
            (5, 4, 3, 2, 1)
        );

        // Test repeated value patterns
        let bbbbb = s5.bbbbb();
        assert_eq!(
            (bbbbb.a, bbbbb.b, bbbbb.c, bbbbb.d, bbbbb.e),
            (2, 2, 2, 2, 2)
        );

        let ccccc = s5.ccccc();
        assert_eq!(
            (ccccc.a, ccccc.b, ccccc.c, ccccc.d, ccccc.e),
            (3, 3, 3, 3, 3)
        );

        // Test mixed patterns
        let aabcc = s5.aabcc();
        assert_eq!(
            (aabcc.a, aabcc.b, aabcc.c, aabcc.d, aabcc.e),
            (1, 1, 2, 3, 3)
        );

        let abcdd = s5.abcdd();
        assert_eq!(
            (abcdd.a, abcdd.b, abcdd.c, abcdd.d, abcdd.e),
            (1, 2, 3, 4, 4)
        );

        let aabbb = s5.aabbb();
        assert_eq!(
            (aabbb.a, aabbb.b, aabbb.c, aabbb.d, aabbb.e),
            (1, 1, 2, 2, 2)
        );

        // Test alternating patterns
        let ababa = s5.ababa();
        assert_eq!(
            (ababa.a, ababa.b, ababa.c, ababa.d, ababa.e),
            (1, 2, 1, 2, 1)
        );

        let babab = s5.babab();
        assert_eq!(
            (babab.a, babab.b, babab.c, babab.d, babab.e),
            (2, 1, 2, 1, 2)
        );

        // Test circular shift patterns
        let bcdea = s5.bcdea();
        assert_eq!(
            (bcdea.a, bcdea.b, bcdea.c, bcdea.d, bcdea.e),
            (2, 3, 4, 5, 1)
        );

        let cdeab = s5.cdeab();
        assert_eq!(
            (cdeab.a, cdeab.b, cdeab.c, cdeab.d, cdeab.e),
            (3, 4, 5, 1, 2)
        );

        // Test edge cases
        let eeeee = s5.eeeee();
        assert_eq!(
            (eeeee.a, eeeee.b, eeeee.c, eeeee.d, eeeee.e),
            (5, 5, 5, 5, 5)
        );

        let ddddd = s5.ddddd();
        assert_eq!(
            (ddddd.a, ddddd.b, ddddd.c, ddddd.d, ddddd.e),
            (4, 4, 4, 4, 4)
        );
    }

    #[test]
    fn test_swizzle_with_str_field_types() {
        struct TestStruct {
            a: &'static str,
            b: &'static str,
        }

        impl TestStruct {
            swizzle!(TestStruct { a, b });
        }

        let s = TestStruct { a: "a", b: "b" };

        let ab = s.ab();
        assert_eq!((ab.a, ab.b), ("a", "b"));

        let ba = s.ba();
        assert_eq!((ba.a, ba.b), ("b", "a"));

        let aa = s.aa();
        assert_eq!((aa.a, aa.b), ("a", "a"));

        let bb = s.bb();
        assert_eq!((bb.a, bb.b), ("b", "b"));
    }

    #[test]
    fn test_swizzle_with_f64_field_types() {
        struct TestStruct {
            a: f64,
            b: f64,
        }

        impl TestStruct {
            swizzle!(TestStruct { a, b });
        }

        let s = TestStruct { a: 1.0, b: 2.0 };

        let ab = s.ab();
        assert_eq!((ab.a, ab.b), (1.0, 2.0));

        let ba = s.ba();
        assert_eq!((ba.a, ba.b), (2.0, 1.0));

        let aa = s.aa();
        assert_eq!((aa.a, aa.b), (1.0, 1.0));

        let bb = s.bb();
        assert_eq!((bb.a, bb.b), (2.0, 2.0));
    }

    pub mod fixtures {
        pub struct TestStruct {
            pub a: u8,
            pub b: u8,
        }
    }

    #[test]
    fn test_swizzle_path_to_self() {
        #[allow(non_local_definitions)]
        impl fixtures::TestStruct {
            swizzle!(fixtures::TestStruct { a, b });
        }

        let s = fixtures::TestStruct { a: 1, b: 2 };

        let ab = s.ab();
        assert_eq!((ab.a, ab.b), (1, 2));
    }

    // Type conversion tests - converting between different structs with different field counts
    #[test]
    fn test_swizzle_type_conversion_scalar_to_vec2() {
        struct Scalar {
            x: f32,
        }

        struct Vec2 {
            x: f32,
            y: f32,
        }

        impl Scalar {
            swizzle!(Vec2 { x: (x), y: (x) });
        }

        let scalar = Scalar { x: 5.0 };
        let vec2 = scalar.xx();

        assert_eq!(vec2.x, 5.0);
        assert_eq!(vec2.y, 5.0);
    }

    #[test]
    fn test_swizzle_type_conversion_scalar_to_vec3() {
        struct Scalar {
            x: f32,
        }

        struct Vec3 {
            x: f32,
            y: f32,
            z: f32,
        }

        impl Scalar {
            swizzle!(Vec3 {
                x: (x),
                y: (x),
                z: (x)
            });
        }

        let scalar = Scalar { x: 3.0 };
        let vec3 = scalar.xxx();

        assert_eq!(vec3.x, 3.0);
        assert_eq!(vec3.y, 3.0);
        assert_eq!(vec3.z, 3.0);
    }

    #[test]
    fn test_swizzle_type_conversion_vec2_to_scalar() {
        struct Vec2 {
            x: f32,
            y: f32,
        }

        struct Scalar {
            x: f32,
        }

        impl Vec2 {
            swizzle!(Scalar { x: (x, y) });
        }

        let vec2 = Vec2 { x: 1.0, y: 2.0 };

        let scalar_x = vec2.x();
        assert_eq!(scalar_x.x, 1.0);

        let scalar_y = vec2.y();
        assert_eq!(scalar_y.x, 2.0);
    }

    #[test]
    fn test_swizzle_type_conversion_vec2_to_vec3() {
        struct Vec2 {
            x: f32,
            y: f32,
        }

        struct Vec3 {
            x: f32,
            y: f32,
            z: f32,
        }

        impl Vec2 {
            swizzle!(Vec3 {
                x: (x, y),
                y: (x, y),
                z: (x, y)
            });
        }

        let vec2 = Vec2 { x: 1.0, y: 2.0 };

        // Test that we can create Vec3 from Vec2 using swizzle
        // The macro generates methods like xxx(), xxy(), etc.
        let vec3_xxx = vec2.xxx();
        assert_eq!(vec3_xxx.x, 1.0);
        assert_eq!(vec3_xxx.y, 1.0);
        assert_eq!(vec3_xxx.z, 1.0);

        let vec3_xxy = vec2.xxy();
        assert_eq!(vec3_xxy.x, 1.0);
        assert_eq!(vec3_xxy.y, 1.0);
        assert_eq!(vec3_xxy.z, 2.0);

        let vec3_xyx = vec2.xyx();
        assert_eq!(vec3_xyx.x, 1.0);
        assert_eq!(vec3_xyx.y, 2.0);
        assert_eq!(vec3_xyx.z, 1.0);

        let vec3_xyy = vec2.xyy();
        assert_eq!(vec3_xyy.x, 1.0);
        assert_eq!(vec3_xyy.y, 2.0);
        assert_eq!(vec3_xyy.z, 2.0);
    }

    #[test]
    fn test_swizzle_type_conversion_mixed_types() {
        struct Source {
            a: u8,
            b: f32,
            c: &'static str,
        }

        struct Dest {
            x: u8,
            y: f32,
            z: &'static str,
        }

        impl Source {
            swizzle!(Dest {
                x: (a),
                y: (b),
                z: (c)
            });
        }

        let source = Source {
            a: 42,
            b: f32::consts::PI,
            c: "hello",
        };
        let dest = source.abc();

        assert_eq!(dest.x, 42);
        assert_eq!(dest.y, f32::consts::PI);
        assert_eq!(dest.z, "hello");
    }

    #[test]
    fn test_swizzle_type_conversion_different_field_counts() {
        struct Vec4 {
            x: f32,
            y: f32,
            z: f32,
            w: f32,
        }

        struct Vec2 {
            x: f32,
            y: f32,
        }

        impl Vec4 {
            swizzle!(Vec2 {
                x: (x, y, z, w),
                y: (x, y, z, w)
            });
        }

        let vec4 = Vec4 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };

        // Test some key combinations - the macro generates methods like xx(), xy(), etc.
        let vec2_xx = vec4.xx();
        assert_eq!(vec2_xx.x, 1.0);
        assert_eq!(vec2_xx.y, 1.0);

        let vec2_xy = vec4.xy();
        assert_eq!(vec2_xy.x, 1.0);
        assert_eq!(vec2_xy.y, 2.0);

        let vec2_zw = vec4.zw();
        assert_eq!(vec2_zw.x, 3.0);
        assert_eq!(vec2_zw.y, 4.0);
    }

    #[test]
    fn test_swizzle_type_conversion_with_self_swizzle() {
        struct Vec3 {
            x: f32,
            y: f32,
            z: f32,
        }

        struct Vec2 {
            x: f32,
            y: f32,
        }

        impl Vec3 {
            // Self-swizzle methods
            swizzle!(Vec3 { x, y, z });
            // Conversion to Vec2
            swizzle!(Vec2 {
                x: (x, y, z),
                y: (x, y, z)
            });
        }

        let vec3 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        // Test self-swizzle
        let vec3_zyx = vec3.zyx();
        assert_eq!((vec3_zyx.x, vec3_zyx.y, vec3_zyx.z), (3.0, 2.0, 1.0));

        // Test conversion to Vec2
        let vec2_xy = vec3.xy();
        assert_eq!((vec2_xy.x, vec2_xy.y), (1.0, 2.0));
    }

    #[test]
    fn test_swizzle_type_conversion_edge_cases() {
        struct Single {
            x: u8,
        }

        struct Double {
            a: u8,
            b: u8,
        }

        impl Single {
            swizzle!(Double { a: (x), b: (x) });
        }

        let single = Single { x: 255 };
        let double = single.xx();

        assert_eq!(double.a, 255);
        assert_eq!(double.b, 255);
    }

    #[test]
    fn test_swizzle_type_conversion_large_to_small() {
        struct Vec5 {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
            e: u8,
        }

        struct Vec2 {
            x: u8,
            y: u8,
        }

        impl Vec5 {
            swizzle!(Vec2 {
                x: (a, b, c, d, e),
                y: (a, b, c, d, e)
            });
        }

        let vec5 = Vec5 {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
            e: 5,
        };

        let vec2_aa = vec5.aa();
        assert_eq!((vec2_aa.x, vec2_aa.y), (1, 1));

        let vec2_ab = vec5.ab();
        assert_eq!((vec2_ab.x, vec2_ab.y), (1, 2));

        let vec2_ee = vec5.ee();
        assert_eq!((vec2_ee.x, vec2_ee.y), (5, 5));
    }

    #[test]
    fn test_swizzle_type_conversion_with_custom_types() {
        #[derive(Debug, PartialEq)]
        struct Color {
            r: u8,
            g: u8,
            b: u8,
        }

        #[derive(Debug, PartialEq)]
        struct Grayscale {
            value: u8,
        }

        impl Color {
            swizzle!(Grayscale { value: (r, g, b) });
        }

        let color = Color {
            r: 100,
            g: 150,
            b: 200,
        };

        let gray_r = color.r();
        assert_eq!(gray_r.value, 100);

        let gray_g = color.g();
        assert_eq!(gray_g.value, 150);

        let gray_b = color.b();
        assert_eq!(gray_b.value, 200);
    }

    #[test]
    fn test_swizzle_type_conversion_chain() {
        struct Vec1 {
            x: f32,
        }

        struct Vec2 {
            x: f32,
            y: f32,
        }

        struct Vec3 {
            x: f32,
            y: f32,
            z: f32,
        }

        impl Vec1 {
            swizzle!(Vec2 { x: (x), y: (x) });
        }

        impl Vec2 {
            swizzle!(Vec3 {
                x: (x, y),
                y: (x, y),
                z: (x, y)
            });
        }

        let vec1 = Vec1 { x: 42.0 };
        let vec2 = vec1.xx();
        let vec3 = vec2.xxx();

        assert_eq!(vec3.x, 42.0);
        assert_eq!(vec3.y, 42.0);
        assert_eq!(vec3.z, 42.0);
    }

    // Additional comprehensive tests for edge cases and complex scenarios
    #[test]
    fn test_swizzle_with_boolean_types() {
        struct BoolStruct {
            a: bool,
            b: bool,
        }

        impl BoolStruct {
            swizzle!(BoolStruct { a, b });
        }

        let bool_struct = BoolStruct { a: true, b: false };

        let aa = bool_struct.aa();
        assert!(aa.a);
        assert!(aa.b);

        let bb = bool_struct.bb();
        assert!(!bb.a);
        assert!(!bb.b);

        let ab = bool_struct.ab();
        assert!(ab.a);
        assert!(!ab.b);

        let ba = bool_struct.ba();
        assert!(!ba.a);
        assert!(ba.b);
    }

    #[test]
    fn test_swizzle_with_option_types() {
        struct OptionStruct {
            a: Option<u8>,
            b: Option<u8>,
        }

        impl OptionStruct {
            swizzle!(OptionStruct { a, b });
        }

        let opt_struct = OptionStruct {
            a: Some(42),
            b: None,
        };

        let aa = opt_struct.aa();
        assert_eq!(aa.a, Some(42));
        assert_eq!(aa.b, Some(42));

        let bb = opt_struct.bb();
        assert_eq!(bb.a, None);
        assert_eq!(bb.b, None);

        let ab = opt_struct.ab();
        assert_eq!(ab.a, Some(42));
        assert_eq!(ab.b, None);

        let ba = opt_struct.ba();
        assert_eq!(ba.a, None);
        assert_eq!(ba.b, Some(42));
    }

    #[test]
    fn test_swizzle_with_reference_types() {
        struct RefStruct<'a> {
            a: &'a str,
            b: &'a str,
        }

        impl<'a> RefStruct<'a> {
            swizzle!(RefStruct<'a> { a, b });
        }

        let s1 = "hello";
        let s2 = "world";
        let ref_struct = RefStruct { a: s1, b: s2 };

        let aa = ref_struct.aa();
        assert_eq!(aa.a, "hello");
        assert_eq!(aa.b, "hello");

        let bb = ref_struct.bb();
        assert_eq!(bb.a, "world");
        assert_eq!(bb.b, "world");

        let ab = ref_struct.ab();
        assert_eq!(ab.a, "hello");
        assert_eq!(ab.b, "world");

        let ba = ref_struct.ba();
        assert_eq!(ba.a, "world");
        assert_eq!(ba.b, "hello");
    }

    #[test]
    fn test_swizzle_with_array_types() {
        struct ArrayStruct {
            a: [u8; 2],
            b: [u8; 2],
        }

        impl ArrayStruct {
            swizzle!(ArrayStruct { a, b });
        }

        let array_struct = ArrayStruct {
            a: [1, 2],
            b: [3, 4],
        };

        let aa = array_struct.aa();
        assert_eq!(aa.a, [1, 2]);
        assert_eq!(aa.b, [1, 2]);

        let bb = array_struct.bb();
        assert_eq!(bb.a, [3, 4]);
        assert_eq!(bb.b, [3, 4]);

        let ab = array_struct.ab();
        assert_eq!(ab.a, [1, 2]);
        assert_eq!(ab.b, [3, 4]);

        let ba = array_struct.ba();
        assert_eq!(ba.a, [3, 4]);
        assert_eq!(ba.b, [1, 2]);
    }

    #[test]
    fn test_swizzle_with_tuple_types() {
        struct TupleStruct {
            a: (u8, u8),
            b: (u8, u8),
        }

        impl TupleStruct {
            swizzle!(TupleStruct { a, b });
        }

        let tuple_struct = TupleStruct {
            a: (1, 2),
            b: (3, 4),
        };

        let aa = tuple_struct.aa();
        assert_eq!(aa.a, (1, 2));
        assert_eq!(aa.b, (1, 2));

        let bb = tuple_struct.bb();
        assert_eq!(bb.a, (3, 4));
        assert_eq!(bb.b, (3, 4));

        let ab = tuple_struct.ab();
        assert_eq!(ab.a, (1, 2));
        assert_eq!(ab.b, (3, 4));

        let ba = tuple_struct.ba();
        assert_eq!(ba.a, (3, 4));
        assert_eq!(ba.b, (1, 2));
    }

    #[test]
    fn test_swizzle_with_nested_structs() {
        #[derive(Debug, PartialEq, Copy, Clone)]
        struct Inner {
            x: u8,
            y: u8,
        }

        #[derive(Debug, PartialEq)]
        struct Outer {
            a: Inner,
            b: Inner,
        }

        impl Outer {
            swizzle!(Outer { a, b });
        }

        let outer = Outer {
            a: Inner { x: 1, y: 2 },
            b: Inner { x: 3, y: 4 },
        };

        let aa = outer.aa();
        assert_eq!(aa.a, Inner { x: 1, y: 2 });
        assert_eq!(aa.b, Inner { x: 1, y: 2 });

        let bb = outer.bb();
        assert_eq!(bb.a, Inner { x: 3, y: 4 });
        assert_eq!(bb.b, Inner { x: 3, y: 4 });

        let ab = outer.ab();
        assert_eq!(ab.a, Inner { x: 1, y: 2 });
        assert_eq!(ab.b, Inner { x: 3, y: 4 });

        let ba = outer.ba();
        assert_eq!(ba.a, Inner { x: 3, y: 4 });
        assert_eq!(ba.b, Inner { x: 1, y: 2 });
    }

    #[test]
    fn test_swizzle_with_generic_types() {
        struct GenericStruct<T> {
            a: T,
            b: T,
        }

        impl<T: Copy> GenericStruct<T> {
            swizzle!(GenericStruct<T> { a, b });
        }

        let generic_u8 = GenericStruct { a: 1u8, b: 2u8 };
        let aa_u8 = generic_u8.aa();
        assert_eq!(aa_u8.a, 1);
        assert_eq!(aa_u8.b, 1);

        let generic_f32 = GenericStruct {
            a: 1.0f32,
            b: 2.0f32,
        };
        let bb_f32 = generic_f32.bb();
        assert_eq!(bb_f32.a, 2.0);
        assert_eq!(bb_f32.b, 2.0);
    }

    #[test]
    fn test_swizzle_with_const_context() {
        struct ConstStruct {
            a: u8,
            b: u8,
        }

        impl ConstStruct {
            swizzle!(ConstStruct { a, b });
        }

        const fn create_swizzled() -> ConstStruct {
            let s = ConstStruct { a: 1, b: 2 };
            s.aa()
        }

        const RESULT: ConstStruct = create_swizzled();
        assert_eq!(RESULT.a, 1);
        assert_eq!(RESULT.b, 1);
    }

    #[test]
    fn test_swizzle_with_method_chaining() {
        struct ChainStruct {
            a: u8,
            b: u8,
            c: u8,
        }

        impl ChainStruct {
            swizzle!(ChainStruct { a, b, c });
        }

        let chain = ChainStruct { a: 1, b: 2, c: 3 };

        // Test method chaining
        let result = chain.abc().cba().bac();
        assert_eq!((result.a, result.b, result.c), (2, 3, 1));
    }

    #[test]
    fn test_swizzle_with_zero_sized_types() {
        #[derive(Copy, Clone)]
        struct ZeroSized;

        struct ZeroStruct {
            a: ZeroSized,
            b: ZeroSized,
        }

        impl ZeroStruct {
            swizzle!(ZeroStruct { a, b });
        }

        let zero_struct = ZeroStruct {
            a: ZeroSized,
            b: ZeroSized,
        };

        let aa = zero_struct.aa();
        let bb = zero_struct.bb();
        let ab = zero_struct.ab();
        let ba = zero_struct.ba();

        // All should work with zero-sized types
        assert_eq!(std::mem::size_of_val(&aa), 0);
        assert_eq!(std::mem::size_of_val(&bb), 0);
        assert_eq!(std::mem::size_of_val(&ab), 0);
        assert_eq!(std::mem::size_of_val(&ba), 0);
    }

    #[test]
    fn test_swizzle_with_copy_and_clone() {
        #[derive(Debug, PartialEq, Clone, Copy)]
        struct CopyStruct {
            a: u8,
            b: u8,
        }

        impl CopyStruct {
            swizzle!(CopyStruct { a, b });
        }

        let copy_struct = CopyStruct { a: 1, b: 2 };

        let aa = copy_struct.aa();
        assert_eq!(aa.a, 1);
        assert_eq!(aa.b, 1);

        // Test that we can clone the result
        #[allow(clippy::clone_on_copy)]
        let aa_cloned = aa.clone();
        assert_eq!(aa_cloned.a, 1);
        assert_eq!(aa_cloned.b, 1);
    }

    // Additional edge case tests
    #[allow(clippy::unit_cmp)]
    #[test]
    fn test_swizzle_with_unit_types() {
        struct UnitStruct {
            a: (),
            b: (),
        }

        impl UnitStruct {
            swizzle!(UnitStruct { a, b });
        }

        let unit_struct = UnitStruct { a: (), b: () };

        let aa = unit_struct.aa();
        assert_eq!(aa.a, ());
        assert_eq!(aa.b, ());

        let bb = unit_struct.bb();
        assert_eq!(bb.a, ());
        assert_eq!(bb.b, ());

        let ab = unit_struct.ab();
        assert_eq!(ab.a, ());
        assert_eq!(ab.b, ());

        let ba = unit_struct.ba();
        assert_eq!(ba.a, ());
        assert_eq!(ba.b, ());
    }

    #[test]
    fn test_swizzle_with_char_types() {
        struct CharStruct {
            a: char,
            b: char,
        }

        impl CharStruct {
            swizzle!(CharStruct { a, b });
        }

        let char_struct = CharStruct { a: 'x', b: 'y' };

        let aa = char_struct.aa();
        assert_eq!(aa.a, 'x');
        assert_eq!(aa.b, 'x');

        let bb = char_struct.bb();
        assert_eq!(bb.a, 'y');
        assert_eq!(bb.b, 'y');

        let ab = char_struct.ab();
        assert_eq!(ab.a, 'x');
        assert_eq!(ab.b, 'y');

        let ba = char_struct.ba();
        assert_eq!(ba.a, 'y');
        assert_eq!(ba.b, 'x');
    }

    #[test]
    fn test_swizzle_with_enum_types() {
        #[derive(Debug, PartialEq, Copy, Clone)]
        enum TestEnum {
            A,
            B,
            C,
        }

        struct EnumStruct {
            a: TestEnum,
            b: TestEnum,
        }

        impl EnumStruct {
            swizzle!(EnumStruct { a, b });
        }

        let enum_struct = EnumStruct {
            a: TestEnum::A,
            b: TestEnum::B,
        };

        let aa = enum_struct.aa();
        assert_eq!(aa.a, TestEnum::A);
        assert_eq!(aa.b, TestEnum::A);

        let bb = enum_struct.bb();
        assert_eq!(bb.a, TestEnum::B);
        assert_eq!(bb.b, TestEnum::B);

        let ab = enum_struct.ab();
        assert_eq!(ab.a, TestEnum::A);
        assert_eq!(ab.b, TestEnum::B);

        let ba = enum_struct.ba();
        assert_eq!(ba.a, TestEnum::B);
        assert_eq!(ba.b, TestEnum::A);
    }

    #[test]
    fn test_swizzle_with_const_generics() {
        struct ArrayStruct<const N: usize> {
            a: [u8; N],
            b: [u8; N],
        }

        impl<const N: usize> ArrayStruct<N> {
            swizzle!(ArrayStruct<N> { a, b });
        }

        let array_struct = ArrayStruct {
            a: [1, 2],
            b: [3, 4],
        };

        let aa = array_struct.aa();
        assert_eq!(aa.a, [1, 2]);
        assert_eq!(aa.b, [1, 2]);

        let bb = array_struct.bb();
        assert_eq!(bb.a, [3, 4]);
        assert_eq!(bb.b, [3, 4]);

        let ab = array_struct.ab();
        assert_eq!(ab.a, [1, 2]);
        assert_eq!(ab.b, [3, 4]);

        let ba = array_struct.ba();
        assert_eq!(ba.a, [3, 4]);
        assert_eq!(ba.b, [1, 2]);
    }
}
