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
        swizzle!(Scalar { x });
        swizzle!(Vec2 { x: (x), y: (x) });
        swizzle!(Vec3 {
            x: (x),
            y: (x),
            z: (x)
        });
    }

    impl Vec2 {
        swizzle!(Scalar { x: (x, y) });
        swizzle!(Vec2 { x, y });
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

    println!("v3: {:?}", v3);

    println!("v3.xxx: {:?}", v3.xxx());
    println!("v3.xxy: {:?}", v3.xxy());
    println!("v3.xxz: {:?}", v3.xxz());
    println!("v3.xyx: {:?}", v3.xyx());
    println!("v3.xyy: {:?}", v3.xyy());
    println!("v3.xyz: {:?}", v3.xyz());
    println!("v3.xzx: {:?}", v3.xzx());
    println!("v3.xzy: {:?}", v3.xzy());
    println!("v3.yxx: {:?}", v3.yxx());
    println!("v3.yxy: {:?}", v3.yxy());
    println!("v3.yxz: {:?}", v3.yxz());
    println!("v3.yyx: {:?}", v3.yyx());
    println!("v3.yyy: {:?}", v3.yyy());
    println!("v3.yyz: {:?}", v3.yyz());
    println!("v3.yzx: {:?}", v3.yzx());
    println!("v3.yzy: {:?}", v3.yzy());
    println!("v3.zxx: {:?}", v3.zxx());
    println!("v3.zxy: {:?}", v3.zxy());
    println!("v3.zxz: {:?}", v3.zxz());
    println!("v3.zyx: {:?}", v3.zyx());
    println!("v3.zyy: {:?}", v3.zyy());
    println!("v3.zyz: {:?}", v3.zyz());
    println!("v3.zzx: {:?}", v3.zzx());
    println!("v3.zzy: {:?}", v3.zzy());
    println!("v3.zzz: {:?}", v3.zzz());

    println!("v3.xx: {:?}", v3.xx());
    println!("v3.xy: {:?}", v3.xy());
    println!("v3.xz: {:?}", v3.xz());
    println!("v3.yx: {:?}", v3.yx());
    println!("v3.yy: {:?}", v3.yy());
    println!("v3.yz: {:?}", v3.yz());
    println!("v3.zx: {:?}", v3.zx());
    println!("v3.zy: {:?}", v3.zy());
    println!("v3.zz: {:?}", v3.zz());

    println!("v3.x: {:?}", v3.x());
    println!("v3.y: {:?}", v3.y());
    println!("v3.z: {:?}", v3.z());

    let v2 = v3.xy();
    println!("v2: {:?}", v2);

    println!("v2.xxx: {:?}", v2.xxx());
    println!("v2.xxy: {:?}", v2.xxy());
    println!("v2.xyx: {:?}", v2.xyx());
    println!("v2.xyy: {:?}", v2.xyy());
    println!("v2.yxx: {:?}", v2.yxx());
    println!("v2.yxy: {:?}", v2.yxy());
    println!("v2.yyx: {:?}", v2.yyx());
    println!("v2.yyy: {:?}", v2.yyy());

    println!("v2.xx: {:?}", v2.xx());
    println!("v2.xy: {:?}", v2.xy());
    println!("v2.yx: {:?}", v2.yx());
    println!("v2.yy: {:?}", v2.yy());

    println!("v2.x: {:?}", v2.x());
    println!("v2.y: {:?}", v2.y());

    let s = v2.x();
    println!("s: {:?}", s.x);

    println!("s.x: {:?}", s.x);

    println!("s.xx: {:?}", s.xx());

    println!("s.xxx: {:?}", s.xxx());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
