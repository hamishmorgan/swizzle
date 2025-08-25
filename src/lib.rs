#![no_std]

//! # Swizzle
//!
//! Macro for generating swizzle functions on structs.
//!
//! The `swizzle!` macro automatically generates functions that allow you to create new instances
//! of a struct with field values rearranged according to any combination of the original fields.
//! This is particularly useful for mathematical operations, graphics programming, and data manipulation.
//!
//! ## What is Swizzling?
//!
//! [Swizzling](https://en.wikipedia.org/wiki/Swizzling_(computer_graphics)) is a technique commonly used in graphics programming where you can access and
//! reorder vector components in any combination. For example, with a 3D vector `(x, y, z)`,
//! you might want to create a new vector with values `(y, x, z)` or `(x, x, x)`, or maybe you want to convert to a 2D vector `(z,x)`.
//!
//! ## Quick Start
//!
//! ```rust
//! use swizzle::swizzle;
//!
//! struct Vec2 {
//!     x: f32,
//!     y: f32,
//! }
//!
//! impl Vec2 {
//!     swizzle!(Vec2 { x, y });
//! }
//!
//! let v = Vec2 { x: 1.0, y: 2.0 };
//! let v_swapped = v.yx();  // Vec2 { x: 2.0, y: 1.0 }
//! let v_repeated = v.xx(); // Vec2 { x: 1.0, y: 1.0 }
//! ```
//!
//! ## How it works
//!
//! The `swizzle!` macro generates functions that rearranged the fields in any combination. Swizzles
//! can also be used to convert between different types, with different numbers/names of attributes.
//! This is particularly useful for mathematical operations, graphics programming, and data manipulation.
//!
//! The macro generates all possible combinations of field values. For self-swizzle of a struct with `n` fields,
//! it creates `n^n` different swizzle functions. When converting between a source struct with `m` fields
//! and destination struct with `n` fields, it creates `n^m` swizzle functions.
//!
//! ## Usage
//!
//! The macro recognizes several different forms of input pattern.
//!
//! ### Single Method Creation
//!
//! The macro can be used to create each swizzle method individually
//!
//! ```rust
//! use swizzle::swizzle;
//!
//! struct Point2 { x: i32, y: i32,  }
//!
//! impl Point2 {
//!     swizzle!(Point2 { x: y, y: x });
//!
//!     // Generates:
//!     //
//!     // pub const fn yx(&self) -> Point2 {
//!     //     Point2 { x: self.y, y: self.x }
//!     // }
//! }
//!
//! let p = Point2 { x: 1, y: 2 };
//! let p_swapped = p.yx();  // Point2 { x: 2, y: 1 }
//! ```
//!
//! Single method creation can also be use to convert between different types with different number of fields.
//!
//! ```rust
//! use swizzle::swizzle;
//!
//! struct Rgb { r: u8, g: u8, b: u8, }
//! struct Rgba { r: u8, g: u8, b: u8, a: u8, }
//!
//! impl Rgba {
//!     swizzle!(Rgb { r: r, g: g, b: b });
//! }
//!
//! let rgba = Rgba { r: 255, g: 255, b: 255, a: 255 };
//! let rgb = rgba.rgb();  // Rgb { r: 255, g: 255, b: 255 }
//! ```
//!
//! It can also be used where the field names are different.
//!
//! ```rust
//! use swizzle::swizzle;
//!
//! struct Foo { x: u8, y: u8, }
//!
//! struct Bar { a: u8, b: u8, c: u8, }
//!
//! impl Foo {
//!     swizzle!(Bar { a: y, b: x, c: y });
//! }
//!
//! let foo = Foo { x: 1, y: 2 };
//! let _ = foo.yxy();  // Bar { a: 2, b: 1, c: 2 }
//! ```
//!
//! ### Multiple Method Creation
//!
//! More usefully than the single method macro, you can create all possible combinations of the given destination fields and source values.
//!
//! ```rust
//! use swizzle::swizzle;
//!
//! struct Point2 { x: i32, y: i32, }
//!
//! impl Point2 {
//!     swizzle!(Point2 { x: (x, y), y: (x, y) });
//!
//!     // Generates:
//!     // swizzle!(Point2 { x: x, y: x });
//!     // swizzle!(Point2 { x: x, y: y });
//!     // swizzle!(Point2 { x: y, y: x });
//!     // swizzle!(Point2 { x: y, y: y });
//! }
//!
//! let p = Point2 { x: 1, y: 2 };
//! let p_swapped = p.yx();  // Point2 { x: 2, y: 1 }
//! ```
//!
//! The source and destination types don't need to be the same, and they can have a different number of fields and different field names.
//!
//! ```rust
//! use swizzle::swizzle;
//!
//! struct Rgb { r: u8, g: u8, b: u8, }
//! struct Rgba { r: u8, g: u8, b: u8, a: u8, }
//!
//! impl Rgba {
//!     swizzle!(Rgb { r: (r,g,b,a), g: (r,g,b,a), b: (r,g,b,a) });
//!
//!     // Generates:
//!     // swizzle!(Rgb { r: r, g: r, b: r });
//!     // swizzle!(Rgb { r: r, g: r, b: g });
//!     // swizzle!(Rgb { r: r, g: r, b: b });
//!     // swizzle!(Rgb { r: r, g: r, b: a });
//!     // ...
//! }
//!
//! let rgba = Rgba { r: 255, g: 255, b: 255, a: 255 };
//! let rgb = rgba.rgb();  // Rgb { r: 255, g: 255, b: 255 }
//! ```
//!
//! ### Self-Swizzle Shorthand
//!
//! ```rust
//! use swizzle::swizzle;
//!
//! struct Vec4 { x: f32, y: f32, z: f32, w: f32, }
//!
//! impl Vec4 {
//!     swizzle!(Vec4 { x, y, z, w });
//!     // Which is equivalent to
//!     // swizzle!(Vec4 { x: (x,y,z,w), y: (x,y,z,w), z: (x,y,z,w), w: (x,y,z,w) });
//! }
//! ```
//!
//! ### Complete Vector Conversions
//!
//! ```rust
//! use swizzle::swizzle;
//!
//! struct Scalar { x: f32 }
//! struct Vec2 { x: f32, y: f32, }
//! struct Vec3 { x: f32, y: f32, z: f32 }
//! struct Vec4 { x: f32, y: f32, z: f32, w: f32 }
//!
//! impl Scalar {
//!     swizzle!(Scalar { x });
//!     swizzle!(Vec2 { x: (x), y: (x) });
//!     swizzle!(Vec3 { x: (x), y: (x), z: (x) });
//!     swizzle!(Vec4 { x: (x), y: (x), z: (x), w: (x) });
//! }
//!
//! impl Vec2 {
//!     swizzle!(Scalar { x: (x,y) });
//!     swizzle!(Vec2 { x, y });
//!     swizzle!(Vec3 { x: (x,y), y: (x,y), z: (x,y) });
//!     swizzle!(Vec4 { x: (x,y), y: (x,y), z: (x,y), w: (x,y) });
//! }
//!
//! impl Vec3 {
//!     swizzle!(Scalar { x: (x,y,z) });
//!     swizzle!(Vec2 { x: (x,y,z), y: (x,y,z) });
//!     swizzle!(Vec3 { x, y, z });
//!     swizzle!(Vec4 { x: (x,y,z), y: (x,y,z), z: (x,y,z), w: (x,y,z) });
//! }
//!
//! impl Vec4 {
//!     swizzle!(Scalar { x: (x,y,z,w) });
//!     swizzle!(Vec2 { x: (x,y,z,w), y: (x,y,z,w) });
//!     swizzle!(Vec3 { x: (x,y,z,w), y: (x,y,z,w), z: (x,y,z,w) });
//!     swizzle!(Vec4 { x, y, z, w });
//! }
//!
//! let v = Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
//! let _ = v.wzyx().xyz().yx().yy().x().xxxx();
//! ```
//!
//! ## Performance Considerations
//!
//! The macro generates `n^m` functions, where `n` is the number of the destination attributes and `m` is the number of source attributes. For a struct with 5 fields that's 3125 distinct functions!
//!
//! For large numbers of fields, consider the compilation time impact. Without optimization the binary sizes may also be greatly increased.
//!
//! On modern hardware 5 fields is manageable, but is _extremely_ slow!
//!
//! - All generated functions are `#[inline]` for optimal performance
//! - Functions are marked as `#[must_use]` to prevent accidental discarding of results
//! - All generated functions are `const fn` for use in const contexts
//!
//! ## Limitations
//!
//! - Field names must be valid Rust identifiers
//! - All fields must be of types that can be copied
//! - The macro generates a lot of functions for structs with many fields
//! - Field order in the struct definition matters for the generated function names
//! - Cross-type swizzling requires compatible field types that are either the same or that can be converted implicitly
//!
//! ## Dependencies
//!
//! This crate depends on the [`pastey`](https://crates.io/crates/pastey) crate for hygienic macro expansion.
//!
//! ## License
//!
//! This project is licensed under the MIT License

mod swizzle;
