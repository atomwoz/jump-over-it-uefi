use alloc::vec;
use alloc::vec::Vec;
use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use uefi::proto::console::gop::{GraphicsOutput, PixelFormat};

pub struct UefiDrawTarget<'a> {
    gop: &'a mut GraphicsOutput,
    buffer: Vec<u8>,
    width: usize,
    height: usize,
    stride: usize,
}

impl<'a> UefiDrawTarget<'a> {
    pub fn new(gop: &'a mut GraphicsOutput) -> Self {
        let mode = gop.current_mode_info();
        let (width, height) = mode.resolution();
        let stride = mode.stride();

        let buffer_size = stride * height * 4;
        let buffer = vec![0; buffer_size];

        Self {
            gop,
            buffer,
            width,
            height,
            stride,
        }
    }

    pub fn flush(&mut self) {
        let mut fb = self.gop.frame_buffer();

        unsafe {
            let src = self.buffer.as_ptr();
            let dst = fb.as_mut_ptr();
            let len = self.buffer.len();
            core::ptr::copy_nonoverlapping(src, dst, len);
        }
    }
}

impl OriginDimensions for UefiDrawTarget<'_> {
    fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
}

impl DrawTarget for UefiDrawTarget<'_> {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let pixel_format = self.gop.current_mode_info().pixel_format();

        for Pixel(point, color) in pixels.into_iter() {
            if point.x < 0 || point.y < 0 {
                continue;
            }
            let x = point.x as usize;
            let y = point.y as usize;

            if x >= self.width || y >= self.height {
                continue;
            }

            let index = (y * self.stride + x) * 4;

            match pixel_format {
                PixelFormat::Rgb => {
                    self.buffer[index] = color.r();
                    self.buffer[index + 1] = color.g();
                    self.buffer[index + 2] = color.b();
                }
                PixelFormat::Bgr => {
                    self.buffer[index] = color.b();
                    self.buffer[index + 1] = color.g();
                    self.buffer[index + 2] = color.r();
                }
                _ => {
                    // Fallback or other formats
                    self.buffer[index] = color.r();
                    self.buffer[index + 1] = color.g();
                    self.buffer[index + 2] = color.b();
                }
            }
        }
        Ok(())
    }
}
