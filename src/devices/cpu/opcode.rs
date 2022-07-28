use crate::devices::bus::Bus;
use super::processor::Cpu;

pub trait Opcode {
    fn ADC(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn BCS(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn BNE(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn BVS(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn CLV(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn DEC(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn INC(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn JSR(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn LSR(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn PHP(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn ROR(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn SEC(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn STX(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn TSX(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn AND(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn BEQ(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn BPL(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn CLC(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn CMP(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn DEX(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn INX(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn LDA(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn NOP(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn PLA(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn RTI(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn SED(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn STY(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn TXA(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn ASL(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn BIT(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn BRK(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn CLD(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn CPX(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn DEY(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn INY(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn LDX(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn ORA(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn PLP(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn RTS(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn SEI(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn TAX(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn TXS(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn BCC(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn BMI(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn BVC(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn CLI(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn CPY(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn EOR(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn JMP(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn LDY(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn PHA(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn ROL(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn SBC(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn STA(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn TAY(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}
    fn TYA(cpu_6502: &mut Cpu, bus: &mut Bus) -> u8 {todo!()}

    // ERROR CODE
    fn XXX(cpu_6502: &mut Cpu) -> u8 {
        0u8
    }
}
