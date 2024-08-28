use image::{ImageBuffer, Rgb};

const FONT_SIZE: u32 = 32;
const SCALE: u32 = 3;
const CHAR_SPACING: u32 = 4;

#[derive(Clone, Copy)]
enum Primitive {
    Line(f32, f32, f32, f32), // x1, y1, x2, y2
    QuadCurve(f32, f32, f32, f32, f32, f32), // x1, y1, cx, cy, x2, y2
}

struct Glyph {
    primitives: Vec<Primitive>,
    width: f32,
}

fn create_font() -> std::collections::HashMap<char, Glyph> {
    let mut font = std::collections::HashMap::new();

    // Define 'A'
    font.insert('A', Glyph {
        primitives: vec![
            Primitive::Line(0.0, 1.0, 0.5, 0.0),
            Primitive::Line(0.5, 0.0, 1.0, 1.0),
            Primitive::Line(0.25, 0.5, 0.75, 0.5),
        ],
        width: 1.0,
    });

    // Define 'B'
    font.insert('B', Glyph {
        primitives: vec![
            Primitive::Line(0.0, 0.0, 0.0, 1.0),
            Primitive::Line(0.0, 0.0, 0.8, 0.0),
            Primitive::QuadCurve(0.8, 0.0, 1.0, 0.25, 0.8, 0.5),
            Primitive::Line(0.0, 0.5, 0.8, 0.5),
            Primitive::QuadCurve(0.8, 0.5, 1.0, 0.75, 0.8, 1.0),
            Primitive::Line(0.8, 1.0, 0.0, 1.0),
        ],
        width: 1.0,
    });

    // Define 'C'
    font.insert('C', Glyph {
        primitives: vec![
            Primitive::QuadCurve( // top curve
                0.8, 0.18,
                0.5, 0.018,
                0.18, 0.5,
            ),

        Primitive::QuadCurve( // bottom curve
            0.18, 0.5,
            0.5, 0.88,
            0.8, 0.72,
        )],
        width: 1.0,
    });

    font
}


fn render_glyph(glyph: &Glyph, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x_offset: u32, y_offset: u32) {
    for primitive in &glyph.primitives {
        match primitive {
            Primitive::Line(x1, y1, x2, y2) => {
                draw_line(
                    image,
                    (x_offset as f32 + x1 * FONT_SIZE as f32 * SCALE as f32) as u32,
                    (y_offset as f32 + y1 * FONT_SIZE as f32 * SCALE as f32) as u32,
                    (x_offset as f32 + x2 * FONT_SIZE as f32 * SCALE as f32) as u32,
                    (y_offset as f32 + y2 * FONT_SIZE as f32 * SCALE as f32) as u32,
                );
            }
            Primitive::QuadCurve(x1, y1, cx, cy, x2, y2) => {
                draw_quad_curve(
                    image,
                    (x_offset as f32 + x1 * FONT_SIZE as f32 * SCALE as f32) as u32,
                    (y_offset as f32 + y1 * FONT_SIZE as f32 * SCALE as f32) as u32,
                    (x_offset as f32 + cx * FONT_SIZE as f32 * SCALE as f32) as u32,
                    (y_offset as f32 + cy * FONT_SIZE as f32 * SCALE as f32) as u32,
                    (x_offset as f32 + x2 * FONT_SIZE as f32 * SCALE as f32) as u32,
                    (y_offset as f32 + y2 * FONT_SIZE as f32 * SCALE as f32) as u32,
                );
            }
        }
    }
}

fn draw_line(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x1: u32, y1: u32, x2: u32, y2: u32) {
    // Bresenham's line algorithm
    let dx = (x2 as i32 - x1 as i32).abs();
    let dy = (y2 as i32 - y1 as i32).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x1 as i32;
    let mut y = y1 as i32;

    loop {
        if x >= 0 && x < image.width() as i32 && y >= 0 && y < image.height() as i32 {
            image.put_pixel(x as u32, y as u32, Rgb([0, 0, 0]));
        }

        if x == x2 as i32 && y == y2 as i32 {
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

fn draw_quad_curve(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x1: u32, y1: u32, cx: u32, cy: u32, x2: u32, y2: u32) {
    for t in (0..=100).map(|t| t as f32 / 100.0) {
        let x = (1.0 - t).powi(2) * x1 as f32 + 2.0 * (1.0 - t) * t * cx as f32 + t.powi(2) * x2 as f32;
        let y = (1.0 - t).powi(2) * y1 as f32 + 2.0 * (1.0 - t) * t * cy as f32 + t.powi(2) * y2 as f32;
        if x >= 0.0 && x < image.width() as f32 && y >= 0.0 && y < image.height() as f32 {
            image.put_pixel(x as u32, y as u32, Rgb([0, 0, 0]));
        }
    }
}

fn render_text(font: &std::collections::HashMap<char, Glyph>, text: &str) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = (FONT_SIZE * SCALE + CHAR_SPACING) * text.len() as u32;
    let height = FONT_SIZE * SCALE;
    let mut image = ImageBuffer::new(width, height);

    // Fill with white background
    for pixel in image.pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }

    for (i, c) in text.chars().enumerate() {
        if let Some(glyph) = font.get(&c) {
            let x_offset = i as u32 * (FONT_SIZE * SCALE + CHAR_SPACING);
            render_glyph(glyph, &mut image, x_offset, 0);
        }
    }

    image
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let font = create_font();
    let text = "ABC";  // Only A and B are defined in this example
    let img = render_text(&font, text);
    img.save("output.png")?;
    println!("Image saved as output.png");
    Ok(())
}