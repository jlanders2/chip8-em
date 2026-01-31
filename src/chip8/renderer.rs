use super::Chip8State;
use super::gpu::{CHIP8_DISPLAY_HEIGHT, CHIP8_DISPLAY_WIDTH};
use raylib::{RaylibHandle, RaylibThread, color::Color, prelude::RaylibDraw};

pub static DISPLAY_WIDTH: u16 = 640;
pub static DISPLAY_HEIGHT: u16 = 480;

pub fn render(state: &Chip8State, rl_handle: &mut RaylibHandle, rl_thread: &RaylibThread) {
    let mut d = rl_handle.begin_drawing(rl_thread);

    // d.clear_background(Color::BLACK);

    let scale_x = DISPLAY_WIDTH / u16::from(CHIP8_DISPLAY_WIDTH);
    let scale_y = DISPLAY_HEIGHT / u16::from(CHIP8_DISPLAY_HEIGHT);

    for y in 0..CHIP8_DISPLAY_HEIGHT {
        for x in 0..CHIP8_DISPLAY_WIDTH {
            // fix to render color based on exact bit for pixels
            let color: Color = if state.display[y as usize][x as usize] {
                Color::WHITE
            } else {
                Color::BLACK
            };
            d.draw_rectangle(
                i32::from(u16::from(x) * scale_x),
                i32::from(u16::from(y) * scale_y),
                i32::from(scale_x),
                i32::from(scale_y),
                color,
            );
        }
    }
}
