use std::io::{self, Read};

use super::Chip8State;

pub fn print_debug(state: &Chip8State, current_instruction: u16) {
    print!("\x1b[H\x1b[J");
    println!("----  DEBUG ----");
    println!("PC: {:04X}", state.r_pc - 2); // -2 because of where i put this log in the main loop
    println!("SP: {}", state.r_sp);
    println!("I: {:04X}", state.r_i);
    println!("DT: {}", state.r_dt);
    println!("ST: {}", state.r_st);
    for x in 0..state.r_v.len() {
        if state.r_v[x] > 0 {
            println!("V{}: {}", x, state.r_v[x]);
        }
    }
    for x in 0..state.input.len() {
        if state.input[x] == true {
            println!("Pressed: {}", x);
        }
    }
    println!("Current Instruction: {:04X}", current_instruction);
    println!("----------------");
    // println!("Press Enter to continue...");
    // let _ = io::stdin().read(&mut [0u8]).unwrap();
}
