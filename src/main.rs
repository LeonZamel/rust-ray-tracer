use std::fs;

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

#[derive(Copy, Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

fn main() {
    let mut image: Vec<Vec<Vec3>> = vec![
        vec![
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            IMAGE_WIDTH
        ];
        IMAGE_HEIGHT
    ];

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            image[j][i] = Vec3 {
                x: 1.0 * j as f64,
                y: 1.0 * i as f64,
                z: 0.5 * 255.0,
            }
        }
    }

    let mut data = "P3\n".to_string()
        + " "
        + &IMAGE_WIDTH.to_string()
        + " "
        + &IMAGE_HEIGHT.to_string()
        + "\n255\n";

    for row in image {
        for vec in row {
            data += &((vec.x as i32).to_string()
                + " "
                + &(vec.y as i32).to_string()
                + " "
                + &(vec.z as i32).to_string()
                + "\n")
        }
    }

    fs::write("image.ppm", data).expect("ERROR: Couldn't write to file!");
}
