use std::collections::HashMap;

use crate::util::helper_func as helpers;

use super::bus::Bus;
use super::types::{M6502AddrModes, M6502Flags, M6502Opcodes};
use super::{HIGH_BYTE, LOOKUP_TABLE, LOW_BYTE, TOP_BIT_THRESH};

#[derive(Debug, Clone)]
pub struct M6502 {
    // cpu Core registers, exposed as public here for ease of access from external
    // examinors. This is all the 6502 has.
    pub acc: u8,    // Accumulator Register
    pub x: u8,      // X Register
    pub y: u8,      // Y Register
    pub stkp: u8,   // Stack Pointer (points to location on cpu.bus)
    pub pc: u16,    // Program Counter
    pub status: u8, // Status Register

    // Assisstive variables to facilitate emulation
    pub fetched: u8,      // Represents the working input value to the ALU
    pub temp: u16,        // A convenience variable used everywhere
    pub addr_abs: u16,    // All used memory addresses end up in here
    pub addr_rel: u16,    // Represents absolute address following a branch
    pub opcode: u8,       // Is the instruction byte
    pub cycles: u8,       // Counts how many cycles the instruction has remaining
    pub clock_count: u32, // A global accumulation of the number of clocks
}

impl M6502 {
    #[inline]
    pub const fn new() -> Self {
        Self {
            acc: 0x00,
            x: 0x00,
            y: 0x00,
            stkp: 0x00,
            pc: 0x0000,
            status: M6502Flags::E as u8,
            fetched: 0x00,
            temp: 0x0000,
            addr_abs: 0x0000,
            addr_rel: 0x0000,
            opcode: 0x00,
            cycles: 0,
            clock_count: 0,
        }
    }

    /// Fetches the next byte of data from the specified address in memory.
    ///
    /// # Arguments
    ///
    /// * `bus` - A reference to the system bus.
    ///
    /// # Description
    ///
    /// The `fetch` function retrieves the next byte of data from the specified address in memory, as determined
    /// by the current instruction being executed. If the current instruction is using an implied addressing mode,
    /// no memory access is performed and the fetched value remains unchanged. Otherwise, the `addr_abs` field of
    /// the CPU is used to retrieve the value from memory, and the result is stored in the `fetched` field of the CPU.
    ///
    /// # Return value
    ///
    /// The function returns the fetched value.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use emulator_6502::{M6502, Bus};
    ///
    /// let mut cpu = M6502::new();
    /// let mut bus = Bus::new();
    ///
    /// // Set up the bus with some data at address 0x1234
    /// bus.write(0x1234, 0xAB);
    ///
    /// // Set the CPU's program counter to point to an instruction that fetches from 0x1234
    /// cpu.pc = 0x8000;
    /// cpu.opcode = 0xAD;
    /// cpu.addr_abs = 0x1234;
    ///
    /// // Fetch the byte of data from the specified address
    /// let fetched_value = cpu.fetch(&mut bus);
    ///
    /// assert_eq!(fetched_value, 0xAB);
    /// assert_eq!(cpu.fetched, 0xAB);
    /// ```
    ///
    #[inline]
    pub fn fetch(&mut self, bus: &Bus) -> u8 {
        if !(LOOKUP_TABLE[self.opcode as usize].2 as usize == M6502::imp as usize) {
            self.fetched = bus.read(self.addr_abs, false);
        }
        self.fetched
    }

    /// Sets or clears the specified flag in the M6502 CPU status register.
    ///
    /// # Arguments
    ///
    /// * `f` - The flag to set or clear.
    /// * `conditional_set` - A boolean value indicating whether to set or clear the flag.
    ///
    /// # Description
    ///
    /// The `set_flag` function is used to set or clear the specified flag in the [`M6502`] CPU status register.
    /// The `conditional_set` parameter determines whether the flag is set or cleared. If `conditional_set`
    /// is true, the flag is set. Otherwise, the flag is cleared. The status register is updated accordingly.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use emulator_6502::{M6502, M6502Flags};
    ///
    /// let mut cpu = M6502::new();
    ///
    /// cpu.set_flag(M6502Flags::C, true); // Set the carry flag
    /// assert_eq!(cpu.status, 0x01);
    ///
    /// cpu.set_flag(M6502Flags::C, false); // Clear the carry flag
    /// assert_eq!(cpu.status, 0x00);
    /// ```
    ///
    #[inline]
    pub fn set_flag(&mut self, f: M6502Flags, conditional_set: bool) {
        if conditional_set {
            self.status |= f as u8;
        } else {
            self.status |= !(f as u8) // flip da bits
        }
    }

