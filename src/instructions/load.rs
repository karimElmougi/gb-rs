fn ld_a_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.a = cpu.fetch_byte(mmu);
    8
}

fn ld_b_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.b = cpu.fetch_byte(mmu);
    8
}

fn ld_c_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.c = cpu.fetch_byte(mmu);
    8
}

fn ld_d_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.d = cpu.fetch_byte(mmu);
    8
}

fn ld_e_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.e = cpu.fetch_byte(mmu);
    8
}

fn ld_h_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.h = cpu.fetch_byte(mmu);
    8
}

fn ld_l_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.l = cpu.fetch_byte(mmu);
    8
}

fn ld_a_nn(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let addr = cpu.fetch_word(mmu);
    cpu.regs.a = mmu.read_byte(addr);
    16
}

fn ld_bc_nn(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_word(mmu);
    cpu.regs.set_bc(n);
    12
}

fn ld_a_a(_: &mut CPU, _: &mut MMU) -> u8 {
    4
}

fn ld_a_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.a = cpu.regs.b;
    4
}

fn ld_a_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.a = cpu.regs.c;
    4
}

fn ld_a_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.a = cpu.regs.d;
    4
}

fn ld_a_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.a = cpu.regs.e;
    4
}

fn ld_a_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.a = cpu.regs.h;
    4
}

fn ld_a_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.a = cpu.regs.l;
    4
}

fn ld_b_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.b = cpu.regs.a;
    4
}

fn ld_b_b(_: &mut CPU, _: &mut MMU) -> u8 {
    4
}

fn ld_b_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.b = cpu.regs.c;
    4
}

fn ld_b_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.b = cpu.regs.d;
    4
}

fn ld_b_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.b = cpu.regs.e;
    4
}

fn ld_b_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.b = cpu.regs.h;
    4
}

fn ld_b_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.b = cpu.regs.l;
    4
}

fn ld_c_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.c = cpu.regs.a;
    4
}

fn ld_c_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.c = cpu.regs.b;
    4
}

fn ld_c_c(_: &mut CPU, _: &mut MMU) -> u8 {
    4
}

fn ld_c_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.c = cpu.regs.d;
    4
}

fn ld_c_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.c = cpu.regs.e;
    4
}

fn ld_c_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.c = cpu.regs.h;
    4
}

fn ld_c_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.c = cpu.regs.l;
    4
}

fn ld_d_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.d = cpu.regs.a;
    4
}

fn ld_d_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.d = cpu.regs.b;
    4
}

fn ld_d_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.d = cpu.regs.c;
    4
}

fn ld_d_d(_: &mut CPU, _: &mut MMU) -> u8 {
    4
}

fn ld_d_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.d = cpu.regs.e;
    4
}

fn ld_d_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.d = cpu.regs.h;
    4
}

fn ld_d_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.d = cpu.regs.l;
    4
}

fn ld_e_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.e = cpu.regs.a;
    4
}

fn ld_e_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.e = cpu.regs.b;
    4
}

fn ld_e_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.e = cpu.regs.c;
    4
}

fn ld_e_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.e = cpu.regs.d;
    4
}

fn ld_e_e(_: &mut CPU, _: &mut MMU) -> u8 {
    4
}

fn ld_e_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.e = cpu.regs.h;
    4
}

fn ld_e_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.e = cpu.regs.l;
    4
}

fn ld_h_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.h = cpu.regs.a;
    4
}

fn ld_h_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.h = cpu.regs.b;
    4
}

fn ld_h_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.h = cpu.regs.c;
    4
}

fn ld_h_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.h = cpu.regs.d;
    4
}

fn ld_h_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.h = cpu.regs.e;
    4
}

fn ld_h_h(_: &mut CPU, _: &mut MMU) -> u8 {
    4
}

fn ld_h_l(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.h = cpu.regs.l;
    4
}

fn ld_l_a(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.l = cpu.regs.a;
    4
}

fn ld_l_b(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.l = cpu.regs.b;
    4
}

fn ld_l_c(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.l = cpu.regs.c;
    4
}

fn ld_l_d(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.l = cpu.regs.d;
    4
}

