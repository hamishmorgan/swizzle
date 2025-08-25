use swizzle::swizzle;

fn main() {
    #[derive(Debug)]
    struct Rgb {
        r: f32,
        g: f32,
        b: f32,
    }

    impl Rgb {
        swizzle!(Rgb { r, g, b });
    }

    let rgb = Rgb {
        r: 0.1,
        g: 0.3,
        b: 0.7,
    };

    print!("rrr: {:?} ", rgb.rrr()); // rrr: Rgb { r: 0.1, g: 0.1, b: 0.1 }
    print!("rrg: {:?} ", rgb.rrg()); // rrg: Rgb { r: 0.1, g: 0.1, b: 0.3 }
    print!("rrb: {:?} ", rgb.rrb()); // rrb: Rgb { r: 0.1, g: 0.1, b: 0.7 }
    println!();
    print!("rgr: {:?} ", rgb.rgr()); // rgr: Rgb { r: 0.1, g: 0.3, b: 0.1 }
    print!("rgg: {:?} ", rgb.rgg()); // rgg: Rgb { r: 0.1, g: 0.3, b: 0.3 }
    print!("rgb: {:?} ", rgb.rgb()); // rgb: Rgb { r: 0.1, g: 0.3, b: 0.7 }
    println!();
    print!("rbr: {:?} ", rgb.rbr()); // rbr: Rgb { r: 0.1, g: 0.7, b: 0.1 }
    print!("rbg: {:?} ", rgb.rbg()); // rbg: Rgb { r: 0.1, g: 0.7, b: 0.3 }
    print!("rbb: {:?} ", rgb.rbb()); // rbb: Rgb { r: 0.1, g: 0.7, b: 0.7 }
    println!();

    print!("grr: {:?} ", rgb.grr()); // grr: Rgb { r: 0.3, g: 0.1, b: 0.1 }
    print!("grg: {:?} ", rgb.grg()); // grg: Rgb { r: 0.3, g: 0.1, b: 0.3 }
    print!("grb: {:?} ", rgb.grb()); // grb: Rgb { r: 0.3, g: 0.1, b: 0.7 }
    println!();
    print!("ggr: {:?} ", rgb.ggr()); // ggr: Rgb { r: 0.3, g: 0.3, b: 0.1 }
    print!("ggg: {:?} ", rgb.ggg()); // ggg: Rgb { r: 0.3, g: 0.3, b: 0.3 }
    print!("ggb: {:?} ", rgb.ggb()); // ggb: Rgb { r: 0.3, g: 0.3, b: 0.7 }
    println!();
    print!("gbr: {:?} ", rgb.gbr()); // gbr: Rgb { r: 0.3, g: 0.7, b: 0.1 }
    print!("gbg: {:?} ", rgb.gbg()); // gbg: Rgb { r: 0.3, g: 0.7, b: 0.3 }
    print!("gbb: {:?} ", rgb.gbb()); // gbb: Rgb { r: 0.3, g: 0.7, b: 0.7 }
    println!();

    print!("brr: {:?} ", rgb.brr()); // brr: Rgb { r: 0.7, g: 0.1, b: 0.1 }
    print!("brg: {:?} ", rgb.brg()); // brg: Rgb { r: 0.7, g: 0.1, b: 0.3 }
    print!("brb: {:?} ", rgb.brb()); // brb: Rgb { r: 0.7, g: 0.1, b: 0.7 }
    println!();
    print!("bgr: {:?} ", rgb.bgr()); // bgr: Rgb { r: 0.7, g: 0.3, b: 0.1 }
    print!("bgg: {:?} ", rgb.bgg()); // bgg: Rgb { r: 0.7, g: 0.3, b: 0.3 }
    print!("bgb: {:?} ", rgb.bgb()); // bgb: Rgb { r: 0.7, g: 0.3, b: 0.7 }
    println!();
    print!("bbr: {:?} ", rgb.bbr()); // bbr: Rgb { r: 0.7, g: 0.7, b: 0.1 }
    print!("bbg: {:?} ", rgb.bbg()); // bbg: Rgb { r: 0.7, g: 0.7, b: 0.3 }
    print!("bbb: {:?} ", rgb.bbb()); // bbb: Rgb { r: 0.7, g: 0.7, b: 0.7 }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