    /// Resets the M6502 CPU, initializing its registers and program counter.
    ///
    /// # Arguments
    ///
    /// * `cpu` - A mutable reference to the [`M6502`] CPU struct to reset.
    /// * `bus` - A reference to the system [`Bus`] used to read the reset vector from memory.
    ///
    /// # Description
    ///
    /// The `reset` function is used to reset the M6502 CPU to its initial state. It sets the program counter
    /// to the address stored in the reset vector (0xFFFC), initializes the CPU registers and flags, and sets
    /// the number of cycles to 8.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use emulator_6502::{M6502, Bus};
    ///
    /// let mut cpu = M6502::new();
    /// let bus = Bus::new();
    ///
    /// M6502::reset(&mut cpu, &bus);
    ///
    /// assert_eq!(cpu.pc, 0x0000);
    /// assert_eq!(cpu.acc, 0x00);
    /// assert_eq!(cpu.x, 0x00);
    /// assert_eq!(cpu.y, 0x00);
    /// assert_eq!(cpu.stkp, 0xFD);
    /// assert_eq!(cpu.status, 0x04);
    /// assert_eq!(cpu.addr_rel, 0x0000);
    /// assert_eq!(cpu.addr_abs, 0x0000);
    /// assert_eq!(cpu.fetched, 0x00);
    /// assert_eq!(cpu.cycles, 8);
    /// ```
    ///
    pub fn reset(cpu: &mut M6502, bus: &Bus) {
        cpu.addr_abs = 0xFFFC;
        let low: u16 = bus.read(cpu.addr_abs + 0, false) as u16;
        let high: u16 = bus.read(cpu.addr_abs + 1, false) as u16;

        cpu.pc = (high << 8) << low;

        cpu.acc = 0;
        cpu.x = 0;
        cpu.y = 0;
        cpu.stkp = 0xFD;
        cpu.status = 0x00 | M6502Flags::U as u8;

        cpu.addr_rel = 0x0000;
        cpu.addr_abs = 0x0000;
        cpu.fetched = 0x00;

        cpu.cycles = 8; // resets take a long time
    }

    // Simulates a clock cycle of the 6502 CPU.
    ///
    /// This function is responsible for fetching and executing the current instruction pointed to by the program counter (PC) of the CPU.
    /// It reads the opcode from memory using the bus, sets the U flag, and advances the PC.
    /// It then looks up the number of cycles required to execute the instruction from a lookup table, adds any additional cycles required,
    /// and updates the CPU's cycle count accordingly.
    ///
    /// # Arguments
    ///
    /// * `cpu` - A mutable reference to the [`M6502`] struct representing the 6502 CPU being simulated.
    /// * `bus` - A mutable reference to the [`Bus`] struct representing the memory and I/O bus connected to the CPU.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut cpu = M6502::new();
    /// let mut bus = Bus::new();
    /// M6502::clock(&mut cpu, &mut bus);
    /// ```
    pub fn clock(cpu: &mut M6502, bus: &mut Bus) {
        if cpu.complete() {
            cpu.opcode = bus.read(cpu.pc, true);
            cpu.set_flag(M6502Flags::U, true);
            cpu.pc += 1;
            cpu.cycles = LOOKUP_TABLE[cpu.opcode as usize].3;

            let added_cycle1: u8 = LOOKUP_TABLE[cpu.opcode as usize].1(cpu, bus);
            let added_cycle2: u8 = LOOKUP_TABLE[cpu.opcode as usize].2(cpu, bus);

            cpu.cycles += added_cycle1 & added_cycle2;
            cpu.set_flag(M6502Flags::U, true);
        }
        cpu.clock_count += 1;
        cpu.cycles -= 1;
    }

    /// Returns the value of a specific flag in the status register.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the M6502 CPU struct containing the status register.
    /// * `f` - The flag to retrieve the value of, represented as a `M6502Flags` enum variant.
    ///
    /// # Returns
    ///
    /// `1` if the specified flag is set in the status register, `0` otherwise.
    ///
    /// # Description
    ///
    /// The `get_flag` function is used to retrieve the value of a specific flag in the status register.
    /// The `f` argument is an enum variant of the `M6502Flags` type, representing the flag to retrieve the
    /// value of. The function returns `1` if the specified flag is set in the status register, and `0`
    /// otherwise.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use emulator_6502::{M6502Flags, M6502};
    ///
    /// let mut cpu = M6502::new();
    /// cpu.status = M6502Flags::C as u8 | M6502Flags::Z as u8;
    ///
    /// assert_eq!(cpu.get_flag(M6502Flags::C), 1);
    /// assert_eq!(cpu.get_flag(M6502Flags::Z), 1);
    /// assert_eq!(cpu.get_flag(M6502Flags::I), 0);
    /// ```
    ///
    #[inline(always)]
    pub const fn get_flag(&self, f: M6502Flags) -> u8 {
        return if self.status & f as u8 > 0 { 1u8 } else { 0u8 };
    }

    /// Returns a boolean indicating whether the current operation is complete or not.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the struct containing the operation information.
    ///
    /// # Returns
    ///
    /// `true` if the operation has completed, `false` otherwise.
    ///
    /// # Description
    ///
    /// The `complete` function is used to check if the current operation has completed. It returns `true`
    /// if the `cycles` field of the struct is equal to zero, indicating that the operation has completed.
    /// Otherwise, it returns `false`, indicating that the operation is still in progress and needs to be
    /// executed for additional cycles.
    ///
    /// # Example
    ///
    /// ```no_run
    /// struct Operation {
    ///     Cycles: u8,
    /// }
    ///
    /// let operation = Operation { Cycles: 0 };
    ///
    /// assert!(operation.complete());
    /// ```
    ///
    #[inline(always)]
    pub const fn complete(&self) -> bool {
        self.cycles == 0
    }

