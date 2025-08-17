# Swizzle

[![Crates.io](https://img.shields.io/crates/v/swizzle)](https://crates.io/crates/swizzle)
[![Documentation](https://docs.rs/swizzle/badge.svg)](https://docs.rs/swizzle)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Rust macro for generating swizzle functions on structs. The `swizzle!` macro automatically generates functions that allow you to create new instances of a struct with field values rearranged according to any combination of the original fields.

## What is Swizzling?

Swizzling is a technique commonly used in graphics programming where you can access and reorder vector components in any combination. For example, with a 3D vector `(x, y, z)`, you might want to create a new vector with values `(y, x, z)` or `(x, x, x)`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
swizzle = "0.1.0"
```

## Quick Start

```rust
use swizzle::swizzle;

struct Vec2 {
    x: f32,
    y: f32,
}

swizzle!(Vec2, x, y);

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

swizzle!(Vec2, x, y);

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

swizzle!(Vec3, x, y, z);

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

swizzle!(Color, r, g, b, a);

let c = Color { r: 255, g: 128, b: 64, a: 255 };
let c_bgr = c.bgrb();    // Color { r: 64, g: 128, b: 255, a: 64 }
let c_grayscale = c.rrrr(); // Color { r: 255, g: 255, b: 255, a: 255 }
```

## How It Works

The macro generates all possible combinations of field values. For a struct with `n` fields, it creates `n^n` different swizzle functions. Each function returns a new instance of the struct with the field values arranged according to the function name.

### Generated Functions

For each possible combination of field names, the macro generates a function named after that combination:

- `aaa()` → returns struct with all fields set to the value of `a`
- `abc()` → returns struct with fields set to `a`, `b`, `c` respectively
- `cba()` → returns struct with fields set to `c`, `b`, `a` respectively

## Features

- **Automatic Generation**: No need to write repetitive swizzle functions manually
- **Type Safety**: All generated functions maintain Rust's type safety
- **Performance**: Functions are marked as `#[inline]` and `#[must_use]`
- **Const Functions**: Can be used in const contexts
- **Comprehensive**: Generates all possible combinations automatically

## Use Cases

- **Graphics Programming**: Swizzling vector components (xy, yx, xyz, etc.)
- **Data Manipulation**: Reordering struct fields for different data views
- **Mathematical Operations**: Creating variations of mathematical objects
- **API Design**: Providing convenient access patterns for struct data

## Performance Considerations

- All generated functions are `#[inline]` for optimal performance
- Functions are marked as `#[must_use]` to prevent accidental discarding of results
- The macro generates `n^n` functions for a struct with `n` fields
- For large numbers of fields, consider the compilation time impact

## Limitations

- Field names must be valid Rust identifiers
- All fields must be of types with the `Copy` trait
- The macro generates a lot of functions for structs with many fields
- Field order in the struct definition matters for the generated function names

## Dependencies

This crate depends on the `paste` crate for hygienic macro expansion.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Changelog

### 0.1.0
- Initial release
- Basic swizzle macro functionality
- Support for arbitrary number of fields
- Comprehensive test coverage

## Roadmap

- [ ] Support for generic types
- [ ] Custom field type mapping
- [ ] Performance optimizations
- [ ] Additional utility functions
