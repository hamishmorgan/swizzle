/// Macro for generating swizzle functions on structs.
///
/// The `swizzle!` macro automatically generates functions that allow you to create new instances
/// of a struct with field values rearranged according to any combination of the original fields.
/// This is particularly useful for mathematical operations, graphics programming, and data manipulation.
///
/// # How it works
///
/// The macro generates all possible combinations of field values. For a struct with `n` fields,
/// it creates `n^n` different swizzle functions. Each function returns a new instance of the struct
/// with the field values arranged according to the function name.
///
/// # Syntax
///
/// ```rust
/// use swizzle::swizzle;
///
/// struct MyStruct {
///     field1: u8,
///     field2: u8,
///     field3: u8,
/// }
///
/// swizzle!(MyStruct, field1, field2, field3);
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
/// swizzle!(Vec2, x, y);
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
/// swizzle!(Vec3, x, y, z);
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
/// swizzle!(Color, r, g, b, a);
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
    // Base case: when we have processed all fields, generate the actual function
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

    // Recursive case: process the head field and continue with the tail
    // This rule handles the recursive generation of all possible combinations
    (@ { $h:ident $($t:ident)* } $self:ident, $($field:ident),+, @{ ( $head:ident $($tail:ident)* ) -> ( $($a:ident)* ) }) => {
        swizzle!(@{ $($t)* } $self, $($field),+, @{ ( $($field)+ ) -> ( $($a)* $head ) } );
        swizzle!(@{ $h $($t)* } $self, $($field),+, @{ ( $($tail)* ) -> ( $($a)* ) } );
    };

    // Termination case: when no more fields to process, do nothing
    (@ { $h:ident $($t:ident)* } $self:ident, $($field:ident),+, @{ ( ) -> ( $($a:ident)* ) } ) => {};

    // Entry point: start the recursive generation process
    // This rule initiates the macro expansion and creates the impl block
    ($self:ident, $($field:ident),+ $(,)? ) => {
        paste::paste! {
            #[doc = "Functions to swizzle a `" $self "`."]
            impl $self {
                swizzle!(@ { $($field)+ } $self, $($field),+, @{ ( $($field)+ ) -> ( ) } );
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_swizzle_struct_1_field() {
        struct TestStruct {
            a: u8,
        }

        swizzle!(TestStruct, a);

        let s1 = TestStruct { a: 1 };
        let s1_a = s1.a();
        assert_eq!(s1_a.a, 1);
    }

    #[test]
    fn test_swizzle_struct_2_fields() {
        struct TestStruct {
            a: u8,
            b: u8,
        }

        swizzle!(TestStruct, a, b);

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
    }

    #[test]
    fn test_swizzle_struct_3_fields() {
        struct TestStruct {
            a: u8,
            b: u8,
            c: u8,
        }

        swizzle!(TestStruct, a, b, c);

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

        swizzle!(TestStruct, a, b, c, d);

        let s4 = TestStruct { a: 1, b: 2, c: 3, d: 4 };

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

        swizzle!(TestStruct, a, b, c, d, e);

        let s5 = TestStruct { a: 1, b: 2, c: 3, d: 4, e: 5 };

        let aaaaa = s5.aaaaa();
        assert_eq!((aaaaa.a, aaaaa.b, aaaaa.c, aaaaa.d, aaaaa.e), (1, 1, 1, 1, 1));

        // Test sequential patterns
        let abcde = s5.abcde();
        assert_eq!((abcde.a, abcde.b, abcde.c, abcde.d, abcde.e), (1, 2, 3, 4, 5));

        let edcba = s5.edcba();
        assert_eq!((edcba.a, edcba.b, edcba.c, edcba.d, edcba.e), (5, 4, 3, 2, 1));

        // Test repeated value patterns
        let bbbbb = s5.bbbbb();
        assert_eq!((bbbbb.a, bbbbb.b, bbbbb.c, bbbbb.d, bbbbb.e), (2, 2, 2, 2, 2));

        let ccccc = s5.ccccc();
        assert_eq!((ccccc.a, ccccc.b, ccccc.c, ccccc.d, ccccc.e), (3, 3, 3, 3, 3));

        // Test mixed patterns
        let aabcc = s5.aabcc();
        assert_eq!((aabcc.a, aabcc.b, aabcc.c, aabcc.d, aabcc.e), (1, 1, 2, 3, 3));

        let abcdd = s5.abcdd();
        assert_eq!((abcdd.a, abcdd.b, abcdd.c, abcdd.d, abcdd.e), (1, 2, 3, 4, 4));

        let aabbb = s5.aabbb();
        assert_eq!((aabbb.a, aabbb.b, aabbb.c, aabbb.d, aabbb.e), (1, 1, 2, 2, 2));

        // Test alternating patterns
        let ababa = s5.ababa();
        assert_eq!((ababa.a, ababa.b, ababa.c, ababa.d, ababa.e), (1, 2, 1, 2, 1));

        let babab = s5.babab();
        assert_eq!((babab.a, babab.b, babab.c, babab.d, babab.e), (2, 1, 2, 1, 2));

        // Test circular shift patterns
        let bcdea = s5.bcdea();
        assert_eq!((bcdea.a, bcdea.b, bcdea.c, bcdea.d, bcdea.e), (2, 3, 4, 5, 1));

        let cdeab = s5.cdeab();
        assert_eq!((cdeab.a, cdeab.b, cdeab.c, cdeab.d, cdeab.e), (3, 4, 5, 1, 2));

        // Test edge cases
        let eeeee = s5.eeeee();
        assert_eq!((eeeee.a, eeeee.b, eeeee.c, eeeee.d, eeeee.e), (5, 5, 5, 5, 5));

        let ddddd = s5.ddddd();
        assert_eq!((ddddd.a, ddddd.b, ddddd.c, ddddd.d, ddddd.e), (4, 4, 4, 4, 4));
    }

}
