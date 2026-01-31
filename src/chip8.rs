mod cpu;
mod gpu;
mod input;
mod memory;
mod renderer;

use rodio::source::{SineWave, Source};

static MEMORY_LIMIT: u16 = 4096;
static STACK_LIMIT: u8 = 16;
static VARIABLE_REGISTER_COUNT: u8 = 16;
static TIMER_TICK_RATE: f32 = 60.0;
static DESIRED_FPS: f32 = 60.0;
static CYCLES_PER_FRAME: f32 = 10.0;
static INSTRUCTION_BYTE_SIZE: usize = 2;

#[derive(Debug, Clone)]
struct Chip8State {
    // Flags
    eti_600_flag: bool,
    vblank_waiting: bool,

    // Memory
    mem: [u8; MEMORY_LIMIT as usize],
    stack: [u16; STACK_LIMIT as usize],

    // Registers
    r_v: [u8; VARIABLE_REGISTER_COUNT as usize], // General Purpose
    r_i: u16,                                    // Memory Addresses
    r_dt: u8,                                    // Delay Timer
    r_st: u8,                                    // Sound Timer
    r_pc: u16,                                   // Program Counter
    r_sp: u8,                                    // Stack Pointer

    // Display
    display: [[bool; gpu::CHIP8_DISPLAY_WIDTH as usize]; gpu::CHIP8_DISPLAY_HEIGHT as usize],

    // Input
    input: [bool; 16],
}

#[derive(Debug, Clone, Copy)]
#[allow(clippy::struct_excessive_bools)]
pub struct Chip8Quirks {
    pub vf_reset: bool,
    pub memory: bool,
    pub display_wait: bool, // TODO: Looks to be working, but not in quirks test file
    pub clipping: bool,     // TODO: Looks to be working, but not in quirks test file
    pub shifting: bool,
    pub jumping: bool,
}

pub fn run<S: AsRef<str>>(chip8_executable_filepath: S, quirks: Chip8Quirks) {
    let mut state = Chip8State {
        eti_600_flag: false,
        vblank_waiting: false,
        mem: [0; 4096],
        stack: [0; 16],
        r_v: [0; 16],
        r_i: 0,
        r_dt: 2,
        r_st: 0,
        r_pc: 0,
        r_sp: 0,
        display: [[false; 64]; 32],
        input: [false; 16],
    };

    if state.eti_600_flag {
        state.r_pc = 0x600;
    } else {
        state.r_pc = 0x200;
    }

    // Load Program
    let _ = memory::load_file_to_memory(&mut state, chip8_executable_filepath.as_ref());

    // Run Program
    start(state, quirks);
}

fn start(mut state: Chip8State, quirks: Chip8Quirks) {
    // TODO rip out as much RL stuff from here and put into renderer
    // Init Rendering Pipeline
    let (mut rl, thread) = raylib::init()
        .size(
            i32::from(renderer::DISPLAY_WIDTH),
            i32::from(renderer::DISPLAY_HEIGHT),
        )
        .title("Chip8 Emu")
        .build();
    #[allow(clippy::cast_sign_loss)]
    rl.set_target_fps(DESIRED_FPS as u32); // Should see if i can get the users hz
    rl.set_trace_log(raylib::ffi::TraceLogLevel::LOG_NONE);

    // initialize timer
    let mut timer_accumulator: f32 = 0.0f32;
    let timer_increment: f32 = TIMER_TICK_RATE / DESIRED_FPS;

    // initialize builtin sprites
    gpu::load_builtin_sprites(&mut state);

    // initialize sound system look into struct and impl for functions
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let sink = rodio::Sink::connect_new(stream_handle.mixer());

    let source = SineWave::new(440.0)
        .amplify(0.2) // Volume (0.0 to 1.0)
        .repeat_infinite();

    sink.append(source);
    sink.play();

    while !rl.window_should_close() {
        if state.r_pc > 4095 {
            break;
        }

        let sound_volume = sink.volume();
        if state.r_st > 0 {
            if sound_volume < 1.0f32 {
                sink.set_volume(0.2f32);
            }
        } else {
            sink.set_volume(0.0f32);
        }

        input::handle_input(&mut state, &mut rl);

        state.vblank_waiting = false;
        for _ in 0..CYCLES_PER_FRAME as i32 {
            let instruction_bytes = memory::read_n_bytes(
                &state.mem,
                state.mem.len(),
                state.r_pc as usize,
                INSTRUCTION_BYTE_SIZE,
            );
            let instruction: u16 =
                (u16::from(instruction_bytes[0]) << 8) | u16::from(instruction_bytes[1]);
            state.r_pc += 2;

            cpu::execute_instruction(&mut state, instruction, quirks);

            if state.vblank_waiting {
                break;
            }
        }

        // move to timers.rs
        // timers::tick();
        if (state.r_dt | state.r_st) > 0 {
            timer_accumulator += timer_increment;
            while timer_accumulator >= 1.0f32 {
                if state.r_dt > 0 {
                    state.r_dt -= 1;
                }
                if state.r_st > 0 {
                    state.r_st -= 1;
                }
                timer_accumulator -= 1.0f32;
            }
        } else {
            timer_accumulator = 0.0f32;
        }

        renderer::render(&state, &mut rl, &thread);
    }
}
