#![allow(non_snake_case)]

use crate::AddressingMode;
use crate::{dh_bus::BUS, dh_cpu::CPU, HIGH_BYTE, LOW_BYTE};

impl AddressingMode for CPU {
    /// Implied Addressing (IMP)
    ///
    /// The `IMP` addressing mode is used for instructions that have an implied operand.
    /// In this addressing mode, the instruction operates on the CPU's registers or flags
    /// without the need to fetch data from memory or use additional operands.
    ///
    /// Flags affected: None
    ///
    /// # Arguments
    ///
    /// * `_bus` - A mutable reference to the system [`Bus`]. This reference is not used in this addressing mode.
    ///
    /// # Returns
    ///
    /// This function returns 0, as it does not affect clock cycles or execution time.
    ///
    /// # Example
    ///
    /// ```rust no_run
    /// // Example usage of the IMP addressing mode
    /// let mut cpu = CPU::new();
    ///
    /// // Set a value in the accumulator register
    /// cpu.a = 0x42;
    ///
    /// IMP(&mut cpu, &mut bus); // Execute the IMP instruction
    ///
    /// // The `fetched` register in the `cpu` will now hold the value from the accumulator.
    /// ```
    fn IMP(&mut self, _: &mut BUS) -> u8 {
        self.fetched = self.a;
        0x00
    }

    /// Immediate Addressing (IMM)
    ///
    /// The `IMM` addressing mode is used to directly load an 8-bit value from the next
    /// byte in the instruction stream. The value is stored in the `abs` register and is
    /// not fetched from memory.
    ///
    /// Flags affected: None
    ///
    /// # Arguments
    ///
    /// * `_bus` - A mutable reference to the system [`Bus`]. This reference is not used in this addressing mode.
    ///
    /// # Returns
    ///
    /// This function returns 0, as it does not affect clock cycles or execution time.
    ///
    /// # Example
    ///
    /// ```rust no_run
    /// // Example usage of the IMM addressing mode
    /// let mut cpu = CPU::new();
    ///
    /// cpu.pc = 0x8000; // Set the program counter to the address of the IMM instruction
    /// IMM(&mut cpu, &mut bus); // Execute the IMM instruction
    ///
    /// // The `abs` register in the `cpu` will now hold the value from the next byte
    /// // in the instruction stream.
    /// ```
    fn IMM(&mut self, _bus: &mut BUS) -> u8 {
        self.abs = self.pc;
        0x00
    }

    /// Zero Page Addressing (ZP0)
    ///
    /// The `ZP0` addressing mode is used to load an 8-bit value from a zero page address
    /// specified by the byte at the current program counter (PC). This addressing mode is
    /// often used for accessing data in the zero page of memory.
    ///
    /// Flags affected: None
    ///
    /// # Arguments
    ///
    /// * `bus` - A mutable reference to the system [`Bus`] for memory access.
    ///
    /// # Returns
    ///
    /// This function returns 0, as it does not affect clock cycles or execution time.
    ///
    /// # Example
    ///
    /// ```rust no_run
    /// // Example usage of the ZP0 addressing mode
    /// let mut cpu = CPU::new();
    /// let mut bus = Bus::new();
    ///
    /// // Set the memory content at a specific zero page address
    /// bus.write(0x50, 0x42); // Data at the zero page address 0x50
    ///
    /// cpu.pc = 0x8000; // Set the program counter to the address of the ZP0 instruction
    /// ZP0(&mut cpu, &mut bus); // Execute the ZP0 instruction
    ///
    /// // The `abs` register in the `cpu` will now hold the value 0x42 from the zero page.
    /// ```
    fn ZP0(&mut self, bus: &mut BUS) -> u8 {
        self.abs = bus.read(self.pc, false) as u16;
        self.pc += 1;
        self.abs &= LOW_BYTE; // checking if high bit is on a new page
        0x00
    }

    /// Zero Page Indexed with X Register Addressing (ZPX)
    ///
    /// The `ZPX` addressing mode is used to load an 8-bit value from a zero page address,
    /// which is the sum of the value in the X register and the byte at the current program
    /// counter (PC). This addressing mode is often used for indexed memory access in the
    /// zero page.
    ///
    /// Flags affected: None
    ///
    /// # Arguments
    ///
    /// * `bus` - A mutable reference to the system [`Bus`] for memory access.
    ///
    /// # Returns
    ///
    /// This function returns 0, as it does not affect clock cycles or execution time.
    ///
    /// # Example
    ///
    /// ```rust no_run
    /// // Example usage of the ZPX addressing mode
    /// let mut cpu = CPU::new();
    /// let mut bus = Bus::new();
    ///
    /// // Set the memory content at a specific zero page address
    /// bus.write(0x50, 0x42); // Data at the zero page address 0x50
    ///
    /// cpu.pc = 0x8000; // Set the program counter to the address of the ZPX instruction
    /// cpu.x = 0x0A; // Set the X register
    /// ZPX(&mut cpu, &mut bus); // Execute the ZPX instruction
    ///
    /// // The `abs` register in the `cpu` will now hold the value 0x42 from the zero page.
    /// ```
    fn ZPX(&mut self, bus: &mut BUS) -> u8 {
        self.abs = bus.read(self.pc, false).wrapping_add(self.x) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.abs &= LOW_BYTE;
        0
    }

