use raylib::{RaylibHandle, ffi::KeyboardKey};

use super::Chip8State;

static RL_KEY_LAYOUT: [KeyboardKey; 16] = [
    KeyboardKey::KEY_ONE,
    KeyboardKey::KEY_TWO,
    KeyboardKey::KEY_THREE,
    KeyboardKey::KEY_FOUR,
    KeyboardKey::KEY_Q,
    KeyboardKey::KEY_W,
    KeyboardKey::KEY_E,
    KeyboardKey::KEY_R,
    KeyboardKey::KEY_A,
    KeyboardKey::KEY_S,
    KeyboardKey::KEY_D,
    KeyboardKey::KEY_F,
    KeyboardKey::KEY_Z,
    KeyboardKey::KEY_X,
    KeyboardKey::KEY_C,
    KeyboardKey::KEY_V,
];

pub fn handle_input(state: &mut Chip8State, rl_hdl: &mut RaylibHandle) {
    for key in RL_KEY_LAYOUT {
        let mapped_key = qwerty_to_chip8(key);
        // let chip8_input_index = chip8_to_index(mapped_key);
        if rl_hdl.is_key_down(key) {
            state.input[mapped_key as usize] = true;
        } else {
            state.input[mapped_key as usize] = false;
        }
    }
}

fn qwerty_to_chip8(keycode: KeyboardKey) -> u8 {
    match keycode {
        KeyboardKey::KEY_ONE => return 0x1,
        KeyboardKey::KEY_TWO => return 0x2,
        KeyboardKey::KEY_THREE => return 0x3,
        KeyboardKey::KEY_FOUR => return 0xC,
        KeyboardKey::KEY_Q => return 0x4,
        KeyboardKey::KEY_W => return 0x5,
        KeyboardKey::KEY_E => return 0x6,
        KeyboardKey::KEY_R => return 0xD,
        KeyboardKey::KEY_A => return 0x7,
        KeyboardKey::KEY_S => return 0x8,
        KeyboardKey::KEY_D => return 0x9,
        KeyboardKey::KEY_F => return 0xE,
        KeyboardKey::KEY_Z => return 0xA,
        KeyboardKey::KEY_X => return 0x0,
        KeyboardKey::KEY_C => return 0xB,
        KeyboardKey::KEY_V => return 0xF,
        _ => return 0,
    }
}

// fn chip8_to_index(chip8_key: u8) -> u8 {
//     match chip8_key {
//         0x1 => return 0x0,
//         0x2 => return 0x1,
//         0x3 => return 0x2,
//         0xC => return 0x3,
//         0x4 => return 0x4,
//         0x5 => return 0x5,
//         0x6 => return 0x6,
//         0xD => return 0x7,
//         0x7 => return 0x8,
//         0x8 => return 0x9,
//         0x9 => return 0xA,
//         0xE => return 0xB,
//         0xA => return 0xC,
//         0x0 => return 0xD,
//         0xB => return 0xE,
//         0xF => return 0xF,
//         _ => panic!("Unknown Chip8 Key {}", chip8_key),
//     }
// }
