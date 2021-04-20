#[allow(dead_code)]

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    assert!(min <= max);
    let mut x = value;
    if x < min {
        x = min;
    }
    if x > max {
        x = max;
    }
    x
}
