use crate::components::dh_cpu::cpu::CPU;
use crate::components::types::{opcodes::M6502Opcodes, CpuFlags};
use crate::components::M6502AddrModes;
use crate::components::{
    dh_bus::bus::BUS, HIGH_BYTE, LOOKUP_TABLE, LOW_BYTE, TOP_BIT_THRESH,
};

impl M6502Opcodes for CPU {
    /// Perform an addition with carry of the value fetched from the memory pointed to by the program
    /// counter to the accumulator register of the MOS 6502 CPU.
    ///
    /// # Arguments
    ///
    /// * `cpu` - A mutable reference to the [`M6502`] CPU.
    /// * `bus` - A mutable reference to the [`Bus`] connected to the CPU.
    ///
    /// # Return value
    ///
    /// The number of clock cycles taken to execute this instruction, which is always 1.
    ///
    /// # Flags affected
    ///
    /// This instruction may affect the following flags: C, Z, V, N.
    ///
    /// # Details
    ///
    /// This instruction adds the fetched value and the carry flag to the accumulator in 16-bit domain,
    /// setting the carry flag if the result exceeds 255. The result is then truncated to 8 bits and stored
    /// in the accumulator. The zero flag is set if the result is zero, the negative flag is set if the most
    /// significant bit of the result is 1, and the signed overflow flag is set based on a complex condition
    /// involving the previous value of the accumulator, the fetched value, and the new value of the accumulator.
    /// See the implementation for more details.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use M6502::{M6502, Bus};
    ///
    /// let mut cpu = M6502::new();
    /// let mut bus = Bus::new();
    ///
    /// cpu.acc = 0x12;
    /// bus.write(0x1234, 0x34);
    /// cpu.pc = 0x1234;
    ///
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::C), false);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::Z), false);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::V), false);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::N), false);
    ///
    /// let cycles = M6502::instructions::adc(&mut cpu, &mut bus);
    ///
    /// assert_eq!(cycles, 1);
    /// assert_eq!(cpu.acc, 0x46);
    /// assert_eq!(cpu.pc, 0x1235);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::C), false);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::Z), false);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::V), false);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::N), false);
    /// ```
    fn ADC(&mut self, bus: &mut BUS) -> u8 {
        // Grab the data that we are adding to the accumulator
        // Add is performed in 16-bit domain for emulation to capture any
        // carry bit, which will exist in bit 8 of the 16-bit word
        self.temp =
            (self.a + self.fetch(bus) + self.get_flag(CpuFlags::C)).into();

        // The carry flag out exists in the high byte bit 0
        self.set_flag(CpuFlags::C, self.temp > 255);

        // The Zero flag is set if the result is 0
        self.set_flag(CpuFlags::Z, (self.temp & LOW_BYTE) == 0);

        // The signed Overflow flag is set based on all that up there! :D
        self.set_flag(
            CpuFlags::V,
            !(self.a as u16 ^ self.fetched as u16)
                & (self.a as u16 ^ self.temp)
                & 0x0080
                != 0,
        );

        // The negative flag is set to the most significant bit of the result
        self.set_flag(CpuFlags::N, (self.temp & TOP_BIT_THRESH) != 0);

        // Load the result into the accumulator (it's 8-bit dont forget!)
        self.a = ((self.temp as u16) & LOW_BYTE) as u8;

        // This instruction has the potential to require an additional clock cycle
        1u8
    }

    /// Perform a bitwise AND operation between the accumulator register of the MOS 6502 CPU and the value
    /// fetched from the memory pointed to by the program counter.
    ///
    /// # Arguments
    ///
    /// * `cpu` - A mutable reference to the [`M6502`] CPU.
    /// * `bus` - A mutable reference to the [`Bus`] connected to the CPU.
    ///
    /// # Return value
    ///
    /// The number of clock cycles taken to execute this instruction, which is always 1.
    ///
    /// # Flags affected
    ///
    /// This instruction may affect the following flags: Z, N.
    ///
    /// # Details
    ///
    /// This instruction performs a bitwise AND operation between the accumulator and the fetched value, storing
    /// the result in the accumulator. The zero flag is set if the result is zero, and the negative flag is set
    /// if the most significant bit of the result is 1. See the implementation for more details.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use M6502::{M6502, Bus};
    ///
    /// let mut cpu = M6502::new();
    /// let mut bus = Bus::new();
    ///
    /// cpu.acc = 0x12;
    /// bus.write(0x1234, 0x34);
    /// cpu.pc = 0x1234;
    ///
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::Z), false);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::N), false);
    ///
    /// let cycles = M6502::instructions::AND(&mut cpu, &mut bus);
    ///
    /// assert_eq!(cycles, 1);
    /// assert_eq!(cpu.acc, 0x12 & 0x34);
    /// assert_eq!(cpu.pc, 0x1235);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::Z), false);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::N), true);
    /// ```
    fn AND(&mut self, bus: &mut BUS) -> u8 {
        self.a &= self.fetch(bus);
        self.set_flag(CpuFlags::Z, self.a == 0x00);
        self.set_flag(CpuFlags::N, (self.a & TOP_BIT_THRESH as u8) != 0);
        1u8
    }

    /// Perform an arithmetic shift left operation on the value fetched from memory or the accumulator
    /// register of the MOS 6502 CPU.
    ///
    /// # Arguments
    ///
    /// * `cpu` - A mutable reference to the [`M6502`] CPU.
    /// * `bus` - A mutable reference to the [`Bus`]connected to the CPU.
    ///
    /// # Return value
    ///
    /// The number of clock cycles taken to execute this instruction, which is always 0.
    ///
    /// # Flags affected
    ///
    /// This instruction may affect the following flags: C, Z, N.
    ///
    /// # Details
    ///
    /// This instruction shifts the bits of the fetched value or the accumulator one position to the left,
    /// storing the result in the temporary register of the CPU. The carry flag is set to the value of the
    /// bit that was shifted out of the most significant bit position, the zero flag is set if the result
    /// is zero, and the negative flag is set if the most significant bit of the result is 1. The shifted
    /// result is stored back in the accumulator or memory, depending on the addressing mode used by the
    /// instruction. See the implementation for more details.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use M6502::{M6502, Bus};
    ///
    /// let mut cpu = M6502::new();
    /// let mut bus = Bus::new();
    ///
    /// bus.write(0x1234, 0b10010010);
    /// cpu.pc = 0x1234;
    ///
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::C), false);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::Z), false);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::N), false);
    ///
    /// let cycles = M6502::instructions::asl(&mut cpu, &mut bus);
    ///
    /// assert_eq!(cycles, 0);
    /// assert_eq!(bus.read(0x1234), 0b00100100);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::C), true);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::Z), false);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::N), false);
    ///
    /// cpu.acc = 0b10010010;
    ///
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::C), false);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::Z), false);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::N), false);
    ///
    /// let cycles = M6502::instructions::asl(&mut cpu, &mut bus);
    ///
    /// assert_eq!(cycles, 0);
    /// assert_eq!(cpu.acc, 0b00100100);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::C), true);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::Z), false);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::N), false);
    /// ```
    #[inline]
    fn ASL(&mut self, bus: &mut BUS) -> u8 {
        self.temp = (self.fetch(bus) << 1).into();
        self.set_flag(CpuFlags::C, (self.temp & HIGH_BYTE) > 0);
        self.set_flag(CpuFlags::Z, (self.temp & LOW_BYTE) == 0);
        self.set_flag(CpuFlags::N, (self.temp & TOP_BIT_THRESH) != 0);
        if LOOKUP_TABLE[self.opcode as usize].addr_mode as usize
            == CPU::IMP as usize
        {
            self.a = (self.temp & LOW_BYTE) as u8;
        } else {
            bus.write(self.abs, (self.temp & LOW_BYTE) as u8);
        }
        0u8
    }

    /// Branch on Carry Clear
    ///
    /// This function implements the "BCC" instruction, which checks if the carry flag is clear. If the carry flag is clear, then
    /// add the relative displacement to the program counter to cause a branch to a new location. The 6502 supports relative
    /// addressing mode, so the value read from memory is the two's complement of a signed byte that represents the relative
    /// displacement to be added to the program counter. If the carry flag is set, then the program counter is incremented
    /// without a branch. This instruction takes two cycles to complete, and an additional cycle if the branch occurs on the same
    /// page. The function returns the number of cycles that the instruction has consumed.
    ///
    /// # Arguments
    ///
    /// * `_` - A mutable reference to the [`Bus`]. This argument is ignored by this function.
    ///
    /// # Returns
    ///
    /// The number of cycles that the instruction has consumed, which is always 0.
    #[inline]
    fn BCC(&mut self, _: &mut BUS) -> u8 {
        if self.get_flag(CpuFlags::C) == 0_u8 {
            self.cycles += 1_u8;
            self.abs = self.pc + self.rel;

            if self.abs & HIGH_BYTE != self.pc & HIGH_BYTE {
                self.cycles += 1_u8;
            }
            self.pc = self.abs;
        }
        0_u8
    }

    /// Branch on carry set
    ///
    /// If the carry flag is set, then add the relative displacement to the program counter to
    /// cause a branch to a new location.
    ///
    /// # Arguments
    ///
    /// * `cpu` - A mutable reference to the MOS 6502 CPU.
    /// * `_` - A mutable reference to the bus connected to the MOS 6502 CPU. This argument is
    /// ignored, as this instruction does not access memory.
    ///
    /// # Return value
    ///
    /// The number of clock cycles taken to execute this instruction, which is always 0x1.
    ///
    /// # Flags affected
    ///
    /// This instruction does not affect any flags.
    ///
    /// # Details
    ///
    /// This instruction adds the relative displacement provided by the instruction operand to the
    /// program counter, causing a branch to a new location if the carry flag is set (i.e. if the
    /// result of the previous arithmetic or bitwise operation resulted in a carry or overflow). If
    /// the branch is taken, an additional cycle is taken if the branch crosses a page boundary. See
    /// the implementation for more details.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use M6502::{M6502, Bus};
    ///
    /// let mut cpu = M6502::new();
    ///
    /// // BCS should branch because C flag is set (carry)
    /// cpu.pc = 0x1234;
    /// cpu.addr_rel = 0x10;
    /// cpu.set_flag(M6502::M6502Flags::C, true);
    ///
    /// let cycles = M6502::instructions::bcs(&mut cpu, &mut Bus::new());
    ///
    /// assert_eq!(cycles, 1);
    /// assert_eq!(cpu.pc, 0x1234 + 0x10);
    ///
    /// // BCS should not branch because C flag is clear (no carry)
    /// cpu.pc = 0x1234;
    /// cpu.addr_rel = 0x10;
    /// cpu.set_flag(M6502::M6502Flags::C, false);
    ///
    /// let cycles = M6502::instructions::bcs(&mut cpu, &mut Bus::new());
    ///
    /// assert_eq!(cycles, 1);
    /// assert_eq!(cpu.pc, 0x1234);
    /// ```
    #[inline]
    fn BCS(&mut self, _: &mut BUS) -> u8 {
        if self.get_flag(CpuFlags::C) == 1_u8 {
            self.cycles += 1_u8;
            self.abs = self.pc + self.rel;

            if self.abs & HIGH_BYTE != self.pc & HIGH_BYTE {
                self.cycles += 1_u8;
            }
            self.pc = self.abs;
        }
        1u8
    }

    /// Branch on equal (zero set)
    ///
    /// If the zero flag is set, then add the relative displacement to the program counter to
    /// cause a branch to a new location.
    ///
    /// # Arguments
    ///
    /// * `cpu` - A mutable reference to the MOS 6502 CPU.
    /// * `_` - A mutable reference to the bus connected to the MOS 6502 CPU. This argument is
    /// ignored, as this instruction does not access memory.
    ///
    /// # Return value
    ///
    /// The number of clock cycles taken to execute this instruction, which is always 0x0.
    ///
    /// # Flags affected
    ///
    /// This instruction does not affect any flags.
    ///
    /// # Details
    ///
    /// This instruction adds the relative displacement provided by the instruction operand to the
    /// program counter, causing a branch to a new location if the zero flag is set (i.e. if the
    /// result of the previous arithmetic or bitwise operation was zero). If the branch is taken, an
    /// additional cycle is taken if the branch crosses a page boundary. See the implementation for
    /// more details.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use M6502::{M6502, Bus};
    ///
    /// let mut cpu = M6502::new();
    ///
    /// // BEQ should branch because Z flag is set (zero)
    /// cpu.pc = 0x1234;
    /// cpu.addr_rel = 0x10;
    /// cpu.set_flag(M6502::M6502Flags::Z, true);
    ///
    /// let cycles = M6502::instructions::beq(&mut cpu, &mut Bus::new());
    ///
    /// assert_eq!(cycles, 0);
    /// assert_eq!(cpu.pc, 0x1234 + 0x10);
    ///
    /// // BEQ should not branch because Z flag is clear (not zero)
    /// cpu.pc = 0x1234;
    /// cpu.addr_rel = 0x10;
    /// cpu.set_flag(M6502::M6502Flags::Z, false);
    ///
    /// let cycles = M6502::instructions::beq(&mut cpu, &mut Bus::new());
    ///
    /// assert_eq!(cycles, 0);
    /// assert_eq!(cpu.pc, 0x1234);
    /// ```
    #[inline]
    fn BEQ(&mut self, _: &mut BUS) -> u8 {
        if self.get_flag(CpuFlags::Z) == 1_u8 {
            self.cycles += 1_u8;
            self.abs = self.pc + self.rel;

            if self.abs & HIGH_BYTE != self.pc & HIGH_BYTE {
                self.cycles += 1_u8;
            }
            self.pc = self.abs;
        }
        0_u8
    }

    /// Bit test
    ///
    /// This instruction performs a bitwise logical AND between the accumulator and the value fetched
    /// from memory. The Z flag is set based on the result of the AND operation (i.e. if the result is
    /// zero), the N flag is set to the value of the most significant bit of the fetched value, and
    /// the V flag is set to the value of the second most significant bit of the fetched value. The
    /// result of the bitwise AND operation is not stored anywhere.
    ///
    /// # Arguments
    ///
    /// * `cpu` - A mutable reference to the MOS 6502 CPU.
    /// * `bus` - A mutable reference to the bus connected to the MOS 6502 CPU.
    ///
    /// # Return value
    ///
    /// The number of clock cycles taken to execute this instruction, which is always 0x0.
    ///
    /// # Flags affected
    ///
    /// This instruction affects the following flags: Z, N, V.
    ///
    /// # Details
    ///
    /// This instruction performs a bitwise AND operation between the value in the accumulator and
    /// the value fetched from memory. The result of the AND operation is stored temporarily in the
    /// CPU's `temp` register as a 16-bit value. The Z flag is set if the result of the AND operation
    /// is zero (i.e. all bits are cleared), otherwise it is cleared. The N flag is set to the value
    /// of the most significant bit of the fetched value, and the V flag is set to the value of the
    /// second most significant bit of the fetched value. The result of the AND operation is not
    /// stored anywhere.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use M6502::{M6502, Bus};
    ///
    /// let mut cpu = M6502::new();
    /// let mut bus = Bus::new();
    ///
    /// // Acc = 0b01010101
    /// // Memory = 0b11110000
    ///
    /// cpu.acc = 0b01010101;
    /// bus.write(0x1234, 0b11110000);
    ///
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::Z), false);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::N), false);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::V), false);
    ///
    /// let cycles = M6502::instructions::bit(&mut cpu, &mut bus);
    ///
    /// assert_eq!(cycles, 0);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::Z), true);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::N), true);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::V), true);
    /// ```
    #[inline]
    fn BIT(&mut self, bus: &mut BUS) -> u8 {
        self.temp = (self.a & self.fetch(bus)) as u16;
        self.set_flag(CpuFlags::Z, (self.temp & LOW_BYTE) == 0x00);
        self.set_flag(CpuFlags::N, (self.fetched & (1 << 7)) != 0);
        self.set_flag(CpuFlags::V, (self.fetched & (1 << 6)) != 0);
        0_u8
    }

    /// Branch on result minus
    ///
    /// This instruction performs a branch operation if the negative flag (N) is set to 1. The branch
    /// operation can jump to a new address within a certain range relative to the current program
    /// counter (PC). If the N flag is set to 1, the program counter is updated to the new address, and
    /// additional clock cycles may be consumed depending on whether the branch crosses a page boundary
    /// or not.
    ///
    /// # Arguments
    ///
    /// * `cpu` - A mutable reference to the MOS 6502 CPU.
    /// * `_` - A mutable reference to the bus connected to the MOS 6502 CPU (unused in this function).
    ///
    /// # Return value
    ///
    /// The number of clock cycles taken to execute this instruction, which is always 0x0.
    ///
    /// # Flags affected
    ///
    /// None of the flags are explicitly affected by this instruction.
    ///
    /// # Details
    ///
    /// This instruction checks the value of the negative flag (N) and performs a branch if it is set
    /// to 1. The branch is performed by calculating the absolute address of the target location by
    /// adding the relative address (`addr_rel`) to the program counter (`pc`). If the branch crosses
    /// a page boundary, an additional clock cycle is consumed. Finally, the program counter (`pc`) is
    /// updated with the new address.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use M6502::{M6502, Bus};
    ///
    /// let mut cpu = M6502::new();
    /// let mut bus = Bus::new();
    ///
    /// cpu.set_flag(M6502::M6502Flags::N, true);
    /// cpu.pc = 0x1234;
    /// cpu.addr_rel = 0x10;
    ///
    /// assert_eq!(cpu.pc, 0x1234);
    ///
    /// let cycles = M6502::instructions::bmi(&mut cpu, &mut bus);
    ///
    /// assert_eq!(cycles, 0);
    /// assert_eq!(cpu.pc, 0x1244); // Branch taken, new address is pc + addr_rel
    /// ```
    #[inline]
    fn BMI(&mut self, _: &mut BUS) -> u8 {
        if self.get_flag(CpuFlags::N) == 1_u8 {
            self.cycles += 1_u8;
            self.abs = self.pc + self.rel;

            if self.abs & HIGH_BYTE != self.pc & HIGH_BYTE {
                self.cycles += 1_u8;
            }
            self.pc = self.abs;
        }
        0_u8
    }

    /// Branch on result not equal
    ///
    /// This instruction performs a branch operation if the zero flag (Z) is set to 0. The branch
    /// operation can jump to a new address within a certain range relative to the current program
    /// counter (PC). If the Z flag is set to 0, the program counter is updated to the new address, and
    /// additional clock cycles may be consumed depending on whether the branch crosses a page boundary
    /// or not.
    ///
    /// # Arguments
    ///
    /// * `cpu` - A mutable reference to the MOS 6502 CPU.
    /// * `_` - A mutable reference to the bus connected to the MOS 6502 CPU (unused in this function).
    ///
    /// # Return value
    ///
    /// The number of clock cycles taken to execute this instruction, which is always 0x0.
    ///
    /// # Flags affected
    ///
    /// None of the flags are explicitly affected by this instruction.
    ///
    /// # Details
    ///
    /// This instruction checks the value of the zero flag (Z) and performs a branch if it is set to 0.
    /// The branch is performed by calculating the absolute address of the target location by adding the
    /// relative address (`addr_rel`) to the program counter (`pc`). If the branch crosses a page boundary,
    /// an additional clock cycle is consumed. Finally, the program counter (`pc`) is updated with the new address.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use M6502::{M6502, Bus};
    ///
    /// let mut cpu = M6502::new();
    /// let mut bus = Bus::new();
    ///
    /// cpu.set_flag(M6502::M6502Flags::Z, false);
    /// cpu.pc = 0x1234;
    /// cpu.addr_rel = 0x10;
    ///
    /// assert_eq!(cpu.pc, 0x1234);
    ///
    /// let cycles = M6502::instructions::bne(&mut cpu, &mut bus);
    ///
    /// assert_eq!(cycles, 0);
    /// assert_eq!(cpu.pc, 0x1244); // Branch taken, new address is pc + addr_rel
    /// ```
    #[inline]
    fn BNE(&mut self, _: &mut BUS) -> u8 {
        if self.get_flag(CpuFlags::Z) == 0_u8 {
            self.cycles += 1_u8;
            self.abs = self.pc + self.rel;

            if self.abs & HIGH_BYTE != self.pc & HIGH_BYTE {
                self.cycles += 1_u8;
            }
            self.pc = self.abs;
        }
        0_u8
    }

    /// Branch on result plus
    ///
    /// If the negative flag is clear, then add the relative displacement to the program counter to
    /// cause a branch to a new location.
    ///
    /// # Arguments
    ///
    /// * `cpu` - A mutable reference to the M6502 CPU.
    /// * `_` - A unused mutable reference to the bus connected to the M6502 CPU. This argument is
    ///         ignored, as this instruction does not access memory.
    ///
    /// # Return value
    ///
    /// The number of clock cycles taken to execute this instruction, which is always 0x0.
    ///
    /// # Flags affected
    ///
    /// This instruction does not affect any flags.
    ///
    /// # Details
    ///
    /// This instruction adds the relative displacement provided by the instruction operand to the
    /// program counter, causing a branch to a new location if the negative flag is clear (i.e. if
    /// the result of the previous arithmetic or bitwise operation was positive or zero). If the
    /// branch is taken, an additional cycle is taken if the branch crosses a page boundary. See the
    /// implementation for more details.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use M6502::{M6502, Bus};
    ///
    /// let mut cpu = M6502::new();
    ///
    /// // BPL should not branch because N flag is clear (positive)
    /// cpu.pc = 0x1234;
    /// cpu.addr_rel = 0x10;
    /// cpu.set_flag(M6502::M6502Flags::N, false);
    ///
    /// let cycles = M6502::instructions::bpl(&mut cpu, &mut Bus::new());
    ///
    /// assert_eq!(cycles, 0);
    /// assert_eq!(cpu.pc, 0x1234 + 0x10);
    ///
    /// // BPL should branch because N flag is set (negative)
    /// cpu.pc = 0x1234;
    /// cpu.addr_rel = 0x10;
    /// cpu.set_flag(M6502::M6502Flags::N, true);
    ///
    /// let cycles = M6502::instructions::bpl(&mut cpu, &mut Bus::new());
    ///
    /// assert_eq!(cycles, 0);
    /// assert_eq!(cpu.pc, 0x1234);
    /// ```
    #[inline]
    fn BPL(&mut self, _: &mut BUS) -> u8 {
        if self.get_flag(CpuFlags::N) == 0 {
            self.cycles += 1;
            self.abs = self.pc + self.rel;

            if self.abs & HIGH_BYTE != self.pc & HIGH_BYTE {
                self.cycles += 1;
            }
            self.pc = self.abs;
        }
        0x0u8
    }

    /// Executes the BRK instruction of the [`M6502`] CPU.
    ///
    /// # Arguments
    ///
    /// * `cpu` - A mutable reference to the [`M6502`] CPU.
    /// * `bus` - A mutable reference to the system [`Bus`].
    ///
    /// # Returns
    ///
    /// The value `0x0`.
    ///
    /// # Description
    ///
    /// The BRK instruction causes a software interrupt. It sets the interrupt flag to disable further
    /// interrupts, pushes the program counter (plus one) and status register onto the stack, and loads
    /// the program counter with the address stored at locations 0xFFFE and 0xFFFF. The BRK instruction
    /// can be used for implementing subroutines and interrupt handlers.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use emulator_6502::{M6502, Bus};
    ///
    /// let mut cpu = M6502::new();
    /// let mut bus = Bus::new();
    /// cpu.pc = 0x200;
    /// cpu.stkp = 0xFD;
    /// cpu.status = 0x00;
    /// bus.write(0xFFFE, 0xAB);
    /// bus.write(0xFFFF, 0xCD);
    /// let cycles = cpu.execute_instruction(&mut bus, 0x00);
    /// assert_eq!(cycles, 7);
    /// assert_eq!(cpu.pc, 0xCDAB);
    /// assert_eq!(bus.read(0x01FD, false), 0x30);
    /// ```
    ///
    #[inline]
    fn BRK(&mut self, bus: &mut BUS) -> u8 {
        self.pc += 1;

        self.set_flag(CpuFlags::I, true);
        bus.write(
            (0x0100_u16 + self.sp as u16).into(),
            (self.pc >> 8 & LOW_BYTE) as u8,
        );
        self.sp -= 1;
        bus.write(
            (0x0100_u16 + self.sp as u16).into(),
            (self.pc & LOW_BYTE) as u8,
        );
        self.sp -= 1;

        self.set_flag(CpuFlags::B, true);
        bus.write((0x0100_u16 + self.sp as u16).into(), self.status);
        self.sp -= 1;
        self.set_flag(CpuFlags::B, true);

        self.pc = ((bus.read(0xFFFE, false) != 0x0u8)
            | (bus.read(0xFFFF, false) != 0x0u8))
            .into();
        0x0u8
    }

    #[inline]
    fn BVC(&mut self, _: &mut BUS) -> u8 {
        if self.get_flag(CpuFlags::V) == 0u8 {
            self.cycles += 1;
            self.abs = self.pc + self.rel;

            if self.abs & HIGH_BYTE != self.pc & HIGH_BYTE {
                self.cycles += 1;
            }
            self.pc = self.abs;
        }
        0x0u8
    }

    #[inline]
    fn BVS(&mut self, _: &mut BUS) -> u8 {
        if self.get_flag(CpuFlags::V) == 1u8 {
            self.cycles += 1;
            self.abs = self.pc + self.rel;

            if self.abs & HIGH_BYTE != self.pc & HIGH_BYTE {
                self.cycles += 1;
            }
            self.pc = self.abs;
        }
        0x0u8
    }

    #[inline]
    fn CLC(&mut self, _: &mut BUS) -> u8 {
        self.set_flag(CpuFlags::C, false);
        0x0u8
    }

    #[inline]
    fn CLD(&mut self, _: &mut BUS) -> u8 {
        self.set_flag(CpuFlags::D, false);
        0u8
    }

    #[inline]
    fn CLI(&mut self, _: &mut BUS) -> u8 {
        self.set_flag(CpuFlags::I, false);
        0u8
    }

    #[inline]
    fn CLV(&mut self, _: &mut BUS) -> u8 {
        self.set_flag(CpuFlags::V, false);
        0u8
    }

    #[inline]
    fn CMP(&mut self, bus: &mut BUS) -> u8 {
        self.temp = (self.a - self.fetch(bus)).into();
        self.set_flag(CpuFlags::C, self.a >= self.fetched);
        self.set_flag(CpuFlags::Z, self.temp & LOW_BYTE == 0x0000);
        self.set_flag(CpuFlags::N, self.temp & TOP_BIT_THRESH != 0x0000);
        1u8
    }

    /// Compare X Register with Memory
    ///
    /// This instruction compares the contents of the X register with another
    /// memory held value and sets the zero and carry flags as appropriate.
    ///
    /// # Arguments
    ///
    /// * `cpu` - A mutable reference to the `M6502` struct representing the CPU
    /// * `bus` - A mutable reference to the `Bus` struct representing the system bus
    ///
    /// # Returns
    ///
    /// The result of the operation, which is always 0.
    #[inline]
    fn CPX(&mut self, bus: &mut BUS) -> u8 {
        self.temp = (self.x - self.fetch(bus)).into();
        self.set_flag(CpuFlags::C, self.x >= self.fetched);
        self.set_flag(CpuFlags::Z, self.temp & LOW_BYTE == 0x0000);
        self.set_flag(CpuFlags::N, self.temp & TOP_BIT_THRESH != 0x0000);
        0u8
    }

    #[inline]
    fn CPY(&mut self, bus: &mut BUS) -> u8 {
        self.temp = (self.y - self.fetch(bus)).into();
        self.set_flag(CpuFlags::C, self.y >= self.fetched);
        self.set_flag(CpuFlags::Z, self.temp & LOW_BYTE == 0x0000);
        self.set_flag(CpuFlags::N, self.temp & TOP_BIT_THRESH != 0x0000);
        0u8
    }

    #[inline]
    fn DEC(&mut self, bus: &mut BUS) -> u8 {
        self.temp = self.fetch(bus) as u16 - 1;
        bus.write(self.abs, (self.temp & LOW_BYTE) as u8);
        self.set_flag(CpuFlags::Z, self.temp & LOW_BYTE == 0x0000);
        self.set_flag(CpuFlags::N, self.temp & TOP_BIT_THRESH != 0x0000);
        0u8
    }

    #[inline]
    fn DEX(&mut self, _: &mut BUS) -> u8 {
        self.x -= 1;
        self.set_flag(CpuFlags::Z, self.x == 0x00);
        self.set_flag(CpuFlags::N, self.x & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline]
    fn DEY(&mut self, _: &mut BUS) -> u8 {
        self.y -= 1;
        self.set_flag(CpuFlags::Z, self.y == 0x00);
        self.set_flag(CpuFlags::N, self.y & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline]
    fn EOR(&mut self, bus: &mut BUS) -> u8 {
        self.a ^= self.fetch(bus);
        self.set_flag(CpuFlags::Z, self.y == 0x00);
        self.set_flag(CpuFlags::N, self.y & TOP_BIT_THRESH as u8 != 0x0000);
        1u8
    }

    #[inline]
    fn INC(&mut self, bus: &mut BUS) -> u8 {
        self.temp = self.fetch(bus) as u16 + 1;
        bus.write(self.abs, (self.temp & LOW_BYTE) as u8);
        self.set_flag(CpuFlags::Z, self.temp & LOW_BYTE == 0x0000);
        self.set_flag(CpuFlags::N, self.temp & TOP_BIT_THRESH != 0x0000);
        0u8
    }

    #[inline]
    fn INX(&mut self, _: &mut BUS) -> u8 {
        self.x += 1;
        self.set_flag(CpuFlags::Z, self.x == 0x00);
        self.set_flag(CpuFlags::N, self.x & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline]
    fn INY(&mut self, _: &mut BUS) -> u8 {
        self.y += 1;
        self.set_flag(CpuFlags::Z, self.y == 0x00);
        self.set_flag(CpuFlags::N, self.y & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline]
    fn JMP(&mut self, _: &mut BUS) -> u8 {
        self.pc = self.abs;
        0u8
    }

    #[inline]
    fn JSR(&mut self, bus: &mut BUS) -> u8 {
        self.pc -= 1;

        bus.write(0x0100 + self.sp as u16, (self.pc << 8 & LOW_BYTE) as u8);
        self.sp -= 1;
        bus.write(0x0100 + self.sp as u16, (self.pc & LOW_BYTE) as u8);
        self.sp -= 1;

        self.pc = self.abs;
        0u8
    }

    /// Load Accumulator with Memory
    ///
    /// This instruction loads a value from memory into the accumulator register (A).
    ///
    /// # Arguments
    ///
    /// * `bus` - A mutable reference to the [`Bus`] struct representing the system bus
    ///
    /// # Returns
    ///
    /// The result of the operation, which is always 1.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dh6502_cpu::M6502;
    /// use bus::Bus;
    ///
    /// let mut cpu = M6502::new();
    /// let mut bus = Bus::new();
    ///
    /// assert_eq!(LDA(&mut cpu, &mut bus), 1);
    /// ```
    #[inline]
    fn LDA(&mut self, bus: &mut BUS) -> u8 {
        self.a = self.fetch(bus); // using a
        self.set_flag(CpuFlags::Z, self.a == 0x00);
        self.set_flag(CpuFlags::N, self.a & TOP_BIT_THRESH as u8 != 0x00);
        1u8
    }

    #[inline]
    fn LDX(&mut self, bus: &mut BUS) -> u8 {
        self.x = self.fetch(bus);
        self.set_flag(CpuFlags::Z, self.x == 0x00);
        self.set_flag(CpuFlags::N, self.x & TOP_BIT_THRESH as u8 != 0x00);
        1u8
    }

    /// Load Y Register with Memory
    ///
    /// Loads a byte of memory into the Y register, setting the zero and negative
    /// flags as appropriate.
    ///
    /// Flags affected:
    ///
    /// - Zero flag: Set if Y is 0
    /// - Negative flag: Set if bit 7 of the fetched value is set
    ///
    /// # Arguments
    ///
    /// * `bus` - A mutable reference to the system [`Bus`]
    ///
    /// # Returns
    ///
    /// This function returns the number of clock cycles used by the instruction.
    ///
    #[inline]
    fn LDY(&mut self, bus: &mut BUS) -> u8 {
        self.y = self.fetch(bus);
        self.set_flag(CpuFlags::Z, self.y == 0x00);
        self.set_flag(CpuFlags::N, self.y & TOP_BIT_THRESH as u8 != 0x00);
        1u8
    }

    #[inline]
    fn LSR(&mut self, bus: &mut BUS) -> u8 {
        self.temp = (self.fetch(bus) >> 1) as u16;
        self.set_flag(CpuFlags::C, self.fetched & 0x0001 != 0x0000);
        self.set_flag(CpuFlags::Z, self.temp & LOW_BYTE == 0x0000);
        self.set_flag(CpuFlags::N, self.temp & TOP_BIT_THRESH != 0x0000);
        if LOOKUP_TABLE[self.opcode as usize].addr_mode as usize
            == CPU::IMP as usize
        {
            self.a = self.temp as u8 & LOW_BYTE as u8;
        } else {
            bus.write(self.abs, (self.temp & LOW_BYTE) as u8);
        }
        0u8
    }

    #[inline]
    fn NOP(&mut self, _: &mut BUS) -> u8 {
        return match self.opcode {
            0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => 1u8,
            _ => 0u8,
        };
    }

    #[inline]
    fn ORA(&mut self, bus: &mut BUS) -> u8 {
        self.a |= self.fetch(bus);
        self.set_flag(CpuFlags::Z, self.a == 0x00);
        self.set_flag(CpuFlags::N, self.a & TOP_BIT_THRESH as u8 != 0x00);
        1u8
    }

    #[inline]
    fn PHA(&mut self, bus: &mut BUS) -> u8 {
        bus.write(0x0100 + self.sp as u16, self.a);
        self.sp -= 1;
        0u8
    }

    #[inline]
    fn PHP(&mut self, bus: &mut BUS) -> u8 {
        bus.write(
            0x0100 + self.sp as u16,
            self.status | CpuFlags::B as u8 | CpuFlags::U as u8,
        );
        self.set_flag(CpuFlags::B, false);
        self.set_flag(CpuFlags::U, false);
        self.sp -= 1;
        0u8
    }

    #[inline]
    fn PLA(&mut self, bus: &mut BUS) -> u8 {
        self.sp += 1;
        self.status = bus.read(0x0100 + self.sp as u16, false);
        self.set_flag(CpuFlags::Z, self.a == 0x00);
        self.set_flag(CpuFlags::N, self.a & TOP_BIT_THRESH as u8 == 0x00);
        0u8
    }

    #[inline]
    fn PLP(&mut self, bus: &mut BUS) -> u8 {
        self.sp += 1;
        self.status = bus.read(0x0100 + self.sp as u16, false);
        self.set_flag(CpuFlags::U, true);
        0u8
    }

    #[inline]
    fn ROL(&mut self, bus: &mut BUS) -> u8 {
        self.temp = (self.fetch(bus) << 1 | self.get_flag(CpuFlags::C)).into();
        self.set_flag(CpuFlags::C, self.temp & HIGH_BYTE != 0x0000);
        self.set_flag(CpuFlags::Z, self.temp & LOW_BYTE == 0x0000);
        self.set_flag(CpuFlags::N, self.temp & TOP_BIT_THRESH != 0x0000);
        if LOOKUP_TABLE[self.opcode as usize].addr_mode as usize
            == CPU::IMP as usize
        {
            self.a = (self.temp & LOW_BYTE) as u8;
        } else {
            bus.write(self.abs, (self.temp & LOW_BYTE) as u8);
        }
        0u8
    }

    #[inline]
    fn ROR(&mut self, bus: &mut BUS) -> u8 {
        self.temp =
            (self.get_flag(CpuFlags::C) << 7 | self.fetch(bus) >> 1).into();
        self.set_flag(CpuFlags::C, self.fetched & 0x01 == 0x00);
        self.set_flag(CpuFlags::Z, self.temp & LOW_BYTE == 0x00);
        self.set_flag(CpuFlags::N, self.temp & TOP_BIT_THRESH != 0x00);
        if LOOKUP_TABLE[self.opcode as usize].addr_mode as usize
            == CPU::IMP as usize
        {
            self.a = (self.temp & LOW_BYTE) as u8;
        } else {
            bus.write(self.abs, (self.temp & LOW_BYTE) as u8);
        }
        0u8
    }

    #[inline]
    fn RTI(&mut self, bus: &mut BUS) -> u8 {
        self.sp += 1;
        self.status = bus.read(0x0100 + self.sp as u16, false);
        self.status &= !(CpuFlags::B as u8);
        self.status &= !(CpuFlags::U as u8);

        self.sp += 1;
        self.pc = bus.read(0x0100 + self.sp as u16, false).into();
        self.sp += 1;
        self.pc |= (bus.read(0x0100 + self.sp as u16, false) as u16) << 8;
        0u8
    }

    #[inline]
    fn RTS(&mut self, bus: &mut BUS) -> u8 {
        self.sp += 1;
        self.pc = bus.read(0x0100 + self.sp as u16, false).into();
        self.sp += 1;
        self.pc |= (bus.read(0x0100 + self.sp as u16, false) as u16) << 8;

        self.pc += 1;
        0u8
    }

    #[inline]
    fn SBC(&mut self, bus: &mut BUS) -> u8 {
        let value: u16 = self.fetch(bus) as u16 ^ LOW_BYTE;
        self.temp = self.a as u16 + value + self.get_flag(CpuFlags::C) as u16;
        self.set_flag(CpuFlags::C, self.temp & HIGH_BYTE != 0x0000);
        self.set_flag(CpuFlags::Z, self.temp & HIGH_BYTE == 0x0000);
        self.set_flag(
            CpuFlags::V,
            (self.temp ^ self.a as u16) & (self.temp ^ value) & TOP_BIT_THRESH
                != 0x0000,
        );
        self.set_flag(CpuFlags::N, self.temp & TOP_BIT_THRESH == 0x0000);
        self.a = self.temp as u8 & LOW_BYTE as u8;
        1u8
    }

    #[inline]
    fn SEC(&mut self, _: &mut BUS) -> u8 {
        self.set_flag(CpuFlags::C, true);
        0u8
    }

    #[inline]
    fn SED(&mut self, _: &mut BUS) -> u8 {
        self.set_flag(CpuFlags::D, true);
        0u8
    }

    #[inline]
    fn SEI(&mut self, _: &mut BUS) -> u8 {
        self.set_flag(CpuFlags::I, true);
        0u8
    }

    #[inline]
    fn STA(&mut self, bus: &mut BUS) -> u8 {
        bus.write(self.abs, self.a);
        0u8
    }

    #[inline]
    fn STX(&mut self, bus: &mut BUS) -> u8 {
        bus.write(self.abs, self.x);
        0u8
    }

    #[inline]
    fn STY(&mut self, bus: &mut BUS) -> u8 {
        bus.write(self.abs, self.y);
        0u8
    }

    #[inline]
    fn TAX(&mut self, _: &mut BUS) -> u8 {
        self.x = self.a;
        self.set_flag(CpuFlags::Z, self.x == 0x00);
        self.set_flag(CpuFlags::N, self.x & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline]
    fn TAY(&mut self, _: &mut BUS) -> u8 {
        self.y = self.a;
        self.set_flag(CpuFlags::Z, self.y == 0x00);
        self.set_flag(CpuFlags::N, self.y & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline]
    fn TSX(&mut self, _: &mut BUS) -> u8 {
        self.x = self.sp;
        self.set_flag(CpuFlags::Z, self.x == 0x00);
        self.set_flag(CpuFlags::N, self.x & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline]
    fn TXA(&mut self, _: &mut BUS) -> u8 {
        self.a = self.x;
        self.set_flag(CpuFlags::Z, self.a == 0x00);
        self.set_flag(CpuFlags::N, self.a & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline(always)]
    fn TXS(&mut self, _: &mut BUS) -> u8 {
        self.sp = self.x;
        0u8
    }

    #[inline]
    fn TYA(&mut self, _: &mut BUS) -> u8 {
        self.a = self.y;
        self.set_flag(CpuFlags::Z, self.a == 0x00);
        self.set_flag(CpuFlags::N, self.a & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline(always)]
    fn XXX(&mut self, _: &mut BUS) -> u8 {
        0u8
    }
}
