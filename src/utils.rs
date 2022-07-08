use std::fs::File;
use std::io::Write;
use crate::vec3::Vec3;

use Vec3 as Color;

fn write_color(mut file: File, pixel: Color) {
    let int_red: i32 = (255.999 * pixel.x()) as i32;
    let int_green: i32 = (255.999 * pixel.y()) as i32;
    let int_blue: i32 = (255.999 * pixel.z()) as i32;

    file.write_all(format!("{int_red} {int_green} {int_blue}\n").as_bytes())
                .expect("Unable to write data");
}

#[cfg(test)]
mod tests {
    use file_diff::diff;
    use super::*;

    #[test]
    fn write_pixel() {
        let result_path = "expected_pixel.txt";
        let file = File::create(result_path).unwrap();
        let pixel = Color::new(0.0, 0.5, 0.9);
        write_color(file, pixel );
        assert_eq!(true, diff("expected_pixel.txt", result_path));
    }
}