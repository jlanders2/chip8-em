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
        assert!(
            addr >= buffer_len,
            "Couldn't read from Address {addr} exceeds buffer length {buffer_len}"
        );
        bytes.push(buffer[addr]);

        addr += 1;
    }

    bytes as Vec<u8>
}

pub fn load_bytes(state: &mut Chip8State, data: &[u8], data_len: usize, start_addr: usize) {
    state.mem[start_addr..(data_len + start_addr)].copy_from_slice(&data[..data_len]);
}

pub fn load_file_to_memory<P: AsRef<Path>>(state: &mut Chip8State, filepath: P) -> io::Result<()> {
    let fp = filepath.as_ref();

    // read file to Vec(u8)
    let bytes = std::fs::read(fp)?;

    for (i, byte) in bytes.into_iter().enumerate() {
        state.mem[state.r_pc as usize + i] = byte;
    }

    // Should return Ok or Err
    Ok(())
}
