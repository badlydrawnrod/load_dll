use crate::block_finder::*;
use crate::read_instruction::*;
use arviss::{disassembler::Disassembler, Address, DispatchRv32ic, HandleRv32c, HandleRv32i};
use std::io::Write;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockWriterError {
    #[error("failed to read memory when compiling 0x{addr:08x}")]
    ReadFailed { addr: Address },

    #[error("block writer failed to write: {err}")]
    WriteFailed {
        #[from]
        err: std::io::Error,
    },
}

pub struct BlockWriter<'a> {
    mem: &'a [u8],
    dis: Disassembler,
    pc: Address,
    is_jump: bool,
}

impl<'a> BlockWriter<'a> {
    pub fn new(mem: &'a [u8]) -> Self {
        Self {
            mem,
            dis: Disassembler,
            pc: 0,
            is_jump: false,
        }
    }

    pub fn begin(&mut self, writer: &mut impl Write) -> Result<(), BlockWriterError> {
        writeln!(writer, "#![no_std]")?;
        writeln!(writer, "use arviss::HandleRv32i;")?;
        writeln!(writer, "use arviss::platforms::basic::*;")?;
        writeln!(writer, "use arviss::decoding::Reg;")?;
        writeln!(writer, "type Cpu = Rv32iCpu::<BasicMem>;")?;

        Ok(())
    }

    #[inline]
    fn instruction_at(&self, addr: Address) -> Result<u32, BlockWriterError> {
        read_instruction(self.mem, addr).map_err(|addr| BlockWriterError::ReadFailed { addr })
    }

    pub fn write_block(
        &mut self,
        writer: &mut impl Write,
        block: &Block,
    ) -> Result<(), BlockWriterError> {
        let mut addr = block.start;
        writeln!(writer, "\n#[no_mangle]")?;
        writeln!(
            writer,
            "pub extern \"C\" fn block_{:08x}_{:08x}(cpu: &mut Cpu) {{",
            block.start, block.end
        )?;
        while addr < block.end {
            self.is_jump = false;
            self.pc = addr;
            let ins = self.instruction_at(addr)?;

            // Disassemble it and compile it.
            let code = self.dis.dispatch(ins);
            let is_compact = (ins & 3) != 3;
            if is_compact {
                // Compact instructions are 2 bytes each.
                writeln!(writer, "// {:08x}     {:04x} {}", addr, ins & 0xffff, code)?;
                addr += 2;
            } else {
                // Regular instructions are 4 bytes each.
                writeln!(writer, "// {:08x} {:08x} {}", addr, ins, code)?;
                addr += 4;
            }
            let code = self.dispatch(ins);
            writeln!(writer, "{code}")?;

            if addr >= block.end {
                writeln!(writer, "// Is jump? {} ", self.is_jump)?;
            }

            if addr >= block.end && !self.is_jump {
                // We only do this for non-jumps, because jumps do it themselves.
                writeln!(writer, "cpu.set_next_pc(0x{addr:08x});")?;
            }
        }
        writeln!(writer, "}}")?;

        Ok(())
    }

    pub fn write_blocks(
        &mut self,
        writer: &mut impl Write,
        blocks: impl IntoIterator<Item = &'a Block>,
    ) -> Result<(), BlockWriterError> {
        self.begin(writer)?;
        for block in blocks {
            self.write_block(writer, block)?;
        }

        Ok(())
    }
}

