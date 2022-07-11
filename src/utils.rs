use std::fs::File;
use std::io::Write;

use rand::Rng;

use Vec3 as Color;

use crate::vec3::Vec3;

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

pub fn random_f64() -> f64 {
    return rand::thread_rng().gen();
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    return rand::thread_rng().gen_range(min..max);
}

#[cfg(test)]
mod tests {
    use file_diff::diff;

    use super::*;

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

    #[test]
    fn random_test() {
        for _ in 0..100 {
            let random = random_f64();
            assert!(0.0 <= random);
            assert!(1.0 >= random);
        }
    }

    #[test]
    fn random_range_test() {
        for _ in 0..100 {
            let random = random_f64_range(-1.0, 10.0);
            assert!(-1.0 <= random);
            assert!(10.0 >= random);
        }
    }
}
