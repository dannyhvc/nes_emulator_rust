use components::{
    dh_bus::bus::BUS, dh_cpu::cpu::CPU, RESET_VECTOR_HIGH_BYTE,
    RESET_VECTOR_LOW_BYTE,
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
}

impl Default for DebuggerState {
    fn default() -> Self {
        let mut this = Self {
            cpu: CPU::new(),
            bus: BUS::new(),
            context: UiContext::ShowRAM,
        };
        CPU::reset(&mut this.cpu, &this.bus);
        mini_program(&mut this);

        this
    }
}

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
fn example_1() -> Vec<Vec<u16>> {
    vec![
        vec![0x8000, 0xA2, 0x00],       // LDX #$00
        vec![0x8002, 0x8E, 0x00, 0x00], // STX $0000
        vec![0x8005, 0xE8],             // INX
        vec![0x8006, 0x8E, 0x00, 0x00], // STX $0000
        vec![0x8009, 0x4C, 0x05, 0x80], // JMP $8005
    ]
}

fn mini_program(DebuggerState { cpu, bus, .. }: &mut DebuggerState) {
    const START: u16 = 0x8000;
    const STOP: u16 = 0x800B;

    let ttape = example_1();

    // Set reset vector (where the program will start exectuing from)
    bus.write(RESET_VECTOR_LOW_BYTE, 0x00);
    bus.write(RESET_VECTOR_HIGH_BYTE, 0x80);

    // is there a better way to do this?
    // NOTE this will add count of WRITE for all program instruction addresses.
    bus.load_instruction_mem(ttape.clone());

    // NOTE this will add count of READ for all locations between START and STOP
    let mut disasm: Vec<_> =
        CPU::disassemble(bus, START, STOP).into_iter().collect();
    disasm.sort();

    dbg!(disasm);
    cpu.reset(&bus);
}
