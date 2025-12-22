use uefi::proto::console::gop::*;

pub type Color = (u8, u8, u8);

pub fn draw_rect(
    gop: &mut GraphicsOutput,
    dim: (usize, usize),
    pos: (usize, usize),
    line_color: Color,
    fill_color: Color,
    line_thickness: usize,
) {
    for y in pos.1..pos.1 + dim.1 {
        for x in pos.0..pos.0 + dim.0 {
            let is_border = x < pos.0 + line_thickness
                || x >= pos.0 + dim.0 - line_thickness
                || y < pos.1 + line_thickness
                || y >= pos.1 + dim.1 - line_thickness;

            set_pixel_rgb(gop, (x, y), if is_border { line_color } else { fill_color });
        }
    }
}

pub fn clear(gop: &mut GraphicsOutput, color: Color) {
    let (width, height) = gop.current_mode_info().resolution();
    for y in 0..height {
        for x in 0..width {
            set_pixel_rgb(gop, (x, y), color);
        }
    }
}

pub fn set_pixel_rgb(gop: &mut GraphicsOutput, pos: (usize, usize), color: Color) {
    let mode = gop.current_mode_info();
    let (width, height) = mode.resolution();

    if pos.0 >= width || pos.1 >= height {
        return;
    }

    let pixel_format = mode.pixel_format();
    let stride = mode.stride(); // pixels per scanline
    let mut fb = gop.frame_buffer();

    let bytes_per_pixel = 4;
    let index = (pos.1 * stride + pos.0) * bytes_per_pixel;

    match pixel_format {
        PixelFormat::Rgb => unsafe {
            fb.write_byte(index, color.0);
            fb.write_byte(index + 1, color.1);
            fb.write_byte(index + 2, color.2);
            fb.write_byte(index + 3, 0);
        },
        PixelFormat::Bgr => unsafe {
            fb.write_byte(index, color.2);
            fb.write_byte(index + 1, color.1);
            fb.write_byte(index + 2, color.0);
            fb.write_byte(index + 3, 0);
        },
        _ => {}
    }
}
