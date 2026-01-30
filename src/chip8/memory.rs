use super::Chip8State;
use std::{io, path::Path};

pub fn read_n_bytes(
    buffer: &[u8],
    buffer_len: usize,
    start_addr: usize,
    n_bytes: usize,
) -> Vec<u8> {
    let mut addr = start_addr;
    let mut bytes = Vec::new();
    while addr != start_addr + n_bytes {
        if addr >= buffer_len {
            panic!(
                "Couldn't read from Address {} exceeds buffer length {}",
                addr, buffer_len
            ) // nice error handling
        }
        bytes.push(buffer[addr]);

        addr += 1;
    }

    return bytes as Vec<u8>;
}

pub fn load_bytes(state: &mut Chip8State, data: &[u8], data_len: usize, start_addr: usize) {
    for i in 0..data_len {
        state.mem[start_addr + i] = data[i];
    }
}

pub fn load_file_to_memory<P: AsRef<Path>>(state: &mut Chip8State, filepath: P) -> io::Result<()> {
    let fp = filepath.as_ref();

    // read file to Vec(u8)
    let program = std::fs::read(fp)?;

    for i in 0..program.len() {
        state.mem[state.r_pc as usize + i] = program[i];
    }

    // Should return Ok or Err
    Ok(())
}
