use firefly_rust::{Color, RGB, set_color};

pub fn set_colors() {
    set_color(Color::from(1), RGB::new(0xff, 0xff, 0xff));
    set_color(Color::from(2), RGB::new(0x0c, 0xe6, 0xf2));
    set_color(Color::from(3), RGB::new(0x00, 0x98, 0xdb));
    set_color(Color::from(4), RGB::new(0x1e, 0x57, 0x9c));
    set_color(Color::from(5), RGB::new(0x20, 0x35, 0x62));
    set_color(Color::from(6), RGB::new(0x25, 0x24, 0x46));
    set_color(Color::from(7), RGB::new(0x20, 0x15, 0x33));
    set_color(Color::from(8), RGB::new(0x96, 0x67, 0x67));
    set_color(Color::from(9), RGB::new(0x47, 0x17, 0x17));
    set_color(Color::from(10), RGB::new(0x0c, 0x2e, 0x24));
    set_color(Color::from(11), RGB::new(0x44, 0x75, 0x62));
    set_color(Color::from(12), RGB::new(0x3b, 0x42, 0x4b));
    set_color(Color::from(13), RGB::new(0x20, 0x15, 0x33));
    set_color(Color::from(14), RGB::new(0x2f, 0x25, 0x22));
    set_color(Color::from(15), RGB::new(0x00, 0x00, 0x00));
    set_color(Color::from(16), RGB::new(0x00, 0x00, 0x00));
}
