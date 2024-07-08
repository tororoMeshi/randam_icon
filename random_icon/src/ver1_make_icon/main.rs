use image::{Rgb, RgbImage};
use rand::Rng;

const ICON_SIZE: u32 = 500;

fn main() {
    let mut img = RgbImage::new(ICON_SIZE, ICON_SIZE);
    let mut rng = rand::thread_rng();

    let corners = vec![
        (0, 0),
        (ICON_SIZE - 1, 0),
        (0, ICON_SIZE - 1),
        (ICON_SIZE - 1, ICON_SIZE - 1),
    ];
    let center = (ICON_SIZE / 2, ICON_SIZE / 2);

    for &corner in &corners {
        let shape_type = rng.gen_range(0..6);
        let size = rng.gen_range(20..50);
        let color = random_color(&mut rng);

        draw_shape(&mut img, shape_type, corner, size, color);
    }

    let shape_type = rng.gen_range(0..6);
    let size = rng.gen_range(20..50);
    let color = random_color(&mut rng);

    draw_shape(&mut img, shape_type, center, size, color);

    img.save("icon.png").unwrap();
}

fn random_color<R: Rng>(rng: &mut R) -> Rgb<u8> {
    Rgb([rng.gen(), rng.gen(), rng.gen()])
}

fn draw_shape(img: &mut RgbImage, shape_type: u8, position: (u32, u32), size: u32, color: Rgb<u8>) {
    match shape_type {
        0 => draw_circle(img, position, size, color),
        1 => draw_semi_circle(img, position, size, color),
        2 => draw_triangle(img, position, size, color),
        3 => draw_square(img, position, size, color),
        4 => draw_pentagon(img, position, size, color),
        5 => draw_hexagon(img, position, size, color),
        _ => (),
    }
}

fn draw_circle(img: &mut RgbImage, position: (u32, u32), size: u32, color: Rgb<u8>) {
    let (cx, cy) = position;
    for x in cx.saturating_sub(size)..=cx.saturating_add(size) {
        for y in cy.saturating_sub(size)..=cy.saturating_add(size) {
            if (x as i32 - cx as i32).pow(2) + (y as i32 - cy as i32).pow(2) <= (size as i32).pow(2)
                && x < ICON_SIZE
                && y < ICON_SIZE
            {
                img.put_pixel(x, y, color);
            }
        }
    }
}

fn draw_semi_circle(img: &mut RgbImage, position: (u32, u32), size: u32, color: Rgb<u8>) {
    let (cx, cy) = position;
    for x in cx.saturating_sub(size)..=cx.saturating_add(size) {
        for y in cy.saturating_sub(size)..=cy {
            if (x as i32 - cx as i32).pow(2) + (y as i32 - cy as i32).pow(2) <= (size as i32).pow(2)
                && x < ICON_SIZE
                && y < ICON_SIZE
            {
                img.put_pixel(x, y, color);
            }
        }
    }
}

fn draw_triangle(img: &mut RgbImage, position: (u32, u32), size: u32, color: Rgb<u8>) {
    let (cx, cy) = position;
    for x in cx.saturating_sub(size)..=cx.saturating_add(size) {
        for y in cy.saturating_sub(size)..=cy {
            if y < ICON_SIZE && x < ICON_SIZE {
                let dx = x as i32 - cx as i32;
                let dy = cy as i32 - y as i32;
                if dx.abs() <= dy {
                    img.put_pixel(x, y, color);
                }
            }
        }
    }
}

fn draw_square(img: &mut RgbImage, position: (u32, u32), size: u32, color: Rgb<u8>) {
    let (cx, cy) = position;
    for x in cx.saturating_sub(size / 2)..=cx.saturating_add(size / 2) {
        for y in cy.saturating_sub(size / 2)..=cy.saturating_add(size / 2) {
            if x < ICON_SIZE && y < ICON_SIZE {
                img.put_pixel(x, y, color);
            }
        }
    }
}

fn draw_pentagon(img: &mut RgbImage, position: (u32, u32), size: u32, color: Rgb<u8>) {
    let (cx, cy) = position;
    let angle = std::f32::consts::PI * 2.0 / 5.0;
    let points: Vec<(i32, i32)> = (0..5)
        .map(|i| {
            let theta = angle * i as f32;
            let x = cx as i32 + (size as f32 * theta.cos()) as i32;
            let y = cy as i32 - (size as f32 * theta.sin()) as i32;
            (x, y)
        })
        .collect();

    draw_polygon(img, &points, color);
}

fn draw_hexagon(img: &mut RgbImage, position: (u32, u32), size: u32, color: Rgb<u8>) {
    let (cx, cy) = position;
    let angle = std::f32::consts::PI * 2.0 / 6.0;
    let points: Vec<(i32, i32)> = (0..6)
        .map(|i| {
            let theta = angle * i as f32;
            let x = cx as i32 + (size as f32 * theta.cos()) as i32;
            let y = cy as i32 - (size as f32 * theta.sin()) as i32;
            (x, y)
        })
        .collect();

    draw_polygon(img, &points, color);
}

fn draw_polygon(img: &mut RgbImage, points: &[(i32, i32)], color: Rgb<u8>) {
    let len = points.len();
    for i in 0..len {
        let (x0, y0) = points[i];
        let (x1, y1) = points[(i + 1) % len];
        draw_line(img, x0, y0, x1, y1, color);
    }
}

fn draw_line(img: &mut RgbImage, x0: i32, y0: i32, x1: i32, y1: i32, color: Rgb<u8>) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x0;
    let mut y = y0;

    loop {
        if x >= 0 && x < ICON_SIZE as i32 && y >= 0 && y < ICON_SIZE as i32 {
            img.put_pixel(x as u32, y as u32, color);
        }

        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}
