use std::collections::HashMap;
use std::time::Duration;

use anyhow::{Context, Result};
use macroquad::prelude::*;
use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};

use schip8::Chip8;

fn process_input(keybindings: &HashMap<KeyCode, u8>, chip: &mut Chip8) {
    let mut keys_pressed: [bool; 16] = [false; 16];

    for (input, chip8_input) in keybindings.iter() {
        if is_key_down(*input) {
            keys_pressed[*chip8_input as usize] = true;
        }
    }

    chip.set_input(keys_pressed);
}

fn play_tone() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let source = SineWave::new(600.0)
        .take_duration(Duration::from_secs_f32(1.0 / 60.0))
        .amplify(0.20);

    sink.append(source);
    sink.sleep_until_end();
}

#[macroquad::main("CHIP-8")]
async fn main() -> Result<()> {
    let keybindings: HashMap<KeyCode, u8> = HashMap::from([
        (KeyCode::Key1, 0x1),
        (KeyCode::Key2, 0x2),
        (KeyCode::Key3, 0x3),
        (KeyCode::Key4, 0xC),
        (KeyCode::Q, 0x4),
        (KeyCode::W, 0x5),
        (KeyCode::E, 0x6),
        (KeyCode::R, 0xD),
        (KeyCode::A, 0x7),
        (KeyCode::S, 0x8),
        (KeyCode::D, 0x9),
        (KeyCode::F, 0xE),
        (KeyCode::Z, 0xA),
        (KeyCode::X, 0x0),
        (KeyCode::C, 0xB),
        (KeyCode::V, 0xF),
    ]);

    let mut chip = Chip8::default();
    let scale: f32 = 20.0;
    request_new_screen_size(
        chip.screen.width as f32 * scale,
        chip.screen.height as f32 * scale,
    );

    let file = load_file("roms/TETRIS").await.context("Reading rom file")?;
    chip.load_rom(&file).context("Loading rom file")?;

    loop {
        process_input(&keybindings, &mut chip);

        chip.tick().context("Interpreter tick")?;

        clear_background(BLACK);
        for y in 0..chip.screen.height {
            for x in 0..chip.screen.width {
                if chip.screen.get_pixel(x, y) {
                    let pixel_x = x as f32;
                    let pixel_y = y as f32;
                    draw_rectangle(pixel_x * scale, pixel_y * scale, scale, scale, WHITE);
                }
            }
        }

        if is_key_pressed(KeyCode::Space) {
            chip.reset();
        }

        if chip.should_play_sound() {
            play_tone();
        }
        next_frame().await
    }
}
