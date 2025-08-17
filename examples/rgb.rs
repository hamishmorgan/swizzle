use swizzle::swizzle;

fn main() {
    #[derive(Debug)]
    struct Rgb {
        r: u8,
        g: u8,
        b: u8,
    }

    swizzle!(Rgb, r, g, b);

    let rgb = Rgb { r: 3, g: 5, b: 7 };

    print!("rrr: {:?} ", rgb.rrr()); // rrr: Rgb { r: 3, g: 3, b: 3 }
    print!("rrg: {:?} ", rgb.rrg()); // rrg: Rgb { r: 3, g: 3, b: 5 }
    print!("rrb: {:?} ", rgb.rrb()); // rrb: Rgb { r: 3, g: 3, b: 7 }
    println!();
    print!("rgr: {:?} ", rgb.rgr()); // rgr: Rgb { r: 3, g: 5, b: 3 }
    print!("rgg: {:?} ", rgb.rgg()); // rgg: Rgb { r: 3, g: 5, b: 5 }
    print!("rgb: {:?} ", rgb.rgb()); // rgb: Rgb { r: 3, g: 5, b: 7 }
    println!();
    print!("rbr: {:?} ", rgb.rbr()); // rbr: Rgb { r: 3, g: 7, b: 3 }
    print!("rbg: {:?} ", rgb.rbg()); // rbg: Rgb { r: 3, g: 7, b: 5 }
    print!("rbb: {:?} ", rgb.rbb()); // rbb: Rgb { r: 3, g: 7, b: 7 }
    println!();

    print!("grr: {:?} ", rgb.grr()); // grr: Rgb { r: 5, g: 3, b: 3 }
    print!("grg: {:?} ", rgb.grg()); // grg: Rgb { r: 5, g: 3, b: 3 }
    print!("grb: {:?} ", rgb.grb()); // grb: Rgb { r: 5, g: 3, b: 5 }
    println!();
    print!("ggr: {:?} ", rgb.ggr()); // ggr: Rgb { r: 5, g: 5, b: 3 }
    print!("ggg: {:?} ", rgb.ggg()); // ggg: Rgb { r: 5, g: 5, b: 5 }
    print!("ggb: {:?} ", rgb.ggb()); // ggb: Rgb { r: 5, g: 5, b: 7 }
    println!();
    print!("gbr: {:?} ", rgb.gbr()); // gbr: Rgb { r: 5, g: 7, b: 3 }
    print!("gbg: {:?} ", rgb.gbg()); // gbg: Rgb { r: 5, g: 7, b: 5 }
    print!("gbb: {:?} ", rgb.gbb()); // gbb: Rgb { r: 5, g: 7, b: 7 }
    println!();

    print!("brr: {:?} ", rgb.brr()); // brr: Rgb { r: 7, g: 3, b: 3 }
    print!("brg: {:?} ", rgb.brg()); // brg: Rgb { r: 7, g: 3, b: 5 }
    print!("brb: {:?} ", rgb.brb()); // brb: Rgb { r: 7, g: 3, b: 7 }
    println!();
    print!("bgr: {:?} ", rgb.bgr()); // bgr: Rgb { r: 7, g: 5, b: 3 }
    print!("bgg: {:?} ", rgb.bgg()); // bgg: Rgb { r: 7, g: 5, b: 5 }
    print!("bgb: {:?} ", rgb.bgb()); // bgb: Rgb { r: 7, g: 5, b: 7 }
    println!();
    print!("bbr: {:?} ", rgb.bbr()); // bbr: Rgb { r: 7, g: 7, b: 3 }
    print!("bbg: {:?} ", rgb.bbg()); // bbg: Rgb { r: 7, g: 7, b: 5 }
    print!("bbb: {:?} ", rgb.bbb()); // bbb: Rgb { r: 7, g: 7, b: 7 }
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
