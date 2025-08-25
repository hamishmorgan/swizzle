# Swizzle

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://github.com/hamishmorgan/swizzle/actions/workflows/rust.yml/badge.svg)](https://github.com/hamishmorgan/swizzle/actions/workflows/rust.yml)

Rust macro for generating swizzle functions on structs. The `swizzle!` macro automatically generates functions that allow you to create new instances of a struct with field values rearranged according to any combination of the original fields.

## What is Swizzling?

Swizzling is a technique commonly used in graphics programming where you can access and reorder vector components in any combination. For example, with a 3D vector `(x, y, z)`, you might want to create a new vector with values `(y, x, z)` or `(x, x, x)`.

## Quick Start

```rust
use swizzle::swizzle;

struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    swizzle!(Vec2 { x, y });
}

let v = Vec2 { x: 1.0, y: 2.0 };
let v_swapped = v.yx();  // Vec2 { x: 2.0, y: 1.0 }
let v_repeated = v.xx(); // Vec2 { x: 1.0, y: 1.0 }
```

## Examples

### 2D Vector
```rust
use swizzle::swizzle;

struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    swizzle!(Vec2 { x, y });
}

let v = Vec2 { x: 1.0, y: 2.0 };
assert_eq!(v.xx().x, 1.0);
assert_eq!(v.xx().y, 1.0);
assert_eq!(v.yy().x, 2.0);
assert_eq!(v.yy().y, 2.0);
assert_eq!(v.xy().x, 1.0);
assert_eq!(v.xy().y, 2.0);
assert_eq!(v.yx().x, 2.0);
assert_eq!(v.yx().y, 1.0);
```

### 3D Vector
```rust
use swizzle::swizzle;

struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    swizzle!(Vec3 { x, y, z });
}

let v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
let v_xy = v.xyy();      // Vec3 { x: 1.0, y: 2.0, z: 2.0 }
let v_reverse = v.zyx(); // Vec3 { x: 3.0, y: 2.0, z: 1.0 }
```

### Color with Alpha
```rust
use swizzle::swizzle;

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    swizzle!(Color { r, g, b, a });
}

let c = Color { r: 255, g: 128, b: 64, a: 255 };
let c_bgr = c.bgrb();    // Color { r: 64, g: 128, b: 255, a: 64 }
let c_grayscale = c.rrrr(); // Color { r: 255, g: 255, b: 255, a: 255 }
```

### Cross-Type Swizzling
```rust
use swizzle::swizzle;

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
    // Create Vec3 from Vec2 by repeating components
    swizzle!(Vec3 { x: (x, y), y: (x, y), z: (x, y) });
}

impl Vec3 {
    // Create Vec2 from Vec3 by selecting components
    swizzle!(Vec2 { x: (x, y, z), y: (x, y, z) });
}

let v2 = Vec2 { x: 1.0, y: 2.0 };
let v3 = v2.xyx(); // Vec3 { x: 1.0, y: 2.0, z: 1.0 }

let v3_orig = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
let v2_proj = v3_orig.xy(); // Vec2 { x: 1.0, y: 2.0 }
```

## How It Works

The macro generates all possible combinations of field values. For a struct with `n` fields, it creates `n^n` different swizzle functions. Each function returns a new instance of the struct with the field values arranged according to the function name.

### Generated Functions

For each possible combination of field names, the macro generates a function named after that combination:

- `aaa()` → returns struct with all fields set to the value of `a`
- `abc()` → returns struct with fields set to `a`, `b`, `c` respectively
- `cba()` → returns struct with fields set to `c`, `b`, `a` respectively

### Advanced Usage

The macro supports cross-type swizzling where you can create instances of different structs:

```rust
swizzle!(TargetStruct { field1: (src1, src2), field2: (src1, src2) });
```

This creates functions that return `TargetStruct` with values from the source struct's fields.

## Features

- **Automatic Generation**: No need to write repetitive swizzle functions manually
- **Type Safety**: All generated functions maintain Rust's type safety
- **Performance**: Functions are marked as `#[inline]` and `#[must_use]`
- **Const Functions**: All generated functions are `const fn` for use in const contexts
- **Cross-Type Support**: Create instances of different structs from source structs
- **Comprehensive**: Generates all possible combinations automatically

## Use Cases

- **Graphics Programming**: Swizzling vector components (xy, yx, xyz, etc.)
- **Data Manipulation**: Reordering struct fields for different data views
- **Mathematical Operations**: Creating variations of mathematical objects
- **API Design**: Providing convenient access patterns for struct data
- **Type Conversion**: Converting between different struct types with compatible fields

## Performance Considerations

- All generated functions are `#[inline]` for optimal performance
- Functions are marked as `#[must_use]` to prevent accidental discarding of results
- All generated functions are `const fn` for use in const contexts.
- The macro generates `n^m` functions, where `n` is the number of the destination attributes and `m` is the number of source attributes. 
- For large numbers of fields, consider the compilation time impact. On modern hardware 5 fields in manageable, but is _extremely_ slow!

## Limitations

- Field names must be valid Rust identifiers.
- All fields must be of a type that can be copied.
- The macro generates a _lot_ of functions for structs with many fields. This can take a long time.
- Cross-type swizzling requires compatible field types that are either the same or that can be converted implicitly. 

## Dependencies

This crate depends on the `paste` crate for hygienic macro expansion.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Limitations / Todo

These features are not supported and maybe I'll add them one day:

 - Migrate to a stable rust edition/version
 - Make less useful swizzles a feature that can disabled; e.g `rgb.rgb()` 
 - Structs with field that implement `Clone` but not `Copy`. 
 - Publish a crate and add corresponding installation instructions.