use crate::vec3::Vec3;
use std::fs::File;
use std::io::Write;

use Vec3 as Color;

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }

    if x > max {
        return max;
    }

    return x;
}

pub fn write_color(file: &mut File, pixel: Color, samples_per_pixel: u32) {
    let scale = 1.0 / (samples_per_pixel as f64);

    let int_red = (256.0 * clamp(pixel.x() * scale, 0.0, 0.999)) as u32;
    let int_green = (256.0 * clamp(pixel.y() * scale, 0.0, 0.999)) as u32;
    let int_blue = (256.0 * clamp(pixel.z() * scale, 0.0, 0.999)) as u32;

    file.write_all(format!("{} {} {}\n", int_red, int_green, int_blue).as_bytes())
        .expect("Unable to write data");
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_diff::diff;

    #[test]
    fn write_pixel() {
        let result_path = "expected_pixel.txt";
        let mut file = File::create(result_path).unwrap();
        let pixel = Color::new(0.0, 0.5, 0.9);
        write_color(&mut file, pixel, 1);
        assert_eq!(true, diff("expected_pixel.txt", result_path));
    }

    #[test]
    fn clamp_test() {
        assert_eq!(0.5, clamp(0.5, 0.0, 1.0));
        assert_eq!(0.0, clamp(-0.5, 0.0, 1.0));
        assert_eq!(1.0, clamp(1.5, 0.0, 1.0));
    }
}