fn ld_l_e(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.l = cpu.regs.e;
    4
}

fn ld_l_h(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.l = cpu.regs.h;
    4
}

fn ld_l_l(_: &mut CPU, _: &mut MMU) -> u8 {
    4
}

fn ld_a_bc(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.a = mmu.read_byte(cpu.regs.get_bc());
    8
}

fn ld_a_de(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.a = mmu.read_byte(cpu.regs.get_de());
    8
}

fn ld_a_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.a = mmu.read_byte(cpu.regs.get_hl());
    8
}

fn ld_b_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.b = mmu.read_byte(cpu.regs.get_hl());
    8
}

fn ld_c_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.c = mmu.read_byte(cpu.regs.get_hl());
    8
}

fn ld_d_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.d = mmu.read_byte(cpu.regs.get_hl());
    8
}

fn ld_e_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.e = mmu.read_byte(cpu.regs.get_hl());
    8
}

fn ld_h_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.h = mmu.read_byte(cpu.regs.get_hl());
    8
}

fn ld_l_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.l = mmu.read_byte(cpu.regs.get_hl());
    8
}

fn ld_de_nn(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_word(mmu);
    cpu.regs.set_de(n);
    12
}

fn ld_hl_nn(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_word(mmu);
    cpu.regs.set_hl(n);
    12
}

fn ld_sp_nn(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    cpu.regs.sp = cpu.fetch_word(mmu);
    12
}

fn ld_bc_a(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    mmu.write_byte(cpu.regs.get_bc(), cpu.regs.a);
    8
}

fn ld_de_a(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    mmu.write_byte(cpu.regs.get_de(), cpu.regs.a);
    8
}

fn ld_hl_a(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    mmu.write_byte(cpu.regs.get_hl(), cpu.regs.a);
    8
}

fn ld_hl_b(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    mmu.write_byte(cpu.regs.get_hl(), cpu.regs.b);
    8
}

fn ld_hl_c(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    mmu.write_byte(cpu.regs.get_hl(), cpu.regs.c);
    8
}

fn ld_hl_d(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    mmu.write_byte(cpu.regs.get_hl(), cpu.regs.d);
    8
}

fn ld_hl_e(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    mmu.write_byte(cpu.regs.get_hl(), cpu.regs.e);
    8
}

fn ld_hl_h(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    mmu.write_byte(cpu.regs.get_hl(), cpu.regs.h);
    8
}

fn ld_hl_l(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    mmu.write_byte(cpu.regs.get_hl(), cpu.regs.l);
    8
}

fn ld_n_a(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let addr = 0xff00 + (cpu.fetch_byte(mmu) as u16);
    mmu.write_byte(addr, cpu.regs.a);
    12
}

fn ld_nn_a(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let addr = cpu.fetch_word(mmu);
    mmu.write_byte(addr, cpu.regs.a);
    16
}

fn ld_nn_sp(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_word(mmu);
    mmu.write_word(n, cpu.regs.sp);
    20
}

fn ld_hl_sp_n(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    let n = cpu.fetch_byte(mmu);
    let sp_lo = (cpu.regs.sp & 0xff) as u8;
    let (_, f) = add(n, sp_lo);
    cpu.regs.f = f & !ZERO & !SUB;
    cpu.regs.set_hl(cpu.regs.sp + ((n as i8) as u16));
    12
}

fn ld_sp_hl(cpu: &mut CPU, _: &mut MMU) -> u8 {
    cpu.regs.sp = cpu.regs.get_hl();
    8
}

fn ldi_a_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    ld_a_hl(cpu, mmu);
    inc_hl(cpu, mmu);
    8
}

fn ldi_hl_a(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    ld_hl_a(cpu, mmu);
    inc_hl(cpu, mmu);
    8
}

fn ldd_a_hl(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    ld_a_hl(cpu, mmu);
    dec_hl(cpu, mmu);
    8
}

fn ldd_hl_a(cpu: &mut CPU, mmu: &mut MMU) -> u8 {
    ld_hl_a(cpu, mmu);
    dec_hl(cpu, mmu);
    8
}
