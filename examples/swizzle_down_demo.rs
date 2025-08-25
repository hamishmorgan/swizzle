use swizzle::swizzle;

#[allow(dead_code)]
fn main() {
    #[derive(Debug)]
    struct Scalar {
        x: f32,
    }

    #[derive(Debug)]
    struct Vec2 {
        x: f32,
        y: f32,
    }

    #[derive(Debug)]
    struct Vec3 {
        x: f32,
        y: f32,
        z: f32,
    }

    impl Scalar {
        swizzle!(Vec2 { x: (x), y: (x) });
        swizzle!(Vec3 {
            x: (x),
            y: (x),
            z: (x)
        });
    }

    impl Vec2 {
        swizzle!(Scalar { x: (x, y) });
        swizzle!(Vec3 {
            x: (x, y),
            y: (x, y),
            z: (x, y)
        });
    }

    impl Vec3 {
        swizzle!(Scalar { x: (x, y, z) });
        swizzle!(Vec2 {
            x: (x, y, z),
            y: (x, y, z)
        });
        swizzle!(Vec3 { x, y, z });
    }

    let v3 = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let v2 = v3.xz();
    println!("v2: {:?}", v2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
