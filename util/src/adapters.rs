type OpFn<CPU, BUS> = for<'a, 'b> fn(&'a mut CPU, &'b mut BUS) -> u8;

///
pub fn opcode_adapter<CPU, BUS>(
    op: &OpFn<CPU, BUS>,
    cpu: &mut CPU,
    bus: &mut BUS,
) {
    let ret_val = (*op)(cpu, bus);
}
