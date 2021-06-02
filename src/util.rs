use macroquad::color::Color;

pub fn color(r: u8, g: u8, b: u8) -> Color {
    Color {
        r: r as f32 / 255.,
        g: g as f32 / 255.,
        b: b as f32 / 255.,
        a: 255.0,
    }
}
