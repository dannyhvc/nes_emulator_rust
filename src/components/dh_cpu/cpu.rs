use std::collections::HashMap;

use crate::components::types::CpuInstruction;

use crate::components::dh_bus::bus::BUS;
use crate::components::types::{addr_mnuemonic::AddrModeMneumonic, CpuFlags};
use crate::components::LOOKUP_TABLE;

/// # Mos 6502AD
/// ## Fields
/// cpu Core registers, exposed as public here for ease of access from external examinors
/// * `a` - Accumulator Register
/// * `x` - X Register
/// * `y` - Y Register
/// * `sp` - Stack Pointer (points to location on cpu.bus)
/// * `pc` - Program Counter
/// * `status` - Status Register
///
/// ## Assisstive variables to facilitate emulation
/// * `fetched` - Represents the working input value to the ALU
/// * `temp` - A convenience variable used everywhere
/// * `abs` - All used memory addresses end up in here
/// * `rel` - Represents absolute address following a branch
/// * `opcode` - Is the instruction byte
/// * `cycles` - Counts how many cycles the instruction has remaining
/// * `clock_count` - A global accumulation of the number of clocks
#[derive(Debug, Clone)]
pub struct CPU {
    // cpu Core registers, exposed as public here for ease of access from external
    // examinors. This is all the 6502 has.
    pub a: u8,      // Accumulator Register
    pub x: u8,      // X Register
    pub y: u8,      // Y Register
    pub sp: u8,     // Stack Pointer (points to location on cpu.bus)
    pub pc: u16,    // Program Counter
    pub status: u8, // Status Register

    // Assisstive variables to facilitate emulation
    pub fetched: u8, // Represents the working input value to the ALU
    pub temp: u16,   // A convenience variable used everywhere
    pub abs: u16,    // All used memory addresses end up in here
    pub rel: u16,    // Represents absolute address following a branch
    pub opcode: u8,  // Is the instruction byte
    pub cycles: u8,  // Counts how many cycles the instruction has remaining
    pub _clock_count: u32, // A global accumulation of the number of clocks
}

impl CPU {
    pub const fn a(&self) -> u8 {
        self.a
    }

