use image::{Rgb, RgbImage};
use rand::Rng;

// 定数パラメータの定義
const ICON_SIZE: u32 = 500;
const MIN_SHAPE_SIZE: u32 = 150;
const MAX_SHAPE_SIZE: u32 = 250;
const SHAPE_TYPES: u8 = 5;
const COLORS: [&str; 31] = [
    "#f19072", //曙色
    "#e4dc8a", //枯れ草色
    "#f8f4e6", //象牙色
    "#b7282e", //茜色
    "#f09199", //桃色
    "#fef4f4", //桜色
    "#c39143", //黄土色
    "#8a3b00", //褐色
    "#f08300", //蜜柑色
    "#ed6d3d", //柿色
    "#ee7800", //橙色
    "#eb6101", //朱色
    "#ffd900", //蒲公英色
    "#ffec47", //菜の花色
    "#f8b500", //山吹色
    "#e6b422", //金色
    "#2f5d50", //天鵞絨
    "#007b43", //常磐色
    "#7ebeab", //青竹色
    "#98d98e", //若緑
    "#dccb18", //緑黄色
    "#928c36", //鶯色
    "#38b48b", //翡翠色
    "#bce2e8", //水色
    "#a0d8ef", //空色
    "#4c6cb3", //群青色
    "#0d0015", //漆黒
    "#bbbcde", //藤色
    "#595857", //墨
    "#f3f3f3", //乳白色
    "#9d5b8b", //京紫
];

fn main() {
    let mut rng = rand::thread_rng();
    let bg_color = random_color(&mut rng);

    // 画像の初期化と背景色の設定
    let bg_rgb = hex_to_rgb(bg_color);
    let mut img = RgbImage::from_pixel(ICON_SIZE, ICON_SIZE, bg_rgb);

    let corners = vec![
        (0, 0),
        (ICON_SIZE - 1, 0),
        (0, ICON_SIZE - 1),
        (ICON_SIZE - 1, ICON_SIZE - 1),
    ];
    let center = (ICON_SIZE / 2, ICON_SIZE / 2);

    for &corner in &corners {
        let shape_type = rng.gen_range(0..SHAPE_TYPES);
        let size = rng.gen_range(MIN_SHAPE_SIZE..MAX_SHAPE_SIZE);
        let color = random_color(&mut rng);
        let color_rgb = hex_to_rgb(color);

        draw_shape(&mut img, shape_type, corner, size, color_rgb);
    }

    let shape_type = rng.gen_range(0..SHAPE_TYPES);
    let size = rng.gen_range(MIN_SHAPE_SIZE..MAX_SHAPE_SIZE);
    let color = random_color(&mut rng);
    let color_rgb = hex_to_rgb(color);

    draw_shape(&mut img, shape_type, center, size, color_rgb);

    img.save("icon.png").unwrap();
}

fn random_color<R: Rng>(rng: &mut R) -> &'static str {
    let color_index = rng.gen_range(0..COLORS.len());
    COLORS[color_index]
}

fn hex_to_rgb(hex: &str) -> Rgb<u8> {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    Rgb([r, g, b])
}

fn draw_shape(img: &mut RgbImage, shape_type: u8, position: (u32, u32), size: u32, color: Rgb<u8>) {
    match shape_type {
        0 => draw_circle(img, position, size, color),
        1 => draw_semi_circle(img, position, size, color),
        2 => draw_square(img, position, size, color), // 三角形を除外して他の図形の番号を変更
        3 => draw_pentagon(img, position, size, color),
        4 => draw_hexagon(img, position, size, color),
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
