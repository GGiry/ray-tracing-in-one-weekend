use std::fs::File;
use std::io::Write;

use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default)]
struct Vec3 {
    #[derivative(Default(value = "0.0"))]
    x: f64,
    #[derivative(Default(value = "0.0"))]
    y: f64,
    #[derivative(Default(value = "0.0"))]
    z: f64,
}

impl Vec3 {

}

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
            let red = index_width as f64 / ((image_width - 1) as f64);
            let green = index_height as f64 / ((image_height - 1) as f64);
            let blue = 0.25;

            let int_red: i32 = (255.999 * red) as i32;
            let int_green: i32 = (255.999 * green) as i32;
            let int_blue: i32 = (255.999 * blue) as i32;

            file.write_all(format!("{int_red} {int_green} {int_blue}\n").as_bytes())
                .expect("Unable to write data");
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

    #[test]
    fn vec3_default() {
        let vec = Vec3::default();

        assert_eq!(0.0, vec.x);
        assert_eq!(0.0, vec.y);
        assert_eq!(0.0, vec.z);
    }
}