#![allow(clippy::many_single_char_names)]
use crate::chip8::{Chip8Quirks, gpu};

use super::Chip8State;
use super::memory::read_n_bytes;

// Need to come back and add good comments for each of the match patterns
pub fn execute_instruction(state: &mut Chip8State, instruction: u16, quirks: Chip8Quirks) {
    let c = ((instruction & 0xF000) >> 12) as u8;
    let x = ((instruction & 0x0F00) >> 8) as u8;
    let y = ((instruction & 0x00F0) >> 4) as u8;
    let d = (instruction & 0x000F) as u8;
    let n = d;
    let kk = (instruction & 0x00FF) as u8;
    let nnn = instruction & 0x0FFF;

    match (c, x, y, d) {
        (0x0, _, 0xE, 0x0) => {
            for row in &mut state.display {
                for col in row {
                    *col = false;
                }
            }
        }
        (0x0, _, 0xE, 0xE) => {
            state.r_sp -= 1;
            state.r_pc = state.stack[state.r_sp as usize];
        }
        (0x1, _, _, _) => state.r_pc = nnn,
        (0x2, _, _, _) => {
            state.stack[state.r_sp as usize] = state.r_pc;
            state.r_sp += 1;
            state.r_pc = nnn;
        }
        (0x3, _, _, _) => {
            if state.r_v[x as usize] == kk {
                state.r_pc += 2;
            }
        }
        (0x4, _, _, _) => {
            if state.r_v[x as usize] != kk {
                state.r_pc += 2;
            }
        }
        (0x5, _, _, _) => {
            if state.r_v[x as usize] == state.r_v[y as usize] {
                state.r_pc += 2;
            }
        }
        (0x6, _, _, _) => state.r_v[x as usize] = kk,
        (0x7, _, _, _) => state.r_v[x as usize] = state.r_v[x as usize].wrapping_add(kk),
        (0x8, _, _, 0x0) => state.r_v[x as usize] = state.r_v[y as usize],
        (0x8, _, _, 0x1) => {
            state.r_v[x as usize] |= state.r_v[y as usize];
            if quirks.vf_reset {
                state.r_v[0xF] = 0;
            }
        }
        (0x8, _, _, 0x2) => {
            state.r_v[x as usize] &= state.r_v[y as usize];
            if quirks.vf_reset {
                state.r_v[0xF] = 0;
            }
        }
        (0x8, _, _, 0x3) => {
            state.r_v[x as usize] ^= state.r_v[y as usize];
            if quirks.vf_reset {
                state.r_v[0xF] = 0;
            }
        }
        (0x8, _, _, 0x4) => {
            let val: u16 = u16::from(state.r_v[x as usize]) + u16::from(state.r_v[y as usize]);
            if val > u16::from(u8::MAX) {
                state.r_v[x as usize] = val as u8;
                state.r_v[0xF] = 1;
            } else {
                state.r_v[x as usize] = val as u8;
                state.r_v[0xF] = 0;
            }
        }
        (0x8, _, _, 0x5) => {
            let flag = state.r_v[x as usize] >= state.r_v[y as usize];
            state.r_v[x as usize] = state.r_v[x as usize].wrapping_sub(state.r_v[y as usize]);
            state.r_v[0xF] = u8::from(flag);
        }
        (0x8, _, _, 0x6) => {
            if !quirks.shifting {
                state.r_v[x as usize] = state.r_v[y as usize];
            }
            let flag = (state.r_v[x as usize] & 0b0000_0001) == 1;
            state.r_v[x as usize] /= 2;
            state.r_v[0xF] = u8::from(flag);
        }
        (0x8, _, _, 0x7) => {
            let flag = state.r_v[x as usize] <= state.r_v[y as usize];
            state.r_v[x as usize] = state.r_v[y as usize].wrapping_sub(state.r_v[x as usize]);
            state.r_v[0xF] = u8::from(flag);
        }
        (0x8, _, _, 0xE) => {
            if !quirks.shifting {
                state.r_v[x as usize] = state.r_v[y as usize];
            }
            let flag = ((state.r_v[x as usize] & 0b1000_0000) >> 7) == 1;
            state.r_v[x as usize] = state.r_v[x as usize].wrapping_mul(2);
            state.r_v[0xF] = u8::from(flag);
        }
        (0x9, _, _, _) => {
            if state.r_v[x as usize] != state.r_v[y as usize] {
                state.r_pc += 2;
            }
        }
        (0xA, _, _, _) => state.r_i = nnn,
        (0xB, _, _, _) => {
            if quirks.jumping {
                state.r_pc = nnn + u16::from(state.r_v[x as usize]);
            } else {
                state.r_pc = nnn + u16::from(state.r_v[0]);
            }
        }
        (0xC, _, _, _) => {
            #[allow(clippy::cast_sign_loss)]
            let rng = rand::random_range(0..256) as u8;
            let result = rng & kk;
            state.r_v[x as usize] = result;
        }
        (0xD, _, _, _) => {
            let bytes = read_n_bytes(&state.mem, state.mem.len(), state.r_i as usize, n as usize);
            if quirks.clipping {
                gpu::clipping_draw(state, x, y, &bytes, n);
            } else {
                gpu::draw(state, x, y, &bytes, n);
            }

            if quirks.display_wait {
                state.vblank_waiting = true;
            }
        }
        (0xE, _, _, 0xE) => {
            let key_index = state.r_v[x as usize];
            if state.input[key_index as usize] {
                state.r_pc += 2;
            }
        }
        (0xE, _, _, 0x1) => {
            let key_index = state.r_v[x as usize];
            if !state.input[key_index as usize] {
                state.r_pc += 2;
            }
        }
        (0xF, _, 0x0, 0x7) => state.r_v[x as usize] = state.r_dt,
        (0xF, _, 0x0, 0xA) => {
            let mut is_key_pressed = false;
            for i in 0..state.input.len() {
                if state.input[i] {
                    state.r_v[x as usize] = i as u8;
                    is_key_pressed = true;
                    break;
                }
            }
            if !is_key_pressed {
                state.r_pc -= 2; // set the pc back to this instruction, otherwise can't handle input
            }
        }
        (0xF, _, 0x1, 0x5) => state.r_dt = state.r_v[x as usize],
        (0xF, _, 0x1, 0x8) => state.r_st = state.r_v[x as usize],
        (0xF, _, 0x1, 0xE) => state.r_i += u16::from(state.r_v[x as usize]),
        (0xF, _, 0x2, 0x9) => state.r_i = u16::from(gpu::get_builtin_sprite_addr(x)),
        (0xF, _, 0x3, 0x3) => {
            let mut decimal = state.r_v[x as usize];
            let mut i = 3;
            loop {
                i -= 1;
                state.mem[(state.r_i + i) as usize] = decimal % 10;
                decimal /= 10;

                if i == 0 {
                    break;
                }
            }
        }
        (0xF, _, _, 0x5) => {
            let mut i = 0;
            while i <= x {
                match y {
                    0x5 => state.mem[(state.r_i + u16::from(i)) as usize] = state.r_v[i as usize],
                    0x6 => state.r_v[i as usize] = state.mem[(state.r_i + u16::from(i)) as usize],
                    _ => panic!("Unmatched OPCODE 0xFx{y}5"),
                }

                i += 1;
            }
            if quirks.memory {
                state.r_i += u16::from(i);
            }
        }
        _ => {}
    }
}
