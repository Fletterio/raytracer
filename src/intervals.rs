pub fn div(a: f32, b: f32) -> Option<f32> {
    if 0.0 == b {
        if a > 0.0 {
            Some(f32::INFINITY)
        } else if a < 0.0 {
            Some(f32::NEG_INFINITY)
        } else {
            None
        }
    } else {
        Some(a / b)
    }
}