    /// Disassembles the code within the specified memory range [start, stop] and returns a HashMap containing the
    /// disassembled code, with the key being the address of the instruction and the value being a String representation
    /// of the instruction.
    ///
    /// # Arguments
    ///
    /// * bus - A mutable reference to the [`Bus`] object used to access the memory of the CPU.
    /// * start - The starting address of the memory range to disassemble.
    /// * stop - The ending address of the memory range to disassemble.
    ///
    /// # Returns
    ///
    /// A HashMap<u16, String> containing the disassembled code, with the key being the address of the instruction and
    /// the value being a String representation of the instruction.
    pub fn disassemble(bus: &mut Bus, start: u16, stop: u16) -> HashMap<u16, String> {
        // Initialize variables for tracking the current address, instruction value, and line address.
        let mut address: u32 = start.into();
        let mut value: u8 = 0x00;
        let mut low: u8 = 0x00;
        let mut high: u8 = 0x00;
        let mut line_address: u16 = 0;

        // Create a HashMap to store the resulting instructions with their corresponding line address.
        let mut lined_maps: HashMap<u16, String> = HashMap::<u16, String>::new();

        // Loop through memory between start and stop addresses.
        while address <= stop as u32 {
            line_address = address as u16;

            // Initialize a string to hold the address and instruction for the current line.
            let mut instruction_address: String =
                format!("${}{}", helpers::to_hex(address, 4), ": ");

            // Read the opcode from memory at the current address.
            let opcode: u8 = bus.read(address as u16, true);

            address += 1;
            instruction_address.push_str(format!("{} ", LOOKUP_TABLE[opcode as usize].0).as_str());

            if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::imp as usize {
                // Implied addressing mode (no operand)

                instruction_address.push_str(" {IMP}");
            } else if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::imm as usize {
                // Immediate addressing mode (8-bit immediate value)

                value = bus.read(address as u16, true);
                address += 1;
                high = 0x00;
                let string_rep = format!("#${} {{imm}}", helpers::to_hex(low as u32, 2));
                instruction_address.push_str(&string_rep);
            } else if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::zp0 as usize {
                // Zero Page addressing mode (8-bit memory location address)

                low = bus.read(address as u16, true);
                address += 1;
                high = 0x00;
                let string_rep = format!("${} {{zp0}}", helpers::to_hex(low as u32, 2));
                instruction_address.push_str(&string_rep);
            } else if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::zpx as usize {
                // Zero Page X addressing mode (8-bit memory location address + X register)

                low = bus.read(address as u16, true);
                address += 1;
                high = 0x00;
                let string_rep = format!("${}, X {{zpx}}", helpers::to_hex(low as u32, 2));
                instruction_address.push_str(&string_rep);
            } else if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::zpy as usize {
                // Zero Page Y addressing mode (8-bit memory location address + X register)

                low = bus.read(address as u16, true);
                address += 1;
                high = 0x00;
                let string_rep = format!("${}, Y {{zpy}}", helpers::to_hex(low as u32, 2));
                instruction_address.push_str(&string_rep);
            } else if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::izx as usize {
                // If the opcode's addressing mode is indexed indirect with X offset, get the next
                // byte, format it as a hex string with "($...,X)" and add it to the instruction address.

                low = bus.read(address as u16, true);
                address += 1;
                high = 0x00;
                let string_rep = format!("(${}, X) {{izx}}", helpers::to_hex(low as u32, 2));
                instruction_address.push_str(&string_rep);
            } else if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::izy as usize {
                // If the opcode's addressing mode is indirect indexed with Y offset, get the next
                // byte, format it as a hex string with "($...),Y" and add it to the instruction address.

                low = bus.read(address as u16, true);
                address += 1;
                high = 0x00;
                let string_rep = format!("(${}), Y {{izy}}", helpers::to_hex(low as u32, 2));
                instruction_address.push_str(&string_rep);
            } else if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::abs as usize {
                // If the opcode's addressing mode is absolute, get the next two bytes, combine them,
                // format them as a hex string with "{abs}", and add it to the instruction address.

                low = bus.read(address as u16, false);
                address += 1;
                high = bus.read(address as u16, false);
                address += 1;
                let string_rep = format!(
                    "${} {{abs}}",
                    helpers::to_hex(((high << 8) | low) as u32, 4)
                );
                instruction_address.push_str(&string_rep);
            } else if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::abx as usize {
                // If the opcode's addressing mode is absolute with X offset, get the next two bytes,
                // combine them, format them as a hex string with "{abx}", and add it to the instruction address.

                low = bus.read(address as u16, false);
                address += 1;
                high = bus.read(address as u16, false);
                address += 1;
                let string_rep = format!(
                    "${} {{abx}}",
                    helpers::to_hex(((high << 8) | low) as u32, 4)
                );
                instruction_address.push_str(&string_rep);
            } else if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::aby as usize {
                // If the opcode's addressing mode is absolute with Y offset, get the next two bytes,
                // combine them, format them as a hex string with "{aby}", and add it to the instruction address.

                low = bus.read(address as u16, false);
                address += 1;
                high = bus.read(address as u16, false);
                address += 1;
                let string_rep = format!(
                    "${} {{aby}}",
                    helpers::to_hex(((high << 8) | low) as u32, 4)
                );
                instruction_address.push_str(&string_rep);
            } else if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::ind as usize {
                // If the opcode's addressing mode is indirect, get the next two bytes, combine them,
                // format them as a hex string with "($...)", and add it to the instruction address.

                low = bus.read(address as u16, false);
                address += 1;
                high = bus.read(address as u16, false);
                address += 1;
                let string_rep = format!(
                    "(${}) {{ind}}",
                    helpers::to_hex(((high << 8) | low) as u32, 4)
                );
                instruction_address.push_str(&string_rep);
            } else if LOOKUP_TABLE[opcode as usize].2 as usize == M6502::rel as usize {
                // Check if the opcode corresponds to relative addressing mode

                // Read the byte value at the memory address and increment the program counter
                value = bus.read(address as u16, false);
                address += 1;

                // Generate a string representation of the instruction address using the value
                // read and the program counter
                let string_rep = format!(
                    "${} [${}] {{rel}}",
                    helpers::to_hex(value as u32, 2),
                    helpers::to_hex(address + value as u32, 4)
                );

                // Append the string representation to the existing instruction address string
                instruction_address.push_str(&string_rep);
            }
            lined_maps.insert(line_address, instruction_address.clone());
        }

        // resulting instructions with their corresponding line addres
        lined_maps
    }
}

