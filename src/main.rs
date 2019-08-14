#![allow(dead_code)]
mod cpu;
mod drivers;
use crate::cpu::*;
use crate::drivers::Display;
use ggez;
/*
For display: store the sprite in an array of u8. 1b == 1px, write position + offset
*/
// TODO restructure files

#[allow(non_snake_case)]
struct MainState {
    CPU: Cpu,
    cpu_state: bool,
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let m = Self {
            CPU: Cpu {
                pc: 0x200,
                i: 0,
                memory: [0; 4096],
                v: [0; 16],
                keypad: Keypad {},
                display: Display {
                    vram: [0x00; SCREEN_HEIGHT * SCREEN_WIDTH * 4],
                },
                stack: [0; 16],
                sp: 0,
                dt: 0,
            },
            cpu_state: false,
        };

        Ok(m)
    }
}

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        if !self.cpu_state {
            let args: Vec<String> = std::env::args().collect();

            let _bytes_read = self
                .CPU
                .load_file(std::path::Path::new(&args[1]))
                .unwrap_or_else(|_| panic!("File not found: {}", args[1]));

            log::debug!("Bytes Read: {}", _bytes_read);

            self.cpu_state = true;
        }

        self.CPU.execute_opcode().unwrap();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut image = ggez::graphics::Image::from_rgba8(ctx, 64, 32, &self.CPU.display.vram)?;

        image.set_filter(ggez::graphics::FilterMode::Nearest);

        ggez::graphics::draw(
            ctx,
            &image,
            ggez::graphics::DrawParam::new().scale(ggez::mint::Vector2 {
                x: 10.0f32,
                y: 10.0f32,
            }),
        )?;

        ggez::graphics::present(ctx)?;

        Ok(())
    }
}

fn main() -> ggez::GameResult {
    let wm = ggez::conf::WindowMode {
        width: 640.0,
        height: 320.0,
        maximized: false,
        fullscreen_type: ggez::conf::FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: true,
    };

    let logger_handle = flexi_logger::Logger::with_str("off, cpu=debug")
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with: {}", e));
    logger_handle.validate_logs(&[("OFF", "cpu", "debug")]);

    let cb = ggez::ContextBuilder::new("chip-8", "Yuuki").window_mode(wm);

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    ggez::event::run(ctx, event_loop, state)
}
