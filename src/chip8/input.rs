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
        let Ok(mapped_key) = qwerty_to_chip8(key) else {
            continue;
        };
        state.input[mapped_key as usize] = rl_hdl.is_key_down(key);
    }
}

fn qwerty_to_chip8(keycode: KeyboardKey) -> Result<u8, String> {
    match keycode {
        KeyboardKey::KEY_ONE => Ok(0x1),
        KeyboardKey::KEY_TWO => Ok(0x2),
        KeyboardKey::KEY_THREE => Ok(0x3),
        KeyboardKey::KEY_FOUR => Ok(0xC),
        KeyboardKey::KEY_Q => Ok(0x4),
        KeyboardKey::KEY_W => Ok(0x5),
        KeyboardKey::KEY_E => Ok(0x6),
        KeyboardKey::KEY_R => Ok(0xD),
        KeyboardKey::KEY_A => Ok(0x7),
        KeyboardKey::KEY_S => Ok(0x8),
        KeyboardKey::KEY_D => Ok(0x9),
        KeyboardKey::KEY_F => Ok(0xE),
        KeyboardKey::KEY_Z => Ok(0xA),
        KeyboardKey::KEY_X => Ok(0x0),
        KeyboardKey::KEY_C => Ok(0xB),
        KeyboardKey::KEY_V => Ok(0xF),
        _ => Err(format!("Un-mapped keycode {}", keycode as u8)),
    }
}
