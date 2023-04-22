extern crate custom_error;
extern crate piston_window;

mod components;
mod util;

use components::{bus::Bus, dh6502_cpu::M6502};
use piston_window::*;

pub fn main() {
    let mut cpu: M6502 = M6502::new();
    let mut bus: Bus = Bus::new();

    let clock_eight_times = |cpu: &mut M6502, bus: &mut Bus| {
        for _ in 0..8 {
            M6502::clock(cpu, bus);
        }
    };

    M6502::reset(&mut cpu, &bus);

    clock_eight_times(&mut cpu, &mut bus);
    M6502::clock(&mut cpu, &mut bus);

    println!("{:?}", cpu);

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle(
                [1.0, 0.0, 0.0, 1.0], // red in RGBA
                [0.0, 0.0, 1.0, 1.0], // we can draw single pixels using XYWL
                context.transform,
                graphics,
            );
        });
    }
}
