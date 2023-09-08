mod components;
mod util;

fn main() {}

#[test]
fn test_clock() {
    use components::{bus::Bus, dh6502_cpu::M6502};
    let mut cpu: M6502 = M6502::new();
    let mut bus: Bus = Bus::new();
    M6502::reset(&mut cpu, &bus);
    for _ in 0..8 {
        M6502::clock(&mut cpu, &mut bus);
    }
    assert!(cpu.cycles == 0);
}