impl M6502Opcodes for M6502 {
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
    fn adc(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        // Grab the data that we are adding to the accumulator
        // Add is performed in 16-bit domain for emulation to capture any
        // carry bit, which will exist in bit 8 of the 16-bit word
        cpu.temp = (cpu.acc + cpu.fetch(bus) + cpu.get_flag(M6502Flags::C)).into();

        // The carry flag out exists in the high byte bit 0
        cpu.set_flag(M6502Flags::C, cpu.temp > 255);

        // The Zero flag is set if the result is 0
        cpu.set_flag(M6502Flags::Z, (cpu.temp & LOW_BYTE) == 0);

        // The signed Overflow flag is set based on all that up there! :D
        cpu.set_flag(
            M6502Flags::V,
            !(cpu.acc as u16 ^ cpu.fetched as u16) & (cpu.acc as u16 ^ cpu.temp) & 0x0080 != 0,
        );

        // The negative flag is set to the most significant bit of the result
        cpu.set_flag(M6502Flags::N, (cpu.temp & TOP_BIT_THRESH) != 0);

        // Load the result into the accumulator (it's 8-bit dont forget!)
        cpu.acc = ((cpu.temp as u16) & LOW_BYTE) as u8;

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
    /// let cycles = M6502::instructions::and(&mut cpu, &mut bus);
    ///
    /// assert_eq!(cycles, 1);
    /// assert_eq!(cpu.acc, 0x12 & 0x34);
    /// assert_eq!(cpu.pc, 0x1235);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::Z), false);
    /// assert_eq!(cpu.get_flag(M6502::M6502Flags::N), true);
    /// ```
    fn and(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.acc &= cpu.fetch(bus);
        cpu.set_flag(M6502Flags::Z, cpu.acc == 0x00);
        cpu.set_flag(M6502Flags::N, (cpu.acc & TOP_BIT_THRESH as u8) != 0);
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
    fn asl(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.temp = (cpu.fetch(bus) << 1).into();
        cpu.set_flag(M6502Flags::C, (cpu.temp & HIGH_BYTE) > 0);
        cpu.set_flag(M6502Flags::Z, (cpu.temp & LOW_BYTE) == 0);
        cpu.set_flag(M6502Flags::N, (cpu.temp & TOP_BIT_THRESH) != 0);
        if LOOKUP_TABLE[cpu.opcode as usize].2 as usize == M6502::imp as usize {
            cpu.acc = (cpu.temp & LOW_BYTE) as u8;
        } else {
            bus.write(cpu.addr_abs, (cpu.temp & LOW_BYTE) as u8);
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
    /// * `cpu` - A mutable reference to the [`M6502`] CPU
    /// * `_` - A mutable reference to the [`Bus`]. This argument is ignored by this function.
    ///
    /// # Returns
    ///
    /// The number of cycles that the instruction has consumed, which is always 0.
    #[inline]
    fn bcc(cpu: &mut M6502, _: &mut Bus) -> u8 {
        if cpu.get_flag(M6502Flags::C) == 0_u8 {
            cpu.cycles += 1_u8;
            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if cpu.addr_abs & HIGH_BYTE != cpu.pc & HIGH_BYTE {
                cpu.cycles += 1_u8;
            }
            cpu.pc = cpu.addr_abs;
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
    fn bcs(cpu: &mut M6502, _: &mut Bus) -> u8 {
        if cpu.get_flag(M6502Flags::C) == 1_u8 {
            cpu.cycles += 1_u8;
            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if cpu.addr_abs & HIGH_BYTE != cpu.pc & HIGH_BYTE {
                cpu.cycles += 1_u8;
            }
            cpu.pc = cpu.addr_abs;
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
    fn beq(cpu: &mut M6502, _: &mut Bus) -> u8 {
        if cpu.get_flag(M6502Flags::Z) == 1_u8 {
            cpu.cycles += 1_u8;
            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if cpu.addr_abs & HIGH_BYTE != cpu.pc & HIGH_BYTE {
                cpu.cycles += 1_u8;
            }
            cpu.pc = cpu.addr_abs;
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
    fn bit(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.temp = (cpu.acc & cpu.fetch(bus)) as u16;
        cpu.set_flag(M6502Flags::Z, (cpu.temp & LOW_BYTE) == 0x00);
        cpu.set_flag(M6502Flags::N, (cpu.fetched & (1 << 7)) != 0);
        cpu.set_flag(M6502Flags::V, (cpu.fetched & (1 << 6)) != 0);
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
    fn bmi(cpu: &mut M6502, _: &mut Bus) -> u8 {
        if cpu.get_flag(M6502Flags::N) == 1_u8 {
            cpu.cycles += 1_u8;
            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if cpu.addr_abs & HIGH_BYTE != cpu.pc & HIGH_BYTE {
                cpu.cycles += 1_u8;
            }
            cpu.pc = cpu.addr_abs;
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
    fn bne(cpu: &mut M6502, _: &mut Bus) -> u8 {
        if cpu.get_flag(M6502Flags::Z) == 0_u8 {
            cpu.cycles += 1_u8;
            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if cpu.addr_abs & HIGH_BYTE != cpu.pc & HIGH_BYTE {
                cpu.cycles += 1_u8;
            }
            cpu.pc = cpu.addr_abs;
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
    fn bpl(cpu: &mut M6502, _: &mut Bus) -> u8 {
        if cpu.get_flag(M6502Flags::N) == 0 {
            cpu.cycles += 1;
            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if cpu.addr_abs & HIGH_BYTE != cpu.pc & HIGH_BYTE {
                cpu.cycles += 1;
            }
            cpu.pc = cpu.addr_abs;
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
    fn brk(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.pc += 1;

        cpu.set_flag(M6502Flags::I, true);
        bus.write(
            (0x0100_u16 + cpu.stkp as u16).into(),
            (cpu.pc >> 8 & LOW_BYTE) as u8,
        );
        cpu.stkp -= 1;
        bus.write(
            (0x0100_u16 + cpu.stkp as u16).into(),
            (cpu.pc & LOW_BYTE) as u8,
        );
        cpu.stkp -= 1;

        cpu.set_flag(M6502Flags::B, true);
        bus.write((0x0100_u16 + cpu.stkp as u16).into(), cpu.status);
        cpu.stkp -= 1;
        cpu.set_flag(M6502Flags::B, true);

        cpu.pc = ((bus.read(0xFFFE, false) != 0x0u8) | (bus.read(0xFFFF, false) != 0x0u8)).into();
        0x0u8
    }

    #[inline]
    fn bvc(cpu: &mut M6502, _: &mut Bus) -> u8 {
        if cpu.get_flag(M6502Flags::V) == 0u8 {
            cpu.cycles += 1;
            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if cpu.addr_abs & HIGH_BYTE != cpu.pc & HIGH_BYTE {
                cpu.cycles += 1;
            }
            cpu.pc = cpu.addr_abs;
        }
        0x0u8
    }

    #[inline]
    fn bvs(cpu: &mut M6502, _: &mut Bus) -> u8 {
        if cpu.get_flag(M6502Flags::V) == 1u8 {
            cpu.cycles += 1;
            cpu.addr_abs = cpu.pc + cpu.addr_rel;

            if cpu.addr_abs & HIGH_BYTE != cpu.pc & HIGH_BYTE {
                cpu.cycles += 1;
            }
            cpu.pc = cpu.addr_abs;
        }
        0x0u8
    }

    #[inline]
    fn clc(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.set_flag(M6502Flags::C, false);
        0x0u8
    }

    #[inline]
    fn cld(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.set_flag(M6502Flags::D, false);
        0u8
    }

    #[inline]
    fn cli(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.set_flag(M6502Flags::I, false);
        0u8
    }

    #[inline]
    fn clv(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.set_flag(M6502Flags::V, false);
        0u8
    }

    #[inline]
    fn cmp(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.temp = (cpu.acc - cpu.fetch(bus)).into();
        cpu.set_flag(M6502Flags::C, cpu.acc >= cpu.fetched);
        cpu.set_flag(M6502Flags::Z, cpu.temp & LOW_BYTE == 0x0000);
        cpu.set_flag(M6502Flags::N, cpu.temp & TOP_BIT_THRESH != 0x0000);
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
    fn cpx(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.temp = (cpu.x - cpu.fetch(bus)).into();
        cpu.set_flag(M6502Flags::C, cpu.x >= cpu.fetched);
        cpu.set_flag(M6502Flags::Z, cpu.temp & LOW_BYTE == 0x0000);
        cpu.set_flag(M6502Flags::N, cpu.temp & TOP_BIT_THRESH != 0x0000);
        0u8
    }

    #[inline]
    fn cpy(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.temp = (cpu.y - cpu.fetch(bus)).into();
        cpu.set_flag(M6502Flags::C, cpu.y >= cpu.fetched);
        cpu.set_flag(M6502Flags::Z, cpu.temp & LOW_BYTE == 0x0000);
        cpu.set_flag(M6502Flags::N, cpu.temp & TOP_BIT_THRESH != 0x0000);
        0u8
    }

    #[inline]
    fn dec(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.temp = cpu.fetch(bus) as u16 - 1;
        bus.write(cpu.addr_abs, (cpu.temp & LOW_BYTE) as u8);
        cpu.set_flag(M6502Flags::Z, cpu.temp & LOW_BYTE == 0x0000);
        cpu.set_flag(M6502Flags::N, cpu.temp & TOP_BIT_THRESH != 0x0000);
        0u8
    }

    #[inline]
    fn dex(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.x -= 1;
        cpu.set_flag(M6502Flags::Z, cpu.x == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.x & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline]
    fn dey(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.y -= 1;
        cpu.set_flag(M6502Flags::Z, cpu.y == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.y & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline]
    fn eor(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.acc ^= cpu.fetch(bus);
        cpu.set_flag(M6502Flags::Z, cpu.y == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.y & TOP_BIT_THRESH as u8 != 0x0000);
        1u8
    }

    #[inline]
    fn inc(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.temp = cpu.fetch(bus) as u16 + 1;
        bus.write(cpu.addr_abs, (cpu.temp & LOW_BYTE) as u8);
        cpu.set_flag(M6502Flags::Z, cpu.temp & LOW_BYTE == 0x0000);
        cpu.set_flag(M6502Flags::N, cpu.temp & TOP_BIT_THRESH != 0x0000);
        0u8
    }

    #[inline]
    fn inx(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.x += 1;
        cpu.set_flag(M6502Flags::Z, cpu.x == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.x & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline]
    fn iny(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.y += 1;
        cpu.set_flag(M6502Flags::Z, cpu.y == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.y & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline]
    fn jmp(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.pc = cpu.addr_abs;
        0u8
    }

    #[inline]
    fn jsr(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.pc -= 1;

        bus.write(0x0100 + cpu.stkp as u16, (cpu.pc << 8 & LOW_BYTE) as u8);
        cpu.stkp -= 1;
        bus.write(0x0100 + cpu.stkp as u16, (cpu.pc & LOW_BYTE) as u8);
        cpu.stkp -= 1;

        cpu.pc = cpu.addr_abs;
        0u8
    }

    #[inline]
    fn lda(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.acc = cpu.fetch(bus);
        cpu.set_flag(M6502Flags::Z, cpu.acc == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.acc & TOP_BIT_THRESH as u8 != 0x00);
        1u8
    }

    #[inline]
    fn ldx(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.x = cpu.fetch(bus);
        cpu.set_flag(M6502Flags::Z, cpu.x == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.x & TOP_BIT_THRESH as u8 != 0x00);
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
    /// * `cpu` - A mutable reference to the [`M6502`] CPU
    /// * `bus` - A mutable reference to the system [`Bus`]
    ///
    /// # Returns
    ///
    /// This function returns the number of clock cycles used by the instruction.
    ///
    #[inline]
    fn ldy(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.y = cpu.fetch(bus);
        cpu.set_flag(M6502Flags::Z, cpu.y == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.y & TOP_BIT_THRESH as u8 != 0x00);
        1u8
    }

    #[inline]
    fn lsr(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.temp = (cpu.fetch(bus) >> 1) as u16;
        cpu.set_flag(M6502Flags::C, cpu.fetched & 0x0001 != 0x0000);
        cpu.set_flag(M6502Flags::Z, cpu.temp & LOW_BYTE == 0x0000);
        cpu.set_flag(M6502Flags::N, cpu.temp & TOP_BIT_THRESH != 0x0000);
        if LOOKUP_TABLE[cpu.opcode as usize].2 as usize == M6502::imp as usize {
            cpu.acc = cpu.temp as u8 & LOW_BYTE as u8;
        } else {
            bus.write(cpu.addr_abs, (cpu.temp & LOW_BYTE) as u8);
        }
        0u8
    }

    #[inline]
    fn nop(cpu: &mut M6502, _: &mut Bus) -> u8 {
        return match cpu.opcode {
            0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => 1u8,
            _ => 0u8,
        };
    }

    #[inline]
    fn ora(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.acc |= cpu.fetch(bus);
        cpu.set_flag(M6502Flags::Z, cpu.acc == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.acc & TOP_BIT_THRESH as u8 != 0x00);
        1u8
    }

    #[inline]
    fn pha(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        bus.write(0x0100 + cpu.stkp as u16, cpu.acc);
        cpu.stkp -= 1;
        0u8
    }

    #[inline]
    fn php(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        bus.write(
            0x0100 + cpu.stkp as u16,
            cpu.status | M6502Flags::B as u8 | M6502Flags::U as u8,
        );
        cpu.set_flag(M6502Flags::B, false);
        cpu.set_flag(M6502Flags::U, false);
        cpu.stkp -= 1;
        0u8
    }

    #[inline]
    fn pla(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.stkp += 1;
        cpu.status = bus.read(0x0100 + cpu.stkp as u16, false);
        cpu.set_flag(M6502Flags::Z, cpu.acc == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.acc & TOP_BIT_THRESH as u8 == 0x00);
        0u8
    }

    #[inline]
    fn plp(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.stkp += 1;
        cpu.status = bus.read(0x0100 + cpu.stkp as u16, false);
        cpu.set_flag(M6502Flags::U, true);
        0u8
    }

    #[inline]
    fn rol(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.temp = (cpu.fetch(bus) << 1 | cpu.get_flag(M6502Flags::C)).into();
        cpu.set_flag(M6502Flags::C, cpu.temp & HIGH_BYTE != 0x0000);
        cpu.set_flag(M6502Flags::Z, cpu.temp & LOW_BYTE == 0x0000);
        cpu.set_flag(M6502Flags::N, cpu.temp & TOP_BIT_THRESH != 0x0000);
        if LOOKUP_TABLE[cpu.opcode as usize].2 as usize == M6502::imp as usize {
            cpu.acc = (cpu.temp & LOW_BYTE) as u8;
        } else {
            bus.write(cpu.addr_abs, (cpu.temp & LOW_BYTE) as u8);
        }
        0u8
    }

    #[inline]
    fn ror(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.temp = (cpu.get_flag(M6502Flags::C) << 7 | cpu.fetch(bus) >> 1).into();
        cpu.set_flag(M6502Flags::C, cpu.fetched & 0x01 == 0x00);
        cpu.set_flag(M6502Flags::Z, cpu.temp & LOW_BYTE == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.temp & TOP_BIT_THRESH != 0x00);
        if LOOKUP_TABLE[cpu.opcode as usize].2 as usize == M6502::imp as usize {
            cpu.acc = (cpu.temp & LOW_BYTE) as u8;
        } else {
            bus.write(cpu.addr_abs, (cpu.temp & LOW_BYTE) as u8);
        }
        0u8
    }

    #[inline]
    fn rti(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.stkp += 1;
        cpu.status = bus.read(0x0100 + cpu.stkp as u16, false);
        cpu.status &= !(M6502Flags::B as u8);
        cpu.status &= !(M6502Flags::U as u8);

        cpu.stkp += 1;
        cpu.pc = bus.read(0x0100 + cpu.stkp as u16, false).into();
        cpu.stkp += 1;
        cpu.pc |= (bus.read(0x0100 + cpu.stkp as u16, false) as u16) << 8;
        0u8
    }

    #[inline]
    fn rts(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.stkp += 1;
        cpu.pc = bus.read(0x0100 + cpu.stkp as u16, false).into();
        cpu.stkp += 1;
        cpu.pc |= (bus.read(0x0100 + cpu.stkp as u16, false) as u16) << 8;

        cpu.pc += 1;
        0u8
    }

    #[inline]
    fn sbc(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        let value: u16 = cpu.fetch(bus) as u16 ^ LOW_BYTE;
        cpu.temp = cpu.acc as u16 + value + cpu.get_flag(M6502Flags::C) as u16;
        cpu.set_flag(M6502Flags::C, cpu.temp & HIGH_BYTE != 0x0000);
        cpu.set_flag(M6502Flags::Z, cpu.temp & HIGH_BYTE == 0x0000);
        cpu.set_flag(
            M6502Flags::V,
            (cpu.temp ^ cpu.acc as u16) & (cpu.temp ^ value) & TOP_BIT_THRESH != 0x0000,
        );
        cpu.set_flag(M6502Flags::N, cpu.temp & TOP_BIT_THRESH == 0x0000);
        cpu.acc = cpu.temp as u8 & LOW_BYTE as u8;
        1u8
    }

    #[inline]
    fn sec(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.set_flag(M6502Flags::C, true);
        0u8
    }

    #[inline]
    fn sed(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.set_flag(M6502Flags::D, true);
        0u8
    }

    #[inline]
    fn sei(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.set_flag(M6502Flags::I, true);
        0u8
    }

    #[inline]
    fn sta(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        bus.write(cpu.addr_abs, cpu.acc);
        0u8
    }

    #[inline]
    fn stx(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        bus.write(cpu.addr_abs, cpu.x);
        0u8
    }

    #[inline]
    fn sty(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        bus.write(cpu.addr_abs, cpu.y);
        0u8
    }

    #[inline]
    fn tax(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.x = cpu.acc;
        cpu.set_flag(M6502Flags::Z, cpu.x == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.x & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline]
    fn tay(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.y = cpu.acc;
        cpu.set_flag(M6502Flags::Z, cpu.y == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.y & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline]
    fn tsx(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.x = cpu.stkp;
        cpu.set_flag(M6502Flags::Z, cpu.x == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.x & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline]
    fn txa(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.acc = cpu.x;
        cpu.set_flag(M6502Flags::Z, cpu.acc == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.acc & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline(always)]
    fn txs(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.stkp = cpu.x;
        0u8
    }

    #[inline]
    fn tya(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.acc = cpu.y;
        cpu.set_flag(M6502Flags::Z, cpu.acc == 0x00);
        cpu.set_flag(M6502Flags::N, cpu.acc & TOP_BIT_THRESH as u8 != 0x0000);
        0u8
    }

    #[inline(always)]
    fn xxx(_: &mut M6502, _: &mut Bus) -> u8 {
        0u8
    }
}

impl M6502AddrModes for M6502 {
    fn imp(cpu: &mut M6502, _: &mut Bus) -> u8 {
        cpu.fetched = cpu.acc;
        0x00
    }

    fn imm(cpu: &mut M6502, _bus: &mut Bus) -> u8 {
        cpu.addr_abs = cpu.pc;
        0x00
    }

    fn zp0(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.addr_abs = bus.read(cpu.pc, false) as u16;
        cpu.pc += 1;
        cpu.addr_abs &= LOW_BYTE; // checking if high bit is on a new page
        0x00
    }

    fn zpx(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.addr_abs = bus.read(cpu.pc + cpu.x as u16, false) as u16;
        cpu.pc += 1;
        cpu.addr_abs &= LOW_BYTE;
        0x00
    }

    fn zpy(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.addr_abs = bus.read(cpu.pc + cpu.y as u16, false) as u16;
        cpu.pc += 1;
        cpu.addr_abs &= LOW_BYTE;
        0x00
    }

    fn abs(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        let lo: u32 = bus.read(cpu.pc as u16, false).into();
        cpu.pc += 1;
        let hi: u32 = bus.read(cpu.pc as u16, false).into();
        cpu.pc += 1;
        cpu.addr_abs = ((hi << 8) | lo) as u16;
        0x00
    }

    fn abx(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        let lo: u32 = bus.read(cpu.pc as u16, false).into();
        cpu.pc += 1;
        let hi: u32 = bus.read(cpu.pc as u16, false).into();
        cpu.pc += 1;
        cpu.addr_abs = ((hi << 8) | lo) as u16;
        cpu.addr_abs += cpu.x as u16;

        return if (cpu.addr_abs & LOW_BYTE) != (hi << 8) as u16 {
            0x01
        } else {
            0x00
        };
    }

    fn aby(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        let lo: u16 = bus.read(cpu.pc as u16, false).into();
        cpu.pc += 1;
        let hi: u16 = bus.read(cpu.pc as u16, false).into();
        cpu.pc += 1;
        cpu.addr_abs = ((hi << 8) | lo) as u16;
        cpu.addr_abs += cpu.y as u16;

        return if (cpu.addr_abs & LOW_BYTE) != (hi << 8) as u16 {
            0x01
        } else {
            0x00
        };
    }

    fn rel(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        cpu.addr_rel = bus.read(cpu.pc, false) as u16;
        cpu.pc += 1;
        if (cpu.addr_rel & 0x08) != 0 {
            cpu.addr_abs |= LOW_BYTE;
        }
        0x00
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
    ///```
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
    fn ind(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        let pointer_lo = bus.read(cpu.pc, false) as u16;
        cpu.pc += 1;
        let pointer_hi = bus.read(cpu.pc as u16, false) as u16;
        cpu.pc += 1;

        let ptr: u16 = (pointer_hi << 8u16) | pointer_lo;

        let lo: u32;
        let hi: u32;
        if pointer_lo == LOW_BYTE {
            lo = (bus.read(ptr & LOW_BYTE, false) as u32) << 8;
            hi = bus.read(ptr + 0, false).into();
            cpu.addr_abs = (lo | hi) as u16;
        } else {
            lo = (bus.read(ptr + 1, false) as u32) << 8;
            hi = bus.read(ptr + 0, false).into();
            cpu.addr_abs = (lo | hi) as u16;
        }
        0x00
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
    /// * `cpu` - A mutable reference to the [`M6502`] struct representing the CPU
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
    /// M6502::izx(&mut cpu, &mut bus);
    /// assert_eq!(cpu.pc, 0x0001);
    /// assert_eq!(cpu.addr_abs, 0x4205);
    ///
    /// let result = bus.read(cpu.addr_abs, true);
    /// assert_eq!(result, 0x42);
    /// ```
    fn izx(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        let t: u8 = bus.read(cpu.pc, false);
        cpu.pc += 1;

        let lo: u32 = bus.read((t + cpu.x) as u16 & LOW_BYTE, false).into();
        let hi: u32 = bus.read((t + cpu.x + 1) as u16 & LOW_BYTE, false).into();

        cpu.addr_abs = ((hi << 8u8) | lo << 8u8) as u16 >> 8u16;
        0x00
    }

    /// Indirect Indexed with Y Addressing Mode
    ///
    /// This addressing mode is used by certain instructions to access memory
    /// indirectly, using a zero page address that is added to the Y register.
    ///
    /// # Arguments
    ///
    /// * `cpu` - A mutable reference to the [`M6502`] struct representing the CPU
    /// * `bus` - A mutable reference to the [`Bus`] struct representing the system bus
    ///
    /// # Returns
    ///
    /// The result of the operation, which is either 0 or 1 depending on whether
    /// the operation resulted in a page boundary crossing.
    fn izy(cpu: &mut M6502, bus: &mut Bus) -> u8 {
        let t = bus.read(cpu.pc, false);
        cpu.pc += 1;

        let lo = bus.read((t + cpu.y) as u16 & LOW_BYTE, false);
        let hi = bus.read((t + cpu.y + 1) as u16 & LOW_BYTE, false);

        cpu.addr_abs = (((hi as u16) << 8u16) | (lo as u16) << 8u16) as u16;
        cpu.addr_abs += cpu.y as u16;

        return if (cpu.addr_abs & HIGH_BYTE) != ((hi as u16) << 8u8) as u16 {
            0x01
        } else {
            0x00
        };
    }
}