    /// Zero Page Indexed with Y Register Addressing (ZPY)
    ///
    /// The `ZPY` addressing mode is used to load an 8-bit value from a zero page address,
    /// which is the sum of the value in the Y register and the byte at the current program
    /// counter (PC). This addressing mode is often used for indexed memory access in the
    /// zero page.
    ///
    /// Flags affected: None
    ///
    /// # Arguments
    ///
    /// * `bus` - A mutable reference to the system [`Bus`] for memory access.
    ///
    /// # Returns
    ///
    /// This function returns 0, as it does not affect clock cycles or execution time.
    ///
    /// # Example
    ///
    /// ```rust no_run
    /// // Example usage of the ZPY addressing mode
    /// let mut cpu = CPU::new();
    /// let mut bus = Bus::new();
    ///
    /// // Set the memory content at a specific zero page address
    /// bus.write(0x50, 0x42); // Data at the zero page address 0x50
    ///
    /// cpu.pc = 0x8000; // Set the program counter to the address of the ZPY instruction
    /// cpu.y = 0x0A; // Set the Y register
    /// ZPY(&mut cpu, &mut bus); // Execute the ZPY instruction
    ///
    /// // The `abs` register in the `cpu` will now hold the value 0x42 from the zero page.
    /// ```
    fn ZPY(&mut self, bus: &mut BUS) -> u8 {
        self.abs = bus.read(self.pc, false).wrapping_add(self.y) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.abs &= LOW_BYTE;
        0
    }

    /// Absolute Addressing (ABS)
    ///
    /// The `ABS` addressing mode is used to load a 16-bit absolute memory address
    /// from two consecutive bytes in memory and store it in the CPU's `abs` register.
    /// The absolute address is formed by combining a 16-bit little-endian value from
    /// two sequential memory locations.
    ///
    /// Flags affected: None
    ///
    /// # Arguments
    ///
    /// * `bus` - A mutable reference to the system [`Bus`] for memory access.
    ///
    /// # Returns
    ///
    /// This function returns the number of clock cycles used by the instruction, which is 0.
    ///
    /// # Example
    ///
    /// ```rust no_run
    /// // Example usage of the ABS addressing mode
    /// let mut cpu = CPU::new();
    /// let mut bus = Bus::new();
    ///
    /// // Set the memory content at a specific address
    /// bus.write(0x8000, 0x12); // LSB
    /// bus.write(0x8001, 0x34); // MSB
    ///
    /// cpu.pc = 0x8000; // Set the program counter to the address of the ABS instruction
    /// ABS(&mut cpu, &mut bus); // Execute the ABS instruction
    ///
    /// // The `abs` register in the `cpu` will now hold the value 0x3412 (little-endian).
    /// ```
    fn ABS(&mut self, bus: &mut BUS) -> u8 {
        let lo = bus.read(self.pc as u16, false);
        self.pc = self.pc.wrapping_add(1);

        let hi = bus.read(self.pc as u16, false);
        self.pc = self.pc.wrapping_add(1);

        self.abs = u16::from_le_bytes([lo, hi]);
        0
    }

    /// Absolute Indexed with X Register Addressing (ABX)
    ///
    /// The `ABX` addressing mode is used to load a 16-bit absolute memory address
    /// from two consecutive bytes in memory, add the value in the X register to it,
    /// and store the result in the CPU's `abs` register. This addressing mode is often
    /// used for indexed memory access.
    ///
    /// Flags affected: None
    ///
    /// # Arguments
    ///
    /// * `bus` - A mutable reference to the system [`Bus`] for memory access.
    ///
    /// # Returns
    ///
    /// This function returns the number of clock cycles used by the instruction, which is 1 if a page boundary is crossed, otherwise 0.
    ///
    /// # Example
    ///
    /// ```rust no_run
    /// // Example usage of the ABX addressing mode
    /// let mut cpu = CPU::new();
    /// let mut bus = Bus::new();
    ///
    /// // Set the memory content at a specific address
    /// bus.write(0x8000, 0x12); // LSB
    /// bus.write(0x8001, 0x34); // MSB
    ///
    /// cpu.pc = 0x8000; // Set the program counter to the address of the ABX instruction
    /// cpu.x = 0x10; // Set the X register
    /// ABX(&mut cpu, &mut bus); // Execute the ABX instruction
    ///
    /// // The `abs` register in the `cpu` will now hold the value 0x3422 (little-endian)
    /// // since X was added to the absolute address.
    /// ```
    fn ABX(&mut self, bus: &mut BUS) -> u8 {
        let lo = bus.read(self.pc as u16, false);
        self.pc = self.pc.wrapping_add(1);

        let hi = bus.read(self.pc as u16, false);
        self.pc = self.pc.wrapping_add(1);

        let base = u16::from_le_bytes([lo, hi]);
        self.abs = base.wrapping_add(self.x as u16);

        ((self.abs & HIGH_BYTE) != (base & HIGH_BYTE)) as u8
    }

