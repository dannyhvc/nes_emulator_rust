use crate::components::dh_cpu::cpu::CPU;

impl std::fmt::Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "a:      0x{:02X} ({})\n", self.a(), self.a())?;
        writeln!(f, "x:      0x{:02X} ({})\n", self.x(), self.x())?;
        writeln!(f, "y:      0x{:02X} ({})\n", self.y(), self.y())?;
        writeln!(f, "sp:     0x{:02X} ({})\n", self.sp(), self.sp())?;
        writeln!(f, "pc:     0x{:04X} ({})\n", self.pc(), self.pc())?;
        writeln!(f, "status: 0x{:02X} ({})\n", self.status(), self.status())?;
        writeln!(
            f,
            "fetched: 0x{:02X} ({})\n",
            self.fetched(),
            self.fetched()
        )?;
        writeln!(f, "temp:   0x{:04X} ({})\n", self.temp(), self.temp())?;
        writeln!(f, "abs:    0x{:04X} ({})\n", self.abs(), self.abs())?;
        writeln!(f, "rel:    0x{:04X} ({})\n", self.rel(), self.rel())?;
        writeln!(f, "opcode: 0x{:02X} ({})\n", self.opcode(), self.opcode())?;
        writeln!(f, "cycles: 0x{:02X} ({})\n", self.cycles(), self.cycles())?;
        writeln!(
            f,
            "_clock_count: 0x{:08X} ({})\n",
            self.clock_count(),
            self.clock_count()
        )?;
        Ok(())
    }
}