impl HandleRv32i for BlockWriter<'_> {
    type Item = String;

    fn illegal(&mut self, ins: u32) -> Self::Item {
        format!(
            r#"
            cpu.handle_trap(TrapCause::IllegalInstruction({ins}));
        "#
        )
    }

    fn beq(
        &mut self,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        self.is_jump = true;
        format!(
            r#"
        if cpu.rx({rs1}) == cpu.rx({rs2}) {{
            cpu.set_next_pc(0x{:08x});
        }} else {{
            cpu.set_next_pc(0x{:08x});
        }}
        "#,
            self.pc.wrapping_add(bimm),
            self.pc.wrapping_add(4)
        )
    }

    fn bne(
        &mut self,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        self.is_jump = true;
        format!(
            r#"
        if cpu.rx({rs1}) != cpu.rx({rs2}) {{
            cpu.set_next_pc(0x{:08x});
        }} else {{
            cpu.set_next_pc(0x{:08x});
        }}
        "#,
            self.pc.wrapping_add(bimm),
            self.pc.wrapping_add(4)
        )
    }

    fn blt(
        &mut self,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        self.is_jump = true;
        format!(
            r#"
        if (cpu.rx({rs1}) as i32) < (cpu.rx({rs2}) as i32) {{
            cpu.set_next_pc(0x{:08x});
        }} else {{
            cpu.set_next_pc(0x{:08x});
        }}
        "#,
            self.pc.wrapping_add(bimm),
            self.pc.wrapping_add(4)
        )
    }

    fn bge(
        &mut self,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        self.is_jump = true;
        format!(
            r#"
        if (cpu.rx({rs1}) as i32) >= (cpu.rx({rs2}) as i32) {{
            cpu.set_next_pc(0x{:08x});
        }} else {{
            cpu.set_next_pc(0x{:08x});
        }}
        "#,
            self.pc.wrapping_add(bimm),
            self.pc.wrapping_add(4)
        )
    }

    fn bltu(
        &mut self,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        self.is_jump = true;
        format!(
            r#"
        if cpu.rx({rs1}) < cpu.rx({rs2}) {{
            cpu.set_next_pc(0x{:08x});
        }} else {{
            cpu.set_next_pc(0x{:08x});
        }}
        "#,
            self.pc.wrapping_add(bimm),
            self.pc.wrapping_add(4)
        )
    }

    fn bgeu(
        &mut self,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        self.is_jump = true;
        format!(
            r#"
        if cpu.rx({rs1}) >= cpu.rx({rs2}) {{
            cpu.set_next_pc(0x{:08x});
        }} else {{
            cpu.set_next_pc(0x{:08x});
        }}
        "#,
            self.pc.wrapping_add(bimm),
            self.pc.wrapping_add(4)
        )
    }

    fn lb(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        iimm: u32,
    ) -> Self::Item {
        format!(
            r#"
            match cpu.read8(cpu.rx({rs1}).wrapping_add({iimm})) {{
                Ok(byte) => {{
                    cpu.wx({rd}, (((byte as i8) as i16) as i32) as u32); // TODO: this should be a function.
                }}
                Err(address) => {{
                    cpu.handle_trap(TrapCause::LoadAccessFault(address));
                }}
            }}
        "#
        )
    }

    fn lh(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        iimm: u32,
    ) -> Self::Item {
        format!(
            r#"
            match cpu.read16(cpu.rx({rs1}).wrapping_add({iimm})) {{
                Ok(half_word) => {{
                    cpu.wx({rd}, ((half_word as i16) as i32) as u32); // TODO: this should be a function.
                }}
                Err(address) => {{
                    cpu.handle_trap(TrapCause::LoadAccessFault(address));
                }}
            }}
    
        "#
        )
    }

    fn lw(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        iimm: u32,
    ) -> Self::Item {
        format!(
            r#"
            match cpu.read32(cpu.rx({rs1}).wrapping_add({iimm})) {{
                Ok(word) => {{
                    cpu.wx({rd}, word);
                }}
                Err(address) => {{
                    cpu.handle_trap(TrapCause::LoadAccessFault(address));
                }}
            }}
        "#
        )
    }

    fn lbu(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        iimm: u32,
    ) -> Self::Item {
        format!(
            r#"
            match cpu.read8(cpu.rx({rs1}).wrapping_add({iimm})) {{
                Ok(byte) => {{
                    cpu.wx({rd}, byte as u32);
                }}
                Err(address) => {{
                    cpu.handle_trap(TrapCause::LoadAccessFault(address));
                }}
            }}
            "#
        )
    }

    fn lhu(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        iimm: u32,
    ) -> Self::Item {
        format!(
            r#"
            match cpu.read16(cpu.rx({rs1}).wrapping_add({iimm})) {{
                Ok(half_word) => {{
                    cpu.wx({rd}, half_word as u32);
                }}
                Err(address) => cpu.handle_trap(TrapCause::LoadAccessFault(address)),
            }}
        "#
        )
    }

    fn addi(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        iimm: u32,
    ) -> Self::Item {
        format!(
            r#"
            cpu.wx({rd}, cpu.rx({rs1}).wrapping_add({iimm}));
        "#
        )
    }

    fn slti(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        iimm: u32,
    ) -> Self::Item {
        format!(
            r#"
            let xreg_rs1 = cpu.rx({rs1}) as i32;
            let iimm = {iimm} as i32;
            cpu.wx({rd}, if xreg_rs1 < iimm {{ 1 }} else {{ 0 }});
        "#
        )
    }

    fn sltiu(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        iimm: u32,
    ) -> Self::Item {
        format!(
            r#"
            cpu.wx({rd}, if cpu.rx({rs1}) < {iimm} {{ 1 }} else {{ 0 }});
        "#
        )
    }

    fn xori(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        iimm: u32,
    ) -> Self::Item {
        format!(
            r#"
            cpu.wx({rd}, cpu.rx({rs1}) ^ {iimm});
        "#
        )
    }

    fn ori(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        iimm: u32,
    ) -> Self::Item {
        format!(
            r#"
            cpu.wx({rd}, cpu.rx({rs1}) | {iimm});
        "#
        )
    }

    fn andi(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        iimm: u32,
    ) -> Self::Item {
        format!(
            r#"
            cpu.wx({rd}, cpu.rx({rs1}) & {iimm});
        "#
        )
    }

    fn jalr(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        iimm: u32,
    ) -> Self::Item {
        self.is_jump = true;
        format!(
            r#"
            let rs1_before = cpu.rx({rs1}); // Because rd and rs1 might be the same register.
            cpu.wx({rd}, 0x{:08x});
            cpu.set_next_pc(rs1_before.wrapping_add({iimm}) & !1);
        "#,
            self.pc.wrapping_add(4),
        )
    }

    fn sb(
        &mut self,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
        simm: u32,
    ) -> Self::Item {
        format!(
            r#"
            if let Err(address) = 
                cpu.write8(cpu.rx({rs1}).wrapping_add({simm}), (cpu.rx({rs2}) & 0xff) as u8)
            {{
                cpu.handle_trap(TrapCause::StoreAccessFault(address))
            }}
        "#
        )
    }

    fn sh(
        &mut self,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
        simm: u32,
    ) -> Self::Item {
        format!(
            r#"
            if let Err(address) = cpu.write16(
                cpu.rx({rs1}).wrapping_add({simm}),
                (cpu.rx({rs2}) & 0xffff) as u16,
            ) {{
                cpu.handle_trap(TrapCause::StoreAccessFault(address))
            }}
        "#
        )
    }

    fn sw(
        &mut self,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
        simm: u32,
    ) -> Self::Item {
        format!(
            r#"
            if let Err(address) = cpu.write32(cpu.rx({rs1}).wrapping_add({simm}), cpu.rx({rs2})) {{
                cpu.handle_trap(TrapCause::StoreAccessFault(address))
            }}
        "#
        )
    }

    fn auipc(&mut self, rd: arviss::decoding::Reg, uimm: u32) -> Self::Item {
        format!(
            r#"
            cpu.wx({rd}, 0x{:08x});
        "#,
            self.pc.wrapping_add(uimm)
        )
    }

    fn lui(&mut self, rd: arviss::decoding::Reg, uimm: u32) -> Self::Item {
        format!(
            r#"
            cpu.wx({rd}, {uimm});
        "#
        )
    }

    fn jal(&mut self, rd: arviss::decoding::Reg, jimm: u32) -> Self::Item {
        self.is_jump = true;
        format!(
            r#"
            cpu.wx({rd}, 0x{:08x});
            cpu.set_next_pc(0x{:08x});   
        "#,
            self.pc.wrapping_add(4),
            self.pc.wrapping_add(jimm)
        )
    }

    fn add(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
    ) -> Self::Item {
        format!(
            r#"
            cpu.wx({rd}, cpu.rx({rs1}).wrapping_add(cpu.rx({rs2})));
        "#
        )
    }

    fn sub(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
    ) -> Self::Item {
        format!(
            r#"
            cpu.wx({rd}, cpu.rx({rs1}).wrapping_sub(cpu.rx({rs2})));
        "#
        )
    }

    fn sll(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
    ) -> Self::Item {
        format!(
            r#"
            cpu.wx({rd}, cpu.rx({rs1}) << (cpu.rx({rs2}) % 32));
        "#
        )
    }

    fn slt(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
    ) -> Self::Item {
        format!(
            r#"
            let xreg_rs1 = cpu.rx({rs1}) as i32;
            let xreg_rs2 = cpu.rx({rs2}) as i32;
            cpu.wx({rd}, if xreg_rs1 < xreg_rs2 {{ 1 }} else {{ 0 }});
        "#
        )
    }

    fn sltu(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
    ) -> Self::Item {
        format!(
            r#"
            let xreg_rs1 = cpu.rx({rs1});
            let xreg_rs2 = cpu.rx({rs2});
            cpu.wx({rd}, if xreg_rs1 < xreg_rs2 {{ 1 }} else {{ 0 }});
        "#
        )
    }

    fn xor(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
    ) -> Self::Item {
        format!(
            r#"
            cpu.wx({rd}, cpu.rx({rs1}) ^ cpu.rx({rs2}));
        "#
        )
    }

    fn srl(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
    ) -> Self::Item {
        format!(
            r#"
            cpu.wx({rd}, cpu.rx({rs1}) >> (cpu.rx({rs2}) % 32));
        "#
        )
    }

    fn sra(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
    ) -> Self::Item {
        format!(
            r#"
            let xreg_rs1 = cpu.rx({rs1}) as i32;
            let shift = (cpu.rx({rs2}) % 32) as i32;
            cpu.wx({rd}, (xreg_rs1 >> shift) as u32);
        "#
        )
    }

    fn or(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
    ) -> Self::Item {
        format!(
            r#"
            cpu.wx({rd}, cpu.rx({rs1}) | cpu.rx({rs2}));
        "#
        )
    }

    fn and(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
    ) -> Self::Item {
        format!(
            r#"
            cpu.wx({rd}, cpu.rx({rs1}) & cpu.rx({rs2}));
        "#
        )
    }

    fn slli(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        shamt: u32,
    ) -> Self::Item {
        format!(
            r#"
            cpu.wx({rd}, cpu.rx({rs1}) << {shamt});
        "#
        )
    }

    fn srli(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        shamt: u32,
    ) -> Self::Item {
        format!(
            r#"
            cpu.wx({rd}, cpu.rx({rs1}) >> {shamt});
        "#
        )
    }

    fn srai(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        shamt: u32,
    ) -> Self::Item {
        format!(
            r#"
            let xreg_rs = cpu.rx({rs1}) as i32;
            let shamt = {shamt} as i32;
            cpu.wx({rd}, (xreg_rs >> shamt) as u32);
        "#
        )
    }

    fn fence(
        &mut self,
        _fm: u32,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
    ) -> Self::Item {
        "".to_string()
    }

    fn ecall(&mut self) -> Self::Item {
        r#"
            cpu.handle_ecall();
        "#
        .to_string()
    }

    fn ebreak(&mut self) -> Self::Item {
        r#"
            cpu.handle_ebreak();
        "#
        .to_string()
    }
}

