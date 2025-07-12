use image::{RgbImage, Rgb};

type Point = (i32, i32);

// Polígonos
fn get_polygons() -> Vec<(Vec<Point>, Rgb<u8>)> {
    vec![
        (vec![(165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
              (230, 360), (250, 380), (220, 385), (205, 410), (193, 383)],
         Rgb([255, 0, 0])), // Estrella

        (vec![(321, 335), (288, 286), (339, 251), (374, 302)],
         Rgb([0, 255, 0])), // Cuadrado

        (vec![(377, 249), (411, 197), (436, 249)],
         Rgb([0, 0, 255])), // Triangulo

        (vec![
            (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37),
            (660, 52), (750, 145), (761, 179), (672, 192), (659, 214),
            (615, 214), (632, 230), (580, 230), (597, 215), (552, 214),
            (517, 144), (466, 180)],
         Rgb([255, 255, 0])), // tetera
    ]
}

fn get_hole() -> Vec<Point> {
    vec![(682, 175), (708, 120), (735, 148), (739, 170)]
}

// Dibuja línea con algoritmo de Bresenham
fn draw_line(img: &mut RgbImage, p1: Point, p2: Point, color: Rgb<u8>) {
    let (mut x0, mut y0) = p1;
    let (x1, y1) = p2;
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x0 >= 0 && x0 < img.width() as i32 && y0 >= 0 && y0 < img.height() as i32 {
            img.put_pixel(x0 as u32, y0 as u32, color);
        }
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

// Rellenado por scanline simple
fn scanline_fill(img: &mut RgbImage, polygon: &[Point], color: Rgb<u8>) {
    let height = img.height() as i32;
    let mut edges = vec![];

    for i in 0..polygon.len() {
        let (x0, y0) = polygon[i];
        let (x1, y1) = polygon[(i + 1) % polygon.len()];
        if y0 == y1 { continue; }
        let (x0, y0, x1, y1) = if y0 < y1 {
            (x0, y0, x1, y1)
        } else {
            (x1, y1, x0, y0)
        };
        edges.push((y0, y1, x0 as f32, (x1 - x0) as f32 / (y1 - y0) as f32));
    }

    for y in 0..height {
        let mut intersections = vec![];
        for &(y0, y1, mut x, inv_slope) in &edges {
            if y >= y0 && y < y1 {
                x += (y - y0) as f32 * inv_slope;
                intersections.push(x as i32);
            }
        }

        intersections.sort();
        for pair in intersections.chunks(2) {
            if pair.len() == 2 {
                for x in pair[0]..pair[1] {
                    if x >= 0 && x < img.width() as i32 {
                        img.put_pixel(x as u32, y as u32, color);
                    }
                }
            }
        }
    }
}

fn main() {
    let width = 800;
    let height = 600;
    let mut img = RgbImage::new(width, height);

    // Rellenar polígonos
    let polygons = get_polygons();
    let hole = get_hole();

    // Dibujar fondo blanco
    for pixel in img.pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }

    // Polígono con agujero
    let (outer_poly, color) = &polygons[3];

    scanline_fill(&mut img, outer_poly, *color);
    scanline_fill(&mut img, &hole, Rgb([255, 255, 255])); 

    // Dibujar y rellenar otros
    for (poly, color) in polygons.iter().enumerate().filter(|(i, _)| *i != 3).map(|(_, v)| v) {
        scanline_fill(&mut img, poly, *color);
    }

    // Dibujar bordes
    for (poly, _) in &polygons {
        for i in 0..poly.len() {
            draw_line(&mut img, poly[i], poly[(i + 1) % poly.len()], Rgb([0, 0, 0]));
        }
    }

    // Borde del agujero
    for i in 0..hole.len() {
        draw_line(&mut img, hole[i], hole[(i + 1) % hole.len()], Rgb([0, 0, 0]));
    }

    img.save("out.png").unwrap();
}
