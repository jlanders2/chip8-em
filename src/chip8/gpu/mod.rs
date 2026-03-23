mod constants;

use super::Chip8State;
use crate::chip8::memory;
use constants::{BUILTIN_SPIRTES, BUILTIN_SPRITES_ADDR};

pub const CHIP8_DISPLAY_WIDTH: u8 = 64;
pub const CHIP8_DISPLAY_HEIGHT: u8 = 32;
pub const SPRITE_WIDTH: u8 = 8;

// I probably don't need state here
// refactor to just receive a mutatable buffer
pub fn draw(state: &mut Chip8State, vx: u8, vy: u8, bytes_to_draw: &[u8], bytes_to_draw_len: u8) {
    state.r_v[0xF] = 0;
    let start_x = state.r_v[vx as usize] % CHIP8_DISPLAY_WIDTH;
    let start_y = state.r_v[vy as usize] % CHIP8_DISPLAY_HEIGHT;

    let mut bytes_idx = 0;
    while bytes_idx < bytes_to_draw_len {
        // TODO: this should be a u8 for safety
        let y: u8 = start_y.wrapping_add(bytes_idx) % CHIP8_DISPLAY_HEIGHT;

        let mut bit_idx = 0;
        while bit_idx < SPRITE_WIDTH {
            let x: u8 = start_x.wrapping_add(bit_idx) % CHIP8_DISPLAY_WIDTH;

            let sprite_pixel = ((bytes_to_draw[bytes_idx as usize] >> (7 - bit_idx)) & 1) == 1;
            if sprite_pixel {
                let current_pixel = state.display[y as usize][x as usize];

                if current_pixel {
                    state.r_v[0xF] = 1;
                }

                state.display[y as usize][x as usize] ^= true;
            }

            bit_idx += 1;
        }

        bytes_idx += 1;
    }
}

pub fn clipping_draw(
    state: &mut Chip8State,
    vx: u8,
    vy: u8,
    bytes_to_draw: &[u8],
    bytes_to_draw_len: u8,
) {
    state.r_v[0xF] = 0;
    let start_x = state.r_v[vx as usize] % CHIP8_DISPLAY_WIDTH;
    let start_y = state.r_v[vy as usize] % CHIP8_DISPLAY_HEIGHT;

    let mut bytes_idx = 0;
    while bytes_idx < bytes_to_draw_len {
        let y: u16 = u16::from(start_y + bytes_idx);
        // TODO: general reminder to cleanup type casts to be consistent
        if y >= u16::from(CHIP8_DISPLAY_HEIGHT) {
            break;
        }

        let mut bit_idx = 0;
        while bit_idx < SPRITE_WIDTH {
            let x = u16::from(start_x + bit_idx);
            if x >= u16::from(CHIP8_DISPLAY_WIDTH) {
                break;
            }

            let sprite_pixel = ((bytes_to_draw[bytes_idx as usize] >> (7 - bit_idx)) & 1) == 1;
            if sprite_pixel {
                let current_pixel = state.display[y as usize][x as usize];

                if current_pixel {
                    state.r_v[0xF] = 1;
                }

                state.display[y as usize][x as usize] ^= true;
            }

            bit_idx += 1;
        }

        bytes_idx += 1;
    }
}

pub fn load_builtin_sprites(state: &mut Chip8State) {
    memory::load_bytes(
        state,
        &BUILTIN_SPIRTES,
        BUILTIN_SPIRTES.len(),
        BUILTIN_SPRITES_ADDR as usize,
    );
}

pub fn get_builtin_sprite_addr(sprite_index: u8) -> u8 {
    constants::BUILTIN_SPRITES_ADDR + (sprite_index * constants::BUILTIN_SPRITES_SIZE)
}
