use std::fs::File;
use std::io::Write;

mod ray;
mod utils;
mod vec3;

use crate::utils::write_color;
use crate::vec3::Vec3;

use Vec3 as Color;

fn create_gradient(path: &str) {
    let mut file = File::create(path).unwrap();

    let image_width = 256;
    let image_height = 256;

    file.write_all(b"P3\n").expect("Unable to write data");
    file.write_all(format!("{image_width} {image_height}\n").as_bytes())
        .expect("Unable to write data");
    file.write_all(b"255\n").expect("Unable to write data");

    for index_height in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {index_height}");
        for index_width in 0..image_width {
            let pixel = Color::new(
                index_width as f64 / ((image_width - 1) as f64),
                index_height as f64 / ((image_height - 1) as f64),
                0.25,
            );

            write_color(&mut file, pixel);
        }
    }
    eprintln!("Done.");
}

fn main() {
    create_gradient("result.ppm");
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_diff::diff;

    #[test]
    fn gradient_ppm() {
        let result_path = "result_gradient.ppm";
        create_gradient(result_path);
        assert_eq!(true, diff("expected_gradient.ppm", result_path));
    }
}
