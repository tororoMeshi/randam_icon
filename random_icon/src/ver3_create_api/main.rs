use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use image::{Rgb, RgbImage};
use rand::Rng;
use std::f32::consts::PI;
use std::io::Cursor;

// 定数パラメータの定義
const ICON_SIZE: u32 = 500;
const MIN_SHAPE_SIZE: u32 = 150;
const MAX_SHAPE_SIZE: u32 = 250;
const SHAPE_TYPES: u8 = 5;
const COLORS: [&str; 31] = [
    "#f19072", "#e4dc8a", "#f8f4e6", "#b7282e", "#f09199", "#fef4f4", "#c39143", "#8a3b00",
    "#f08300", "#ed6d3d", "#ee7800", "#eb6101", "#ffd900", "#ffec47", "#f8b500", "#e6b422",
    "#2f5d50", "#007b43", "#7ebeab", "#98d98e", "#dccb18", "#928c36", "#38b48b", "#bce2e8",
    "#a0d8ef", "#4c6cb3", "#0d0015", "#bbbcde", "#595857", "#f3f3f3", "#9d5b8b",
];

async fn generate_icon() -> impl Responder {
    let mut rng = rand::thread_rng();
    let bg_color = random_color(&mut rng);

    // 画像の初期化と背景色の設定
    let bg_rgb = hex_to_rgb(bg_color);
    let mut img = RgbImage::from_pixel(ICON_SIZE, ICON_SIZE, bg_rgb);

    // コーナーとセンターの位置を定義
    let corners = vec![
        (0, 0),
        (ICON_SIZE - 1, 0),
        (0, ICON_SIZE - 1),
        (ICON_SIZE - 1, ICON_SIZE - 1),
    ];
    let center = (ICON_SIZE / 2, ICON_SIZE / 2);

    // 各コーナーに図形を描画
    for &corner in &corners {
        draw_random_shape(&mut img, corner, &mut rng);
    }

    // センターに図形を描画
    draw_random_shape(&mut img, center, &mut rng);

    // 画像をバイナリ形式で返す
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    img.write_to(&mut cursor, image::ImageOutputFormat::Png).unwrap();

    HttpResponse::Ok()
        .content_type("image/png")
        .body(buffer)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/generate-icon", web::get().to(generate_icon))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// ランダムな図形を描画する関数
fn draw_random_shape<R: Rng>(img: &mut RgbImage, position: (u32, u32), rng: &mut R) {
    let shape_type = rng.gen_range(0..SHAPE_TYPES);
    let size = rng.gen_range(MIN_SHAPE_SIZE..MAX_SHAPE_SIZE);
    let angle = rng.gen_range(0.0..2.0 * PI);
    let color = random_color(rng);
    let color_rgb = hex_to_rgb(color);

    draw_shape(img, shape_type, position, size, angle, color_rgb);
}

// ランダムな色を取得する関数
fn random_color<R: Rng>(rng: &mut R) -> &'static str {
    let color_index = rng.gen_range(0..COLORS.len());
    COLORS[color_index]
}

// HEXカラーコードをRGBに変換する関数
fn hex_to_rgb(hex: &str) -> Rgb<u8> {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    Rgb([r, g, b])
}

// 図形を描画する関数
fn draw_shape(
    img: &mut RgbImage,
    shape_type: u8,
    position: (u32, u32),
    size: u32,
    angle: f32,
    color: Rgb<u8>,
) {
    match shape_type {
        0 => draw_circle(img, position, size, color),
        1 => draw_semi_circle(img, position, size, angle, color),
        2 => draw_square(img, position, size, angle, color),
        3 => draw_pentagon(img, position, size, angle, color),
        4 => draw_hexagon(img, position, size, angle, color),
        _ => (),
    }
}

// 円を描画する関数
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

// 半円を描画する関数
fn draw_semi_circle(img: &mut RgbImage, position: (u32, u32), size: u32, angle: f32, color: Rgb<u8>) {
    let (cx, cy) = position;
    let radius = size as f32;
    for x in cx.saturating_sub(size)..=cx.saturating_add(size) {
        for y in cy.saturating_sub(size)..=cy.saturating_add(size) {
            let dx = x as f32 - cx as f32;
            let dy = y as f32 - cy as f32;
            let distance_squared = dx * dx + dy * dy;
            if distance_squared <= radius * radius {
                let point_angle = (dy).atan2(dx);
                let adjusted_angle = (point_angle - angle + 2.0 * PI) % (2.0 * PI);
                if (PI / 2.0..=3.0 * PI / 2.0).contains(&adjusted_angle) && x < ICON_SIZE && y < ICON_SIZE {
                    img.put_pixel(x, y, color);
                }
            }
        }
    }
}

// 四角形を描画する関数
fn draw_square(
    img: &mut RgbImage,
    position: (u32, u32),
    size: u32,
    angle: f32,
    color: Rgb<u8>,
) {
    let (cx, cy) = position;
    let half_size = size as f32 / 2.0;
    let points: Vec<(i32, i32)> = (0..4)
        .map(|i| {
            let theta = angle + (PI / 4.0) * (2.0 * i as f32);
            let x = cx as f32 + half_size * theta.cos();
            let y = cy as f32 + half_size * theta.sin();
            (x.round() as i32, y.round() as i32)
        })
        .collect();

    fill_polygon(img, &points, color);
}

// 五角形を描画する関数
fn draw_pentagon(
    img: &mut RgbImage,
    position: (u32, u32),
    size: u32,
    angle: f32,
    color: Rgb<u8>,
) {
    let (cx, cy) = position;
    let points: Vec<(i32, i32)> = (0..5)
        .map(|i| {
            let theta = angle + (PI * 2.0 / 5.0) * (i as f32);
            let x = cx as f32 + (size as f32 * theta.cos());
            let y = cy as f32 + (size as f32 * theta.sin());
            (x.round() as i32, y.round() as i32)
        })
        .collect();

    fill_polygon(img, &points, color);
}

// 六角形を描画する関数
fn draw_hexagon(
    img: &mut RgbImage,
    position: (u32, u32),
    size: u32,
    angle: f32,
    color: Rgb<u8>,
) {
    let (cx, cy) = position;
    let points: Vec<(i32, i32)> = (0..6)
        .map(|i| {
            let theta = angle + (PI * 2.0 / 6.0) * (i as f32);
            let x = cx as f32 + (size as f32 * theta.cos());
            let y = cy as f32 + (size as f32 * theta.sin());
            (x.round() as i32, y.round() as i32)
        })
        .collect();

    fill_polygon(img, &points, color);
}

// ポリゴンを塗りつぶす関数
fn fill_polygon(img: &mut RgbImage, points: &[(i32, i32)], color: Rgb<u8>) {
    let (min_y, max_y) = points.iter().fold((i32::MAX, i32::MIN), |(min_y, max_y), &(_, y)| {
        (min_y.min(y), max_y.max(y))
    });

    for y in min_y..=max_y {
        let mut intersections = vec![];
        for i in 0..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[(i + 1) % points.len()];
            if (y1 <= y && y < y2) || (y2 <= y && y < y1) {
                let x = x1 + (y - y1) * (x2 - x1) / (y2 - y1);
                intersections.push(x);
            }
        }
        intersections.sort();
        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                let x_start = intersections[i];
                let x_end = intersections[i + 1];
                for x in x_start..=x_end {
                    if x >= 0 && x < ICON_SIZE as i32 && y >= 0 && y < ICON_SIZE as i32 {
                        img.put_pixel(x as u32, y as u32, color);
                    }
                }
            }
        }
    }
}
