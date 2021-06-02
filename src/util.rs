use macroquad::color::Color;
use parry2d::math::Point;
use parry2d::shape::Polyline;

pub fn color(r: u8, g: u8, b: u8) -> Color {
    Color {
        r: r as f32 / 255.,
        g: g as f32 / 255.,
        b: b as f32 / 255.,
        a: 255.0,
    }
}

pub fn wireframe_to_polyline(wf: &[(f32, f32)]) -> Polyline {
    Polyline::new(
        wf.iter().cloned().map(|(x, y)| Point::new(x, y)).collect(),
        None,
    )
}
