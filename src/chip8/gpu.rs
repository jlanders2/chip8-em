use crate::chip8::memory;

use super::Chip8State;

pub static CHIP8_DISPLAY_WIDTH: u8 = 64;
pub static CHIP8_DISPLAY_HEIGHT: u8 = 32;
pub static SPRITE_WIDTH: u8 = 8;

// I probably don't need state here
// refactor to just receive a mutatable buffer
pub fn draw(state: &mut Chip8State, vx: u8, vy: u8, bytes_to_draw: &[u8], bytes_to_draw_len: u8) {
    state.r_v[0xF] = 0;
    let start_x = state.r_v[vx as usize] % CHIP8_DISPLAY_WIDTH as u8;
    let start_y = state.r_v[vy as usize] % CHIP8_DISPLAY_HEIGHT as u8;

    let mut bytes_idx = 0;
    while bytes_idx < bytes_to_draw_len {
        // TODO: this should be a u8 for safety
        let y: u8 = start_y.wrapping_add(bytes_idx) % CHIP8_DISPLAY_HEIGHT as u8;

        let mut bit_idx = 0;
        while bit_idx < SPRITE_WIDTH {
            let x: u8 = start_x.wrapping_add(bit_idx) % CHIP8_DISPLAY_WIDTH as u8;

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
    let start_x = state.r_v[vx as usize] % CHIP8_DISPLAY_WIDTH as u8;
    let start_y = state.r_v[vy as usize] % CHIP8_DISPLAY_HEIGHT as u8;

    let mut bytes_idx = 0;
    while bytes_idx < bytes_to_draw_len {
        let y: u16 = (start_y + bytes_idx) as u16;
        // TODO: general reminder to cleanup type casts to be consistent
        if y >= CHIP8_DISPLAY_HEIGHT as u16 {
            break;
        }

        let mut bit_idx = 0;
        while bit_idx < SPRITE_WIDTH {
            let x = (start_x + bit_idx) as u16;
            if x >= CHIP8_DISPLAY_WIDTH as u16 {
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
