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

pub fn channel_to_rgb(pixel: f64, scale: f64) -> u8 {
    // Gamma correction is color ^ 1 / gamma value. Here we gamma correct of 2.
    let gamma_corrected = (pixel * scale).sqrt();
    return (256.0 * clamp(gamma_corrected, 0.0, 0.999)) as u8;
}

pub fn color_to_rbg(pixel: Color, samples_per_pixel: u32) -> Vec<u8> {
    let scale = 1.0 / (samples_per_pixel as f64);
    let mut result = Vec::new();

    result.push(channel_to_rgb(pixel.x(), scale));
    result.push(channel_to_rgb(pixel.y(), scale));
    result.push(channel_to_rgb(pixel.z(), scale));

    return result;
}

pub fn random_f64() -> f64 {
    return rand::thread_rng().gen();
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    return rand::thread_rng().gen_range(min..max);
}

#[cfg(test)]
mod tests {
    use super::*;

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
