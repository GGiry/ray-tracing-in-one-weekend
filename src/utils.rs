use crate::vec3::Vec3;
use std::fs::File;
use std::io::Write;

use Vec3 as Color;

pub(crate) fn write_color(file: &mut File, pixel: Color) {
    let int_red: i32 = (255.999 * pixel.x()) as i32;
    let int_green: i32 = (255.999 * pixel.y()) as i32;
    let int_blue: i32 = (255.999 * pixel.z()) as i32;

    file.write_all(format!("{int_red} {int_green} {int_blue}\n").as_bytes())
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
        write_color(&mut file, pixel);
        assert_eq!(true, diff("expected_pixel.txt", result_path));
    }
}
