#![allow(dead_code)]
mod cpu;
use crate::cpu::*;
use ggez;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::sync::Mutex;

struct MainState {}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let m = Self {};
        Ok(m)
    }
}

impl ggez::event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        ggez::graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let mut image =
            ggez::graphics::Image::from_rgba8(ctx, 64, 32, &CPU.lock().unwrap().display.vram)?;
        image.set_filter(ggez::graphics::FilterMode::Nearest);
        // image.encode(ctx, ggez::graphics::ImageFormat::Png, "/test.png")?;
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

lazy_static::lazy_static!(
static ref CPU: Mutex<Cpu> = Mutex::new(Cpu {
    pc: 0x200,
    i: 0,
    memory: [0; 4096],
    v: [0; 16],
    keypad: Keypad {},
    display: Display {
        vram: [0x00; 32 * 64 * 4],
    },
    stack: [0; 16],
    sp: 0,
    dt: 0,
}
);
);
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
    let _ = CPU.lock().unwrap().load_file("invaders.c8");
    let _ = CPU.lock().unwrap().display.set_pixel(0, 0);
    let _ = CPU.lock().unwrap().display.set_pixel(63, 0);
    let _ = CPU.lock().unwrap().display.set_pixel(63, 2);
    let _ = CPU.lock().unwrap().display.set_pixel(63, 4);
    let _ = CPU.lock().unwrap().display.set_pixel(63, 6);
    let _ = CPU.lock().unwrap().display.set_pixel(63, 31);

    let cb = ggez::ContextBuilder::new("chip-8", "Yuuki").window_mode(wm);

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    ggez::event::run(ctx, event_loop, state)
}