    /// Absolute Indexed with Y Register Addressing (ABY)
    ///
    /// The `ABY` addressing mode is used to load a 16-bit absolute memory address
    /// from two consecutive bytes in memory, add the value in the Y register to it,
    /// and store the result in the CPU's `abs` register. This addressing mode is often
    /// used for indexed memory access with the Y register.
    ///
    /// Flags affected: None
    ///
    /// # Arguments
    ///
    /// * `bus` - A mutable reference to the system [`Bus`] for memory access.
    ///
    /// # Returns
    ///
    /// This function returns the number of clock cycles used by the instruction, which is 1 if a page boundary is crossed, otherwise 0.
    ///
    /// # Example
    ///
    /// ```rust no_run
    /// // Example usage of the ABY addressing mode
    /// let mut cpu = CPU::new();
    /// let mut bus = Bus::new();
    ///
    /// // Set the memory content at a specific address
    /// bus.write(0x8000, 0x12); // LSB
    /// bus.write(0x8001, 0x34); // MSB
    ///
    /// cpu.pc = 0x8000; // Set the program counter to the address of the ABY instruction
    /// cpu.y = 0x10; // Set the Y register
    /// ABY(&mut cpu, &mut bus); // Execute the ABY instruction
    ///
    /// // The `abs` register in the `cpu` will now hold the value 0x3422 (little-endian)
    /// // since Y was added to the absolute address.
    /// ```
    fn ABY(&mut self, bus: &mut BUS) -> u8 {
        let lo = bus.read(self.pc, false);
        self.pc = self.pc.wrapping_add(1);

        let hi = bus.read(self.pc, false);
        self.pc = self.pc.wrapping_add(1);

        let base = u16::from_le_bytes([lo, hi]);
        self.abs = base.wrapping_add(self.y as u16);

        ((self.abs & HIGH_BYTE) != (base & HIGH_BYTE)) as u8
    }

    /// Relative Addressing (REL)
    ///
    /// The `REL` addressing mode is used for branch instructions, which involve a signed
    /// 8-bit offset relative to the current program counter (PC). This offset is read from
    /// memory, sign-extended to 16 bits, and stored in the CPU's `rel` register. It is used
    /// to determine the destination address for branching.
    ///
    /// Flags affected: None
    ///
    /// # Arguments
    ///
    /// * `bus` - A mutable reference to the system [`Bus`] for memory access.
    ///
    /// # Returns
    ///
    /// This function returns 0, as it does not affect clock cycles or execution time.
    ///
    /// # Example
    ///
    /// ```rust no_run
    /// // Example usage of the REL addressing mode for branching
    /// let mut cpu = CPU::new();
    /// let mut bus = Bus::new();
    ///
    /// // Set the memory content at a specific address with a relative offset
    /// bus.write(0x8000, 0x10); // Relative offset of +16 (positive)
    ///
    /// cpu.pc = 0x8000; // Set the program counter to the address of the REL instruction
    /// REL(&mut cpu, &mut bus); // Execute the REL instruction
    ///
    /// // The `rel` register in the `cpu` will now hold the value 16 (sign-extended).
    /// ```
    fn REL(&mut self, bus: &mut BUS) -> u8 {
        self.rel = bus.read(self.pc, false) as u16;
        self.pc += 1;
        if (self.rel & 0x80) != 0 {
            self.rel |= LOW_BYTE;
        }
        0
    }

