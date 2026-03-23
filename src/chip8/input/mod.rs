mod constants;
mod internal;

use constants::RL_KEY_LAYOUT;
use internal::qwerty_to_chip8;
use raylib::RaylibHandle;

use super::Chip8State;

pub fn handle_input(state: &mut Chip8State, rl_hdl: &mut RaylibHandle) {
    for key in RL_KEY_LAYOUT {
        let Ok(mapped_key) = qwerty_to_chip8(key) else {
            continue;
        };
        state.input[mapped_key as usize] = rl_hdl.is_key_down(key);
    }
}
