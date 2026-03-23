use raylib::ffi::KeyboardKey;

pub fn qwerty_to_chip8(keycode: KeyboardKey) -> Result<u8, String> {
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
