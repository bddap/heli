pub fn meter_coords_to_pixel_coords(xy: (f32, f32), window_size: &LogicalSize) -> (f32, f32) {
    let (x, y) = xy;
    let scale = window_size.height as f32 / 100.0;
    (x * scale, y * scale)
}
