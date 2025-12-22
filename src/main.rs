#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::time::Duration;
use log::info;
use uefi::boot::{self, open_protocol};
use uefi::proto::console::gop::*;
use uefi::{prelude::*, println};

pub mod gfx;

const RECT_WIDTH: usize = 100;
const RECT_HEIGHT: usize = 100;

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

    let mode = gop.current_mode_info();
    println!(
        "Current mode: {}x{} as {:?}",
        mode.resolution().0,
        mode.resolution().1,
        mode.pixel_format()
    );
    boot::stall(Duration::from_secs(2).as_micros() as usize);

    for x in 0..(mode.resolution().0 / RECT_WIDTH) {
        for y in 0..mode.resolution().1 {
            gfx::draw_rect(
                &mut gop,
                (RECT_WIDTH, RECT_HEIGHT),
                (x * RECT_WIDTH, y),
                (100, 149, 237),
                (240, 240, 240),
                1,
            );
            boot::stall(Duration::from_millis(1).as_micros() as usize);
        }
    }

    boot::stall(Duration::from_secs(10).as_micros() as usize);
    Status::SUCCESS
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    info!("Panic: {}", _info);
    loop {}
}
