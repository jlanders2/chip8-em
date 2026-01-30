use crate::chip8::memory;

use super::Chip8State;

pub static CHIP8_DISPLAY_WIDTH: i32 = 64;
pub static CHIP8_DISPLAY_HEIGHT: i32 = 32;
pub static SPRITE_WIDTH: u8 = 8;

// I probably don't need state here
// refactor to just receive a mutatable buffer
pub fn draw(state: &mut Chip8State, vx: u8, vy: u8, bytes_to_draw: &[u8], bytes_to_draw_len: u8) {
    let mut bytes_idx = 0;
    let start_x = state.r_v[vx as usize];
    let start_y = state.r_v[vy as usize];
    for y in start_y..start_y + bytes_to_draw_len {
        if (y as i32) < CHIP8_DISPLAY_HEIGHT {
            // 8 is the hardcoded sprite width, has to atleast have 1 8 bit value to display
            let mut bit_idx = 0;
            for x in start_x..start_x + SPRITE_WIDTH {
                if (x as i32) < CHIP8_DISPLAY_WIDTH {
                    let sprite_pixel = ((bytes_to_draw[bytes_idx] >> (7 - bit_idx)) & 1) == 1;
                    let current_pixel = state.display[y as usize][x as usize];
                    let new_pixel = current_pixel ^ sprite_pixel;

                    state.display[y as usize][x as usize] = new_pixel;

                    if new_pixel != current_pixel {
                        state.r_v[0xF] = 1;
                    }

                    bit_idx += 1;
                }
            }
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
    return BUILTIN_SPRITES_ADDR + (sprite_index * BUILTIN_SPRITES_SIZE);
}

static BUILTIN_SPRITES_ADDR: u8 = 0;
static BUILTIN_SPRITES_SIZE: u8 = 5;
static BUILTIN_SPIRTES: [u8; 80] = [
    0xF0, // 0
    0x90, // 0
    0x90, // 0
    0x90, // 0
    0xF0, // 0
    0x20, // 1
    0x60, // 1
    0x20, // 1
    0x20, // 1
    0x70, // 1
    0xF0, // 2
    0x10, // 2
    0xF0, // 2
    0x80, // 2
    0xF0, // 2
    0xF0, // 3
    0x10, // 3
    0xF0, // 3
    0x10, // 3
    0xF0, // 3
    0x90, // 4
    0x90, // 4
    0xF0, // 4
    0x10, // 4
    0x10, // 4
    0xF0, // 5
    0x80, // 5
    0xF0, // 5
    0x10, // 5
    0xF0, // 5
    0xF0, // 6
    0x80, // 6
    0xF0, // 6
    0x90, // 6
    0xF0, // 6
    0xF0, // 7
    0x10, // 7
    0x20, // 7
    0x40, // 7
    0x40, // 7
    0xF0, // 8
    0x90, // 8
    0xF0, // 8
    0x90, // 8
    0xF0, // 8
    0xF0, // 9
    0x90, // 9
    0xF0, // 9
    0x10, // 9
    0xF0, // 9
    0xF0, // A
    0x90, // A
    0xF0, // A
    0x90, // A
    0x90, // A
    0xE0, // B
    0x90, // B
    0xE0, // B
    0x90, // B
    0xE0, // B
    0xF0, // C
    0x80, // C
    0x80, // C
    0x80, // C
    0xF0, // C
    0xE0, // D
    0x90, // D
    0x90, // D
    0x90, // D
    0xE0, // D
    0xF0, // E
    0x80, // E
    0xF0, // E
    0x80, // E
    0xF0, // E
    0xF0, // F
    0x80, // F
    0xF0, // F
    0x80, // F
    0x80, // F
];
