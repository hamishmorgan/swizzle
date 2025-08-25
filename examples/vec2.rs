use swizzle::swizzle;

fn main() {
    #[derive(Debug)]
    struct Vec2 {
        x: f32,
        y: f32,
    }

    impl Vec2 {
        swizzle!(Vec2 { x, y });
    }

    let vec = Vec2 { x: 1.5, y: 2.5 };

    println!("Useful patterns:");
    println!("Original: {:?}", vec);
    println!("Swizzled xx: {:?} (repeating x component)", vec.xx());
    println!("Swizzled yy: {:?} (repeating y component)", vec.yy());
    println!("Swizzled yx: {:?} (swapped components)", vec.yx());
    println!("Swizzled xy: {:?} (original order)", vec.xy());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
