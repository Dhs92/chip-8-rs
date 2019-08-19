#![allow(dead_code)]
mod cpu;
mod drivers;

use std::time::Duration;
use std::thread::sleep;
use glium::{
    glutin::{dpi, ContextBuilder, Event, EventsLoop, WindowBuilder, WindowEvent},
    Display, Surface, backend,
};

/*
For display: store the sprite in an array of u8. 1b == 1px, write position + offset
*/
// TODO restructure files
// TODO read create_window args from JSON file with serde

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut events_loop = EventsLoop::new();
    let display = create_window(640.0, 320.0, &events_loop, "chip-8")?;

    let mut closed = false;
    while !closed {
        let begin = std::time::Instant::now();
        let mut target = display.draw();

        target.clear_color(0.0, 0.0, 0.0, 0.0);

        target.finish()?;

        events_loop.poll_events(|ev| if let Event::WindowEvent {event: WindowEvent::CloseRequested, .. }  = ev {
            closed = true;
        });

        let end = std::time::Instant::now();

        let delta = end - begin;

        if delta <= Duration::from_millis(1/60) {
            sleep(Duration::from_millis(1/60).checked_sub(delta).unwrap());
        }

    }

    Ok(())
}

fn create_window(x_dim: f64, y_dim: f64, events_loop: &EventsLoop, title: &str) -> Result<Display, backend::glutin::DisplayCreationError> {
    let wb = WindowBuilder::new()
        .with_dimensions(dpi::LogicalSize::new(x_dim, y_dim))
        .with_title(title);

    let cb = ContextBuilder::new();

    match Display::new(wb, cb, &events_loop) {
        Ok(display) => { Ok(display) }
        Err(e) => { Err(e) }
    }
}