use schip8::Chip8;
use anyhow::{Context, Result};
use macroquad::prelude::*;

#[macroquad::main("CHIP-8")]
async fn main() -> Result<()> {
    let mut chip = Chip8::default();
    let scale: f32 = 20.0;
    request_new_screen_size(64.0 * scale, 32.0 * scale);

    let file = load_file("roms/ibm.ch8").await.context("Reading rom file")?;
    chip.load_rom(&file).context("Loading rom file")?;

    loop {
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
        
        next_frame().await
    }
}