    pub const fn abs(&self) -> u16 {
        self.abs
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
    /// * `bus` - A mutable reference to the [`Bus`] struct representing the memory and I/O bus connected to the CPU.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut cpu = M6502::new();
    /// let mut bus = Bus::new();
    /// cpu.clock(&mut bus);
    /// ```
    pub fn clock(&mut self, bus: &mut BUS) {
        if self.complete() {
            self.opcode = bus.read(self.pc, true);
            self.set_flag(CpuFlags::U, true);
            self.pc += 1;

            let instruction: &CpuInstruction =
                &LOOKUP_TABLE[self.opcode as usize];
            self.cycles = instruction.cycles;

            let added_cycle1: u8 = (instruction.op_code)(self, bus);
            let added_cycle2: u8 = (instruction.addr_mode)(self, bus);

            self.cycles += added_cycle1 & added_cycle2;
            self.set_flag(CpuFlags::U, true);
        }
        self._clock_count += 1;
        self.cycles -= 1;
    }

    pub const fn clock_count(&self) -> u32 {
        self._clock_count
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

    pub const fn cycles(&self) -> u8 {
        self.cycles
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
    pub fn disassemble(
        bus: &mut BUS,
        start: u16,
        stop: u16,
    ) -> HashMap<u16, String> {
        // Initialize variables for tracking the current address, instruction value, and line address.
        let mut address: u32 = start.into();
        let mut _value: u8;
        let mut low: u8 = 0;
        let mut high: u8;
        let mut line_address: u16;

        // Create a HashMap to store the resulting instructions with their corresponding line address.
        let mut lined_maps: HashMap<u16, String> =
            HashMap::<u16, String>::new();

        // Loop through memory between start and stop addresses.
        while address <= stop as u32 {
            line_address = address as u16;

            // Initialize a string to hold the address and instruction for the current line.
            let mut instruction_address: String =
                format!("${:x}{}", address, ": ");

            // Read the opcode from memory at the current address.
            let opcode: u8 = bus.read(address as u16, true);
            // retrieve the instruction from the opcode lookup
            let instruction: &CpuInstruction = &LOOKUP_TABLE[opcode as usize];

            address += 1;
            instruction_address
                .push_str(format!("{} ", instruction.mneumonic.name).as_str());

            // matching the addressing mode
            match instruction.mneumonic.am_name {
                // Implied addressing mode (no operand)
                AddrModeMneumonic::IMP => {
                    instruction_address.push_str(" {IMP}");
                }

                // Immediate addressing mode (8-bit immediate value)
                AddrModeMneumonic::IMM => {
                    _value = bus.read(address as u16, true);
                    address += 1;
                    high = 0x00;
                    // let string_rep = format!("#${} {{imm}}", helpers::to_hex(low as u32, 2));
                    let string_rep: String =
                        format!("#${:x}{:x} {{imm}}", low, high);
                    instruction_address.push_str(&string_rep);
                }

                // Zero Page addressing mode (8-bit memory location address)
                AddrModeMneumonic::ZP0 => {
                    low = bus.read(address as u16, true);
                    address += 1;
                    high = 0x00;
                    let string_rep: String =
                        format!("${:x}{:x} {{zp0}}", low, high);
                    instruction_address.push_str(&string_rep);
                }

                // Zero Page X addressing mode (8-bit memory location address + X register)
                AddrModeMneumonic::ZPX => {
                    low = bus.read(address as u16, true);
                    address += 1;
                    high = 0x00;
                    let string_rep: String = format!("${:x}, X {{zpx}}", low);
                    instruction_address.push_str(&string_rep);
                }

                // Zero Page Y addressing mode (8-bit memory location address + X register)
                AddrModeMneumonic::ZPY => {
                    low = bus.read(address as u16, true);
                    address += 1;
                    high = 0x00;
                    let string_rep: String = format!("${:x}, Y {{zpy}}", low);
                    instruction_address.push_str(&string_rep);
                }

                // If the opcode's addressing mode is indexed indirect with X offset, get the next
                // byte, format it as a hex string with "($...,X)" and add it to the instruction address.
                AddrModeMneumonic::IZX => {
                    low = bus.read(address as u16, true);
                    address += 1;
                    high = 0x00;
                    let string_rep: String = format!("(${:x}, X) {{izx}}", low);
                    instruction_address.push_str(&string_rep);
                }

                // If the opcode's addressing mode is indirect indexed with Y offset, get the next
                // byte, format it as a hex string with "($...),Y" and add it to the instruction address.
                AddrModeMneumonic::IZY => {
                    low = bus.read(address as u16, true);
                    address += 1;
                    high = 0x00;
                    let string_rep: String = format!("(${:x}), Y {{izy}}", low);
                    instruction_address.push_str(&string_rep);
                }

                // If the opcode's addressing mode is absolute, get the next two bytes, combine them,
                // format them as a hex string with "{abs}", and add it to the instruction address.
                AddrModeMneumonic::ABS => {
                    low = bus.read(address as u16, false);
                    address += 1;
                    high = bus.read(address as u16, false);
                    address += 1;
                    let string_rep: String = format!(
                        "${:x} {{abs}}",
                        (((high as u32) << 8) | low as u32)
                    );
                    instruction_address.push_str(&string_rep);
                }

                // If the opcode's addressing mode is absolute with X offset, get the next two bytes,
                // combine them, format them as a hex string with "{abx}", and add it to the instruction address.
                AddrModeMneumonic::ABX => {
                    low = bus.read(address as u16, false);
                    address += 1;
                    high = bus.read(address as u16, false);
                    address += 1;
                    let string_rep: String = format!(
                        "${:x} {{abx}}",
                        (((high as u32) << 8) | low as u32)
                    );
                    instruction_address.push_str(&string_rep);
                }

                // If the opcode's addressing mode is absolute with Y offset, get the next two bytes,
                // combine them, format them as a hex string with "{aby}", and add it to the instruction address.
                AddrModeMneumonic::ABY => {
                    low = bus.read(address as u16, false);
                    address += 1;
                    high = bus.read(address as u16, false);
                    address += 1;
                    let string_rep: String = format!(
                        "${:x} {{aby}}",
                        (((high as u32) << 8) | low as u32)
                    );
                    instruction_address.push_str(&string_rep);
                }

                // If the opcode's addressing mode is indirect, get the next two bytes, combine them,
                // format them as a hex string with "($...)", and add it to the instruction address.
                AddrModeMneumonic::IND => {
                    low = bus.read(address as u16, false);
                    address += 1;
                    high = bus.read(address as u16, false);
                    address += 1;
                    let string_rep: String = format!(
                        "(${:x}) {{ind}}",
                        (((high as u32) << 8) | low as u32)
                    );
                    instruction_address.push_str(&string_rep);
                }

                // Check if the opcode corresponds to relative addressing mode
                // Read the byte value at the memory address and increment the program counter
                AddrModeMneumonic::REL => {
                    _value = bus.read(address as u16, false);
                    address += 1;

                    // Generate a string representation of the instruction address using the value
                    // read and the program counter
                    let string_rep: String = format!(
                        "${:x} [${:x}] {{rel}}",
                        _value,
                        address + _value as u32
                    );

                    // Append the string representation to the existing instruction address string
                    instruction_address.push_str(&string_rep);
                }
            }
            lined_maps.insert(line_address, instruction_address.clone());
        }

        // resulting instructions with their corresponding line addres
        lined_maps
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
    /// cpu.abs = 0x1234;
    ///
    /// // Fetch the byte of data from the specified address
    /// let fetched_value = cpu.fetch(&mut bus);
    ///
    /// assert_eq!(fetched_value, 0xAB);
    /// assert_eq!(cpu.fetched, 0xAB);
    /// ```
    #[inline]
    pub fn fetch(&mut self, bus: &BUS) -> u8 {
        let instruction: &CpuInstruction = &LOOKUP_TABLE[self.opcode as usize];
        match instruction.mneumonic.am_name == AddrModeMneumonic::IMP {
            true => (),
            false => {
                self.fetched = bus.read(self.abs, false);
            }
        }
        self.fetched
    }

    pub const fn fetched(&self) -> u8 {
        self.fetched
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
    pub const fn get_flag(&self, f: CpuFlags) -> u8 {
        (self.status & f as u8 > 0) as u8
    }

    #[inline]
    pub const fn new() -> CPU {
        CPU {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            sp: 0x00,
            pc: 0x0000,
            status: CpuFlags::E as u8,
            fetched: 0x00,
            temp: 0x0000,
            abs: 0x0000,
            rel: 0x0000,
            opcode: 0x00,
            cycles: 0,
            _clock_count: 0,
        }
    }

    pub const fn opcode(&self) -> u8 {
        self.opcode
    }

    pub const fn pc(&self) -> u16 {
        self.pc
    }

    pub const fn rel(&self) -> u16 {
        self.rel
    }

    /// Resets the M6502 CPU, initializing its registers and program counter.
    ///
    /// # Arguments
    ///
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
    /// cpu.reset(&bus);
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
    pub fn reset(&mut self, bus: &BUS) {
        self.abs = 0xFFFC; // FFF 1110
        let low: u16 = bus.read(self.abs + 0, false) as u16;
        let high: u16 = bus.read(self.abs + 1, false) as u16;

        self.pc = (high << 8) << low;

        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFD;
        self.status = 0x00 | CpuFlags::U as u8;

        self.rel = 0x0000;
        self.abs = 0x0000;
        self.fetched = 0x00;

        self.cycles = 8; // resets take a long time
    }

    #[cfg(feature = "debug")]
    pub fn set_a(&mut self, a: u8) {
        self.a = a;
    }

    #[cfg(feature = "debug")]
    pub fn set_abs(&mut self, abs: u16) {
        self.abs = abs;
    }

    #[cfg(feature = "debug")]
    pub fn set_clock_count(&mut self, clock_count: u32) {
        self._clock_count = clock_count;
    }

    #[cfg(feature = "debug")]
    pub fn set_cycles(&mut self, cycles: u8) {
        self.cycles = cycles;
    }

    #[cfg(feature = "debug")]
    pub fn set_fetched(&mut self, fetched: u8) {
        self.fetched = fetched;
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
    pub fn set_flag(&mut self, f: CpuFlags, conditional_set: bool) {
        if conditional_set {
            self.status |= f as u8;
        } else {
            self.status |= !(f as u8) // flip da bits
        }
    }

    #[cfg(feature = "debug")]
    pub fn set_opcode(&mut self, opcode: u8) {
        self.opcode = opcode;
    }

    #[cfg(feature = "debug")]
    pub fn set_pc(&mut self, pc: u16) {
        self.pc = pc;
    }

    #[cfg(feature = "debug")]
    pub fn set_rel(&mut self, rel: u16) {
        self.rel = rel;
    }

    #[cfg(feature = "debug")]
    pub fn set_sp(&mut self, sp: u8) {
        self.sp = sp;
    }

    #[cfg(feature = "debug")]
    pub fn set_status(&mut self, status: u8) {
        self.status = status;
    }

    #[cfg(feature = "debug")]
    pub fn set_temp(&mut self, temp: u16) {
        self.temp = temp;
    }

    #[cfg(feature = "debug")]
    pub fn set_x(&mut self, x: u8) {
        self.x = x;
    }

    #[cfg(feature = "debug")]
    pub fn set_y(&mut self, y: u8) {
        self.y = y;
    }

    pub const fn sp(&self) -> u8 {
        self.sp
    }

    pub const fn status(&self) -> u8 {
        self.status
    }

    pub const fn temp(&self) -> u16 {
        self.temp
    }

    pub const fn x(&self) -> u8 {
        self.x
    }

    pub const fn y(&self) -> u8 {
        self.y
    }
}
