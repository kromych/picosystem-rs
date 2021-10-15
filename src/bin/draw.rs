#![no_std]
#![no_main]

use cortex_m_rt::entry;
use display::{HEIGHT, WIDTH};
use log::info;
use rlane_picosystem_games as rpsg;
use rpsg::{display, hardware, time};

use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;

#[link_section = ".boot2"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[entry]
fn main() -> ! {
    let mut hw = hardware::Hardware::new();
    info!("Finished initialization");

    let colors: [Rgb565; 3] = [Rgb565::RED, Rgb565::GREEN, Rgb565::BLUE];

    let mut cursorx = 120;
    let mut cursory = 120;
    let mut color_index = 0;
    let mut cursor_size = 1;

    let mut frame = 0;
    let mut prev_time_us = time::time_us();
    let mut prev_frame = 0;
    loop {
        if hw.input.dpad_left.is_held() && cursorx > 0 {
            cursorx = cursorx - 1;
        }
        if hw.input.dpad_right.is_held() && cursorx < WIDTH - 1 {
            cursorx = cursorx + 1;
        }
        if hw.input.dpad_up.is_held() && cursory > 0 {
            cursory = cursory - 1;
        }
        if hw.input.dpad_down.is_held() && cursory < HEIGHT - 1 {
            cursory = cursory + 1;
        }
        if hw.input.button_y.is_pressed() {
            color_index = (color_index + 1) % colors.len();
        }
        if hw.input.button_x.is_pressed() {
            cursor_size = (cursor_size + 1) % 8;
        }

        let make_cursor = |color| {
            Circle::new(
                Point::new(
                    cursorx as i32 - cursor_size / 2,
                    cursory as i32 - cursor_size / 2,
                ),
                cursor_size as u32 + 1,
            )
            .into_styled(PrimitiveStyleBuilder::new().fill_color(color).build())
        };

        if hw.input.button_a.is_held() {
            make_cursor(colors[color_index])
                .draw(&mut hw.display)
                .unwrap();
        }

        {
            // Selected color
            Rectangle::new(Point::new(0, 0), Size::new(20, 20))
                .into_styled(
                    PrimitiveStyleBuilder::new()
                        .fill_color(colors[color_index])
                        .stroke_color(Rgb565::WHITE)
                        .stroke_width(2)
                        .build(),
                )
                .draw(&mut hw.display)
                .unwrap();
        }

        {
            let color = if frame % 32 < 16 {
                colors[color_index]
            } else {
                Rgb565::WHITE
            };
            let cursor = make_cursor(color);
            cursor
                .draw(&mut display::XorDisplay::new(&mut hw.display))
                .unwrap();
            hw.display.flush();
            cursor
                .draw(&mut display::XorDisplay::new(&mut hw.display))
                .unwrap();
        }

        let now = time::time_us();
        if now - prev_time_us > 1000_000 {
            let frame_time = (now - prev_time_us) / (frame - prev_frame) as u32;
            info!("Frame time: {} us", frame_time);
            prev_frame = frame;
            prev_time_us = now;
        }
        frame += 1;
    }
}
