#![allow(non_snake_case, non_camel_case_types)]

type OpFn<cpu_t, bus_t> = for<'a, 'b> fn(&'a mut cpu_t, &'b mut bus_t) -> u8;

///
pub fn opcode_adapter<cpu_t, bus_t>(
    op: &OpFn<cpu_t, bus_t>,
    cpu: &mut cpu_t,
    bus: &mut bus_t,
) {
    let ret_val = (*op)(cpu, bus);
}