    /// This function implements the "Indirect" addressing mode for the M6502 CPU.
    /// It reads the two bytes located at the program counter address, and uses them as a 16-bit pointer
    /// to read the actual 16-bit address from memory, which is then stored in the cpu's addr_abs register.
    ///
    /// # Arguments
    ///
    /// * `cpu` - A mutable reference to the [`M6502`] CPU.
    /// * `bus` - A mutable reference to the [`Bus`] struct representing the system bus
    ///
    /// # Returns
    ///
    /// This function always returns 0x00, as this addressing mode doesn't perform any actual computation.
    ///
    /// # Example
    ///
    ///```no_run
    /// # use rust_computer_emulator::components::{M6502, Bus};
    /// # let mut cpu = M6502::new();
    /// # let mut bus = Bus::new();
    /// cpu.pc = 0x1000;
    /// bus.write(0x1000, 0x42);
    /// bus.write(0x1001, 0x84);
    ///
    /// let result = M6502::ind(&mut cpu, &mut bus);
    /// assert_eq!(cpu.addr_abs, 0x8442);
    /// assert_eq!(result, 0x00);
    ///```
    fn IND(&mut self, bus: &mut BUS) -> u8 {
        let ptr_lo = bus.read(self.pc, false);
        self.pc = self.pc.wrapping_add(1);

        let ptr_hi = bus.read(self.pc, false);
        self.pc = self.pc.wrapping_add(1);

        let ptr = u16::from_le_bytes([ptr_lo, ptr_hi]);

        // Replicate the 6502 page-boundary hardware bug:
        // if the low byte of the pointer is 0xFF the high byte wraps within the page.
        let lo: u8;
        let hi: u8;
        if u16::from(ptr_lo) == LOW_BYTE {
            lo = bus.read(ptr & HIGH_BYTE, false);
            hi = bus.read(ptr, false);
        } else {
            lo = bus.read(ptr.wrapping_add(1), false);
            hi = bus.read(ptr, false);
        }
        self.abs = u16::from_le_bytes([lo, hi]);
        0
    }

    /// Indirect Zero-Page Indexed with X Addressing Mode
    ///
    /// This addressing mode is used by certain instructions to access memory indirectly,
    /// using a zero page address that is added to the X register. The address is read
    /// from the zero page address (t + X) and the low byte is used as the lower 8 bits
    /// of the effective address, while the high byte is fetched from the next location
    /// in memory (t + X + 1), wrapping around if necessary. This results in an effective
    /// address that can range from 0x0000 to 0xFFFF, with the X register being added to
    /// the zero page address t, and the page boundary crossing detection being performed
    /// on the resulting effective address.
    ///
    /// # Arguments
    ///
    /// * `bus` - A mutable reference to the [`Bus`] struct representing the system bus
    ///
    /// # Returns
    ///
    /// The result of the operation, which is always 0x00.
    ///
    /// # Examples
    ///```no_run
    /// use rust6502::M6502;
    /// use rust6502::Bus;
    /// let mut cpu = M6502::new();
    /// let mut bus = Bus::new();
    ///
    /// cpu.x = 0x04;
    /// bus.write(0x10, 0x05);
    /// bus.write(0x11, 0x06);
    /// bus.write(0x0605, 0x42);
    ///
    /// assert_eq!(cpu.pc, 0x0000);
    /// assert_eq!(cpu.addr_abs, 0x0000);
    ///
    /// M6502::IZX(&mut cpu, &mut bus);
    /// assert_eq!(cpu.pc, 0x0001);
    /// assert_eq!(cpu.addr_abs, 0x4205);
    ///
    /// let result = bus.read(cpu.addr_abs, true);
    /// assert_eq!(result, 0x42);
    /// ```
    fn IZX(&mut self, bus: &mut BUS) -> u8 {
        let t: u8 = bus.read(self.pc, false);
        self.pc = self.pc.wrapping_add(1);

        let lo = bus.read((t.wrapping_add(self.x)) as u16 & LOW_BYTE, false);
        let hi = bus.read(
            (t.wrapping_add(self.x).wrapping_add(1)) as u16 & LOW_BYTE,
            false,
        );

        self.abs = u16::from_le_bytes([lo, hi]);
        self.abs = hi.wrapping_shl(hi as u32) as u16 | lo as u16;
        0
    }

    /// Indirect Indexed with Y Addressing Mode
    ///
    /// This addressing mode is used by certain instructions to access memory
    /// indirectly, using a zero page address that is added to the Y register.
    ///
    /// # Arguments
    ///
    /// * `bus` - A mutable reference to the [`Bus`] struct representing the system bus
    ///
    /// # Returns
    ///
    /// The result of the operation, which is either 0 or 1 depending on whether
    /// the operation resulted in a page boundary crossing.
    fn IZY(&mut self, bus: &mut BUS) -> u8 {
        let temp = bus.read(self.pc, false) as u16;
        self.pc = self.pc.wrapping_add(1);

        let lo = bus.read(temp & LOW_BYTE, false); // FIX: was (t + y) & LO
        let hi = bus.read((temp + 1) & LOW_BYTE, false); // FIX: was (t + y + 1) & LO

        let base = u16::from_le_bytes([lo, hi]);
        self.abs = base.wrapping_add(self.y as u16);

        ((self.abs & HIGH_BYTE) != (base & HIGH_BYTE)) as u8
    }
}
