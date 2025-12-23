#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use core::time::Duration;

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle, RoundedRectangle},
};
use uefi::proto::console::gop::*;
use uefi::proto::console::text::{Input, Key, ScanCode};
use uefi::{prelude::*, println};

use crate::gfx::UefiDrawTarget;

pub mod gfx;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let gop_handle = boot::get_handle_for_protocol::<GraphicsOutput>().unwrap();
    let mut gop = unsafe {
        boot::open_protocol::<GraphicsOutput>(
            boot::OpenProtocolParams {
                handle: gop_handle,
                agent: boot::image_handle(),
                controller: None,
            },
            boot::OpenProtocolAttributes::GetProtocol,
        )
    }
    .unwrap();

    let input_handle = boot::get_handle_for_protocol::<Input>().unwrap();
    let mut input = unsafe {
        boot::open_protocol::<Input>(
            boot::OpenProtocolParams {
                handle: input_handle,
                agent: boot::image_handle(),
                controller: None,
            },
            boot::OpenProtocolAttributes::GetProtocol,
        )
    }
    .unwrap();

    input.reset(false).unwrap();

    let mut display = UefiDrawTarget::new(&mut gop);

    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::RED)
        .stroke_width(3)
        .fill_color(Rgb888::GREEN)
        .build();

    let mut pt = Point::new(50, 500);
    let mut velocity_y = 0;
    const GRAVITY: i32 = 1;
    const JUMP_FORCE: i32 = -20;
    const GROUND_Y: i32 = 500;

    loop {
        // Handle input
        let mut quit = false;
        while let Ok(Some(key)) = input.read_key() {
            match key {
                Key::Special(ScanCode::UP) => {
                    if pt.y >= GROUND_Y {
                        velocity_y = JUMP_FORCE;
                    }
                }
                Key::Special(ScanCode::LEFT) => pt.x -= 10,
                Key::Special(ScanCode::RIGHT) => pt.x += 10,
                Key::Printable(c) if c == 'q' => {
                    quit = true;
                    break;
                }
                _ => {}
            }
        }
        if quit {
            break;
        }

        // Physics
        velocity_y += GRAVITY;
        pt.y += velocity_y;

        if pt.y > GROUND_Y {
            pt.y = GROUND_Y;
            velocity_y = 0;
        }

        display.clear(Rgb888::BLACK).unwrap();

        RoundedRectangle::with_equal_corners(
            Rectangle::new(pt, Size::new(48, 32)),
            Size::new(10, 10),
        )
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

        display.flush();

        boot::stall(100 as usize);
    }

    Status::SUCCESS
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("Panic: {}", _info);
    loop {}
}
