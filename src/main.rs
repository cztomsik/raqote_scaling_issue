use raqote::*;

fn main() {
    let mut dt = DrawTarget::new(600, 600);

    // gen checkboard image
    //let (width, height, square_size) = (64, 64, 32);
    //let (width, height, square_size) = (32, 32, 16);
    //let (width, height, square_size) = (16, 16, 8);
    //let (width, height, square_size) = (8, 8, 4);
    //let (width, height, square_size) = (4, 4, 2);
    let (width, height, square_size) = (2, 2, 1);
    let data = gen_checkerboard(width, height, square_size);
    let data = unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u32, data.len() / 4) };
    let image = Image { width, height, data };

    // draw it scaled
    let source = Source::Image(image, ExtendMode::Pad, FilterMode::Nearest, Transform::identity().post_scale(width as f32 / 600., height as f32 / 600.));
    dt.fill_rect(0., 0., 600., 600., &source, &DrawOptions::new());

    dt.write_png("out.png").unwrap();
}

// square_size is expected to be power of 2
fn gen_checkerboard(width: i32, height: i32, square_size: i32) -> Box<[u8]> {
    let mut data = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let v = if x & square_size != y & square_size { 0xFF } else { 0x00 };
            data.extend(vec![v, v, v, 0xFF]);
        }
    }

    data.into_boxed_slice()
}
