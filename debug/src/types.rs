use components::{
    dh_bus::BUS, dh_cpu::CPU, RESET_VECTOR_HIGH_BYTE, RESET_VECTOR_LOW_BYTE,
};
use iced::keyboard::Key;

#[derive(Debug, Clone, Default)]
pub struct DebuggerApp;

#[derive(Debug, Clone, PartialEq)]
pub enum DebuggerMsg {
    Start,
    KeyPressed(Key),
    RefreshContext(UiContext),
    CpuActions(CpuActions),
    End,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum CpuActions {
    Reset,
    Clock,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum UiContext {
    ShowRAM,
    ShowCPU,
    ShowPPU,
    ShowAPU,
}

#[derive(Clone, Hash)]
pub struct DebuggerState {
    pub bus: BUS,
    pub cpu: CPU,
    pub context: UiContext,
    pub disasm: Vec<(u16, String)>,
    pub disasm_idx: usize,
}

impl Default for DebuggerState {
    fn default() -> Self {
        let mut cpu = CPU::new();
        let mut bus = BUS::new();
        let disasm = mini_program(&mut cpu, &mut bus);

        Self {
            cpu,
            bus,
            context: UiContext::ShowRAM,
            disasm,
            disasm_idx: 0usize,
        }
    }
}

#[deprecated]
fn example_0() -> Vec<Vec<u16>> {
    vec![
        vec![0x8000, 0xA2, 0x0A],       // A2 0A       LDX #10
        vec![0x8002, 0x8E, 0x00, 0x00], // 8E 00 00    STX $0000
        vec![0x8005, 0xA2, 0x03],       // A2 03       LDX #3
        vec![0x8007, 0x8E, 0x01, 0x00], // 8E 01 00    STX $0001
        vec![0x800A, 0xAC, 0x00, 0x00], // AC 00 00    LDY $0000
        vec![0x800D, 0xA9, 0x00],       // A9 00       LDA #0
        vec![0x800F, 0x18],             // 18          CLC
        vec![0x8010, 0x6D, 0x01, 0x00], // 6D 01 00    ADC $0001
        vec![0x8040, 0x88],             // 88          DEY
        vec![0x8050, 0xD0, 0xFA], // D0 FA       BNE loop -- FA is the relative offset for the branch
        vec![0x8070, 0x8D, 0x02, 0x00], // 8D 02 00    STA $0002
        vec![0x80A0, 0xEA],       // EA          NOP
        vec![0x80B0, 0xEA],       // EA          NOP
        vec![0x80C0, 0xEA],       // EA          NOP
    ]
}

/// Looping example
#[deprecated]
fn example_1() -> Vec<Vec<u16>> {
    vec![
        vec![0x8000, 0xA2, 0x00],       // LDX #$00
        vec![0x8002, 0x8E, 0x00, 0x00], // STX $0000
        vec![0x8005, 0xE8],             // INX
        vec![0x8006, 0x8E, 0x00, 0x00], // STX $0000
        vec![0x8009, 0x4C, 0x05, 0x80], // JMP $8005
    ]
}

// highly coupled specific function designed to immitate a simple incrementor in a loop
fn program_1(cpu: &mut CPU, bus: &mut BUS, offset: &mut usize) {
    const PROGRAM_STR: &str = "A2 00 8E 00 00 E8 8E 00 00 4C 05 80";
    log::info!("{PROGRAM_STR}");
    _ = bus.load_program(PROGRAM_STR, offset);

    bus.write(RESET_VECTOR_LOW_BYTE, 0x00);
    bus.write(RESET_VECTOR_HIGH_BYTE, 0x80);
    cpu.reset(&bus);
}

fn mini_program(cpu: &mut CPU, bus: &mut BUS) -> Vec<(u16, String)> {
    const START_DEBUG: u16 = 0x8000;
    const STOP_DEBUG: u16 = 0x800B;

    let mut offset: usize = START_DEBUG.into();
    program_1(cpu, bus, &mut offset);

    // NOTE this will add count of READ for all locations between START and STOP
    let mut disasm: Vec<_> = CPU::disassemble(bus, START_DEBUG, STOP_DEBUG)
        .into_iter()
        .collect();
    disasm.sort();
    disasm
}