impl HandleRv32c for BlockWriter<'_> {
    type Item = String;

    fn c_addi4spn(&mut self, rdp: arviss::decoding::Reg, imm: u32) -> Self::Item {
        format!(
            r#"
            cpu.addi({rdp}, Reg::SP, {imm});
        "#
        )
    }

    fn c_lw(
        &mut self,
        rdp: arviss::decoding::Reg,
        rs1p: arviss::decoding::Reg,
        imm: u32,
    ) -> Self::Item {
        format!(
            r#"
            cpu.lw({rdp}, {rs1p}, {imm});
        "#
        )
    }

    fn c_sw(
        &mut self,
        rs1p: arviss::decoding::Reg,
        rs2p: arviss::decoding::Reg,
        imm: u32,
    ) -> Self::Item {
        format!(
            r#"
            cpu.sw({rs1p}, {rs2p}, {imm});            
        "#
        )
    }

    fn c_sub(&mut self, rdrs1p: arviss::decoding::Reg, rs2p: arviss::decoding::Reg) -> Self::Item {
        format!(
            r#"
            cpu.sub({rdrs1p}, {rdrs1p}, {rs2p});
        "#
        )
    }

    fn c_xor(&mut self, rdrs1p: arviss::decoding::Reg, rs2p: arviss::decoding::Reg) -> Self::Item {
        format!(
            r#"
            cpu.xor({rdrs1p}, {rdrs1p}, {rs2p});
        "#
        )
    }

    fn c_or(&mut self, rdrs1p: arviss::decoding::Reg, rs2p: arviss::decoding::Reg) -> Self::Item {
        format!(
            r#"
            cpu.or({rdrs1p}, {rdrs1p}, {rs2p});
        "#
        )
    }

    fn c_and(&mut self, rdrs1p: arviss::decoding::Reg, rs2p: arviss::decoding::Reg) -> Self::Item {
        format!(
            r#"
            cpu.and({rdrs1p}, {rdrs1p}, {rs2p});
        "#
        )
    }

    fn c_nop(&mut self, _imm: u32) -> Self::Item {
        "".to_string()
    }

    fn c_addi16sp(&mut self, imm: u32) -> Self::Item {
        format!(
            r#"
            cpu.addi(Reg::SP, Reg::SP, {imm});
        "#
        )
    }

    fn c_andi(&mut self, rsrs1p: arviss::decoding::Reg, imm: u32) -> Self::Item {
        format!(
            r#"
            cpu.andi({rsrs1p}, {rsrs1p}, {imm});
        "#
        )
    }

    fn c_addi(&mut self, rdrs1n0: arviss::decoding::Reg, imm: u32) -> Self::Item {
        format!(
            r#"
            cpu.addi({rdrs1n0}, {rdrs1n0}, {imm});
        "#
        )
    }

    fn c_li(&mut self, rd: arviss::decoding::Reg, imm: u32) -> Self::Item {
        format!(
            r#"
            cpu.addi({rd}, Reg::ZERO, {imm});            
        "#
        )
    }

    fn c_lui(&mut self, rdn2: arviss::decoding::Reg, imm: u32) -> Self::Item {
        format!(
            r#"
            cpu.lui({rdn2}, {imm});
        "#
        )
    }

    fn c_j(&mut self, imm: u32) -> Self::Item {
        self.is_jump = true;
        format!(
            r#"
            cpu.set_next_pc(0x{:08x});
        "#,
            self.pc.wrapping_add(imm)
        )
    }

    fn c_beqz(&mut self, rs1p: arviss::decoding::Reg, imm: u32) -> Self::Item {
        self.is_jump = true;
        format!(
            r#"
        if cpu.rx({rs1p}) == 0 {{
            cpu.set_next_pc(0x{:08x});
        }} else {{
            cpu.set_next_pc(0x{:08x});
        }}
        "#,
            self.pc.wrapping_add(imm),
            self.pc.wrapping_add(2)
        )
    }

    fn c_bnez(&mut self, rs1p: arviss::decoding::Reg, imm: u32) -> Self::Item {
        self.is_jump = true;
        format!(
            r#"
        if cpu.rx({rs1p}) != 0 {{
            cpu.set_next_pc(0x{:08x});
        }} else {{
            cpu.set_next_pc(0x{:08x});
        }}
        "#,
            self.pc.wrapping_add(imm),
            self.pc.wrapping_add(2)
        )
    }

    fn c_jr(&mut self, rs1n0: arviss::decoding::Reg) -> Self::Item {
        self.is_jump = true;
        format!(
            r#"
            cpu.set_next_pc(cpu.rx({rs1n0}) & !1);
        "#
        )
    }

    fn c_jalr(&mut self, rs1n0: arviss::decoding::Reg) -> Self::Item {
        self.is_jump = true;
        format!(
            r#"
            cpu.wx(Reg::RA, 0x{:08x});
            cpu.set_next_pc(cpu.rx({rs1n0}) & !1);
            "#,
            self.pc.wrapping_add(2)
        )
    }

    fn c_ebreak(&mut self) -> Self::Item {
        r#"
            cpu.ebreak();
        "#
        .to_string()
    }

    fn c_mv(&mut self, rd: arviss::decoding::Reg, rs2n0: arviss::decoding::Reg) -> Self::Item {
        format!(
            r#"
            cpu.add({rd}, Reg::ZERO, {rs2n0});
        "#
        )
    }

    fn c_add(&mut self, rdrs1: arviss::decoding::Reg, rs2n0: arviss::decoding::Reg) -> Self::Item {
        format!(
            r#"
            cpu.add({rdrs1}, {rdrs1}, {rs2n0});
        "#
        )
    }

    fn c_lwsp(&mut self, rdn0: arviss::decoding::Reg, imm: u32) -> Self::Item {
        format!(
            r#"
            cpu.lw({rdn0}, Reg::SP, {imm});
        "#
        )
    }

    fn c_swsp(&mut self, rs2: arviss::decoding::Reg, imm: u32) -> Self::Item {
        format!(
            r#"
            cpu.sw(Reg::SP, {rs2}, {imm});
        "#
        )
    }

    fn c_jal(&mut self, imm: u32) -> Self::Item {
        self.is_jump = true;
        format!(
            r#"
            cpu.wx(Reg::RA, {0}.wrapping_add(2));
            cpu.set_next_pc(0x{:08x});
        "#,
            self.pc.wrapping_add(imm)
        )
    }

    fn c_srli(&mut self, rdrs1p: arviss::decoding::Reg, imm: u32) -> Self::Item {
        format!(
            r#"
            cpu.srli({rdrs1p}, {rdrs1p}, {imm});
        "#
        )
    }

    fn c_srai(&mut self, rdrs1p: arviss::decoding::Reg, imm: u32) -> Self::Item {
        format!(
            r#"
            cpu.srai({rdrs1p}, {rdrs1p}, {imm});
        "#
        )
    }

    fn c_slli(&mut self, rdrs1n0: arviss::decoding::Reg, imm: u32) -> Self::Item {
        format!(
            r#"
            cpu.slli({rdrs1n0}, {rdrs1n0}, {imm});
        "#
        )
    }
}
