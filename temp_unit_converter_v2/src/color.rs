#![allow(dead_code)]

#[derive(Debug)]
enum Color {
    RGB(u8, u8, u8),
    CMYK(f32, f32, f32, f32),
    Hex(String),
}

impl Color {
    fn convert_to_cmyk(&self) {
        match *self {
            Self::RGB(r, g, b) => {
                let r1 = r as f32 / 255.0;
                let g1 = g as f32 / 255.0;
                let b1 = b as f32 / 255.0;

                let k = 1.0 - (r1.max(g1)).max(b1);
                let c = (1.0 - r1 - k) / (1.0 - k);
                let m = (1.0 - g1 - k) / (1.0 - k);
                let y = (1.0 - b1 - k) / (1.0 - k);

                println!("{:.2}, {:.2}, {:.2}, {:.2}", c, m, y, k);
            }
            _ => (),
        }
    }
    fn convert_to_rgb(&self) {
        match *self {
            Self::CMYK(c, m, y, k) => {
                let r = (255.0 * (1.0 - c) * (1.0 - k)) as u8;
                let g = (255.0 * (1.0 - m) * (1.0 - k)) as u8;
                let b = (255.0 * (1.0 - y) * (1.0 - k)) as u8;

                println!("{}, {}, {}", r, g, b);
            }
            _ => (),
        }
    }
}

fn main() {
    let my_rgb = Color::RGB(240, 236, 18);
    let my_cmyk = Color::CMYK(0.0, 0.02, 0.93, 0.06);

    my_rgb.convert_to_cmyk();
    my_cmyk.convert_to_rgb();
}
