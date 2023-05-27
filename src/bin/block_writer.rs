use arviss::{
    backends::memory::basic::*, disassembler::Disassembler, DispatchRv32ic, HandleRv32c,
    HandleRv32i,
};
use load_dll::block_finder::*;

struct BlockWriter {}

impl HandleRv32i for BlockWriter {
    type Item = String;

    fn illegal(&mut self, ins: u32) -> Self::Item {
        format!(
            r#"
            self.handle_trap(TrapCause::IllegalInstruction({ins}));
        "#
        )
    }

    fn beq(
        &mut self,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        format!(
            r#"
        if self.rx({rs1}) == self.rx({rs2}) {{
            self.set_next_pc(self.pc().wrapping_add({bimm}));
        }}
        "#
        )
    }

    fn bne(
        &mut self,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        format!(
            r#"
        if self.rx({rs1}) != self.rx({rs2}) {{
            self.set_next_pc(self.pc().wrapping_add({bimm}));
        }}
        "#
        )
    }

    fn blt(
        &mut self,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        format!(
            r#"
        if (self.rx({rs1}) as i32) < (self.rx({rs2}) as i32) {{
            self.set_next_pc(self.pc().wrapping_add({bimm}));
        }}
        "#
        )
    }

    fn bge(
        &mut self,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        format!(
            r#"
        if (self.rx({rs1}) as i32) >= (self.rx({rs2}) as i32) {{
            self.set_next_pc(self.pc().wrapping_add({bimm}));
        }}
        "#
        )
    }

    fn bltu(
        &mut self,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        format!(
            r#"
        if self.rx({rs1}) < self.rx({rs2}) {{
            self.set_next_pc(self.pc().wrapping_add({bimm}));
        }}
        "#
        )
    }

    fn bgeu(
        &mut self,
        rs1: arviss::decoding::Reg,
        rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        format!(
            r#"
        if self.rx({rs1}) >= self.rx({rs2}) {{
            self.set_next_pc(self.pc().wrapping_add({bimm}));
        }}
        "#
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
            match self.read8(self.rx({rs1}).wrapping_add({iimm})) {{
                Ok(byte) => {{
                    self.wx({rd}, (((byte as i8) as i16) as i32) as u32); // TODO: this should be a function.
                }}
                Err(address) => {{
                    self.handle_trap(TrapCause::LoadAccessFault(address));
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
            match self.read16(self.rx({rs1}).wrapping_add({iimm})) {{
                Ok(half_word) => {{
                    self.wx({rd}, ((half_word as i16) as i32) as u32); // TODO: this should be a function.
                }}
                Err(address) => {{
                    self.handle_trap(TrapCause::LoadAccessFault(address));
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
            match self.read32(self.rx({rs1}).wrapping_add({iimm})) {{
                Ok(word) => {{
                    self.wx({rd}, word);
                }}
                Err(address) => {{
                    self.handle_trap(TrapCause::LoadAccessFault(address));
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
            match self.read8(self.rx({rs1}).wrapping_add({iimm})) {{
                Ok(byte) => {{
                    self.wx({rd}, byte as u32);
                }}
                Err(address) => {{
                    self.handle_trap(TrapCause::LoadAccessFault(address));
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
            match self.read16(self.rx({rs1}).wrapping_add({iimm})) {{
                Ok(half_word) => {{
                    self.wx({rd}, half_word as u32);
                }}
                Err(address) => self.handle_trap(TrapCause::LoadAccessFault(address)),
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
            self.wx({rd}, self.rx({rs1}).wrapping_add({iimm}));
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
            let xreg_rs1 = self.rx({rs1}) as i32;
            let iimm = {iimm} as i32;
            self.wx({rd}, if xreg_rs1 < iimm {{ 1 }} else {{ 0 }});
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
            self.wx({rd}, if self.rx({rs1}) < {iimm} {{ 1 }} else {{ 0 }});
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
            self.wx({rd}, self.rx({rs1}) ^ {iimm});
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
            self.wx({rd}, self.rx({rs1}) | {iimm});
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
            self.wx({rd}, self.rx({rs1}) & {iimm});
        "#
        )
    }

    fn jalr(
        &mut self,
        rd: arviss::decoding::Reg,
        rs1: arviss::decoding::Reg,
        iimm: u32,
    ) -> Self::Item {
        format!(
            r#"
            let rs1_before = self.rx({rs1}); // Because rd and rs1 might be the same register.
            self.wx({rd}, self.pc().wrapping_add(4));
            self.set_next_pc(rs1_before.wrapping_add({iimm}) & !1);
        "#
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
                self.write8(self.rx({rs1}).wrapping_add({simm}), (self.rx({rs2}) & 0xff) as u8)
            {{
                self.handle_trap(TrapCause::StoreAccessFault(address))
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
            if let Err(address) = self.write16(
                self.rx({rs1}).wrapping_add({simm}),
                (self.rx({rs2}) & 0xffff) as u16,
            ) {{
                self.handle_trap(TrapCause::StoreAccessFault(address))
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
            if let Err(address) = self.write32(self.rx({rs1}).wrapping_add({simm}), self.rx({rs2})) {{
                self.handle_trap(TrapCause::StoreAccessFault(address))
            }}
        "#
        )
    }

    fn auipc(&mut self, rd: arviss::decoding::Reg, uimm: u32) -> Self::Item {
        format!(
            r#"
            self.wx({rd}, self.pc().wrapping_add({uimm}));
        "#
        )
    }

    fn lui(&mut self, rd: arviss::decoding::Reg, uimm: u32) -> Self::Item {
        format!(
            r#"
            self.wx({rd}, {uimm});
        "#
        )
    }

    fn jal(&mut self, rd: arviss::decoding::Reg, jimm: u32) -> Self::Item {
        format!(
            r#"
            self.wx({rd}, self.pc().wrapping_add(4));
            self.set_next_pc(self.pc().wrapping_add({jimm}));   
        "#
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
            self.wx({rd}, self.rx({rs1}).wrapping_add(self.rx({rs2})));
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
            self.wx({rd}, self.rx({rs1}).wrapping_sub(self.rx({rs2})));
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
            self.wx({rd}, self.rx({rs1}) << (self.rx({rs2}) % 32));
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
            let xreg_rs1 = self.rx({rs1}) as i32;
            let xreg_rs2 = self.rx({rs2}) as i32;
            self.wx({rd}, if xreg_rs1 < xreg_rs2 {{ 1 }} else {{ 0 }});
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
            let xreg_rs1 = self.rx({rs1});
            let xreg_rs2 = self.rx({rs2});
            self.wx({rd}, if xreg_rs1 < xreg_rs2 {{ 1 }} else {{ 0 }});
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
            self.wx({rd}, self.rx({rs1}) ^ self.rx({rs2}));
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
            self.wx({rd}, self.rx({rs1}) >> (self.rx({rs2}) % 32));
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
            let xreg_rs1 = self.rx({rs1}) as i32;
            let shift = (self.rx({rs2}) % 32) as i32;
            self.wx({rd}, (xreg_rs1 >> shift) as u32);
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
            self.wx({rd}, self.rx({rs1}) | self.rx({rs2}));
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
            self.wx({rd}, self.rx({rs1}) & self.rx({rs2}));
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
            self.wx({rd}, self.rx({rs1}) << {shamt});
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
            self.wx({rd}, self.rx({rs1}) >> {shamt});
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
            let xreg_rs = self.rx({rs1}) as i32;
            let shamt = {shamt} as i32;
            self.wx({rd}, (xreg_rs >> shamt) as u32);
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
        format!(
            r#"
            self.handle_ecall();
        "#
        )
    }

    fn ebreak(&mut self) -> Self::Item {
        format!(
            r#"
            self.handle_ebreak();
        "#
        )
    }
}

impl HandleRv32c for BlockWriter {
    type Item = String;

    fn c_addi4spn(&mut self, rdp: arviss::decoding::Reg, imm: u32) -> Self::Item {
        format!(
            r#"
            self.addi({rdp}, Reg::SP, {imm});
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
            self.lw({rdp}, {rs1p}, {imm});
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
            self.sw({rs1p}, {rs2p}, {imm});            
        "#
        )
    }

    fn c_sub(&mut self, rdrs1p: arviss::decoding::Reg, rs2p: arviss::decoding::Reg) -> Self::Item {
        format!(
            r#"
            self.sub({rdrs1p}, {rdrs1p}, {rs2p});
        "#
        )
    }

    fn c_xor(&mut self, rdrs1p: arviss::decoding::Reg, rs2p: arviss::decoding::Reg) -> Self::Item {
        format!(
            r#"
            self.xor({rdrs1p}, {rdrs1p}, {rs2p});
        "#
        )
    }

    fn c_or(&mut self, rdrs1p: arviss::decoding::Reg, rs2p: arviss::decoding::Reg) -> Self::Item {
        format!(
            r#"
            self.or({rdrs1p}, {rdrs1p}, {rs2p});
        "#
        )
    }

    fn c_and(&mut self, rdrs1p: arviss::decoding::Reg, rs2p: arviss::decoding::Reg) -> Self::Item {
        format!(
            r#"
            self.and({rdrs1p}, {rdrs1p}, {rs2p});
        "#
        )
    }

    fn c_nop(&mut self, _imm: u32) -> Self::Item {
        "".to_string()
    }

    fn c_addi16sp(&mut self, imm: u32) -> Self::Item {
        format!(
            r#"
            self.addi(Reg::SP, Reg::SP, {imm});
        "#
        )
    }

    fn c_andi(&mut self, rsrs1p: arviss::decoding::Reg, imm: u32) -> Self::Item {
        format!(
            r#"
            self.andi({rsrs1p}, {rsrs1p}, {imm});
        "#
        )
    }

    fn c_addi(&mut self, rdrs1n0: arviss::decoding::Reg, imm: u32) -> Self::Item {
        format!(
            r#"
            self.addi({rdrs1n0}, {rdrs1n0}, {imm});
        "#
        )
    }

    fn c_li(&mut self, rd: arviss::decoding::Reg, imm: u32) -> Self::Item {
        format!(
            r#"
            self.addi({rd}, Reg::ZERO, {imm});            
        "#
        )
    }

    fn c_lui(&mut self, rdn2: arviss::decoding::Reg, imm: u32) -> Self::Item {
        format!(
            r#"
            self.lui({rdn2}, {imm});
        "#
        )
    }

    fn c_j(&mut self, imm: u32) -> Self::Item {
        format!(
            r#"
            self.set_next_pc(self.pc().wrapping_add({imm}));            
        "#
        )
    }

    fn c_beqz(&mut self, rs1p: arviss::decoding::Reg, imm: u32) -> Self::Item {
        format!(
            r#"
            self.beq({rs1p}, Reg::ZERO, {imm});
        "#
        )
    }

    fn c_bnez(&mut self, rs1p: arviss::decoding::Reg, imm: u32) -> Self::Item {
        format!(
            r#"
            self.bne({rs1p}, Reg::ZERO, {imm});
        "#
        )
    }

    fn c_jr(&mut self, rs1n0: arviss::decoding::Reg) -> Self::Item {
        format!(
            r#"
            self.set_next_pc(self.rx({rs1n0}) & !1);
        "#
        )
    }

    fn c_jalr(&mut self, rs1n0: arviss::decoding::Reg) -> Self::Item {
        format!(
            r#"
            self.wx(Reg::RA, self.pc().wrapping_add(2));
            self.set_next_pc(self.rx({rs1n0}) & !1);
    
        "#
        )
    }

    fn c_ebreak(&mut self) -> Self::Item {
        format!(
            r#"
            self.ebreak();
        "#
        )
    }

    fn c_mv(&mut self, rd: arviss::decoding::Reg, rs2n0: arviss::decoding::Reg) -> Self::Item {
        format!(
            r#"
            self.add({rd}, Reg::ZERO, {rs2n0});
        "#
        )
    }

    fn c_add(&mut self, rdrs1: arviss::decoding::Reg, rs2n0: arviss::decoding::Reg) -> Self::Item {
        format!(
            r#"
            self.add({rdrs1}, {rdrs1}, {rs2n0});
        "#
        )
    }

    fn c_lwsp(&mut self, rdn0: arviss::decoding::Reg, imm: u32) -> Self::Item {
        format!(
            r#"
            self.lw({rdn0}, Reg::SP, {imm});
        "#
        )
    }

    fn c_swsp(&mut self, rs2: arviss::decoding::Reg, imm: u32) -> Self::Item {
        format!(
            r#"
            self.sw(Reg::SP, {rs2}, {imm});
        "#
        )
    }

    fn c_jal(&mut self, imm: u32) -> Self::Item {
        format!(
            r#"
            self.wx(Reg::RA, self.pc().wrapping_add(2));
            self.set_next_pc(self.pc().wrapping_add({imm}));
        "#
        )
    }

    fn c_srli(&mut self, rdrs1p: arviss::decoding::Reg, imm: u32) -> Self::Item {
        format!(
            r#"
            self.srli({rdrs1p}, {rdrs1p}, {imm});
        "#
        )
    }

    fn c_srai(&mut self, rdrs1p: arviss::decoding::Reg, imm: u32) -> Self::Item {
        format!(
            r#"
            self.srai({rdrs1p}, {rdrs1p}, {imm});
        "#
        )
    }

    fn c_slli(&mut self, rdrs1n0: arviss::decoding::Reg, imm: u32) -> Self::Item {
        format!(
            r#"
            self.slli({rdrs1n0}, {rdrs1n0}, {imm});
        "#
        )
    }
}

pub fn main() {
    // Load the image into a buffer.
    let path = "images/hello_world.rv32ic";
    let Ok(file_data) = std::fs::read(path) else {
        eprintln!("Failed to read file: `{}`", path);
        std::process::exit(1);
    };
    let image = file_data.as_slice();

    // Copy the image into memory.
    let mut mem = BasicMem::new();
    if let Err(addr) = mem.write_bytes(0, image) {
        eprintln!("Failed to initialize memory at: 0x{:08x}", addr);
        std::process::exit(1);
    };

    // Find the basic blocks in the image.
    let text_size = image.len() - 4; // TODO: The image needs to tell us how big its text and initialized data are.
    let mut block_finder = BlockFinder::<BasicMem>::with_mem(&mem, text_size);
    let blocks = match block_finder.find_blocks(0) {
        Ok(blocks) => blocks,
        Err(err) => {
            eprintln!("ERROR: {}", err);
            std::process::exit(1);
        }
    };

    // Output each basic block as Rust code.
    let mut dis = Disassembler;
    let mut block_writer = BlockWriter {};
    for block in blocks {
        let mut addr = block.start;
        println!("\nfn block_{:08x}_{:08x}() {{", block.start, block.end);
        while addr < block.end {
            let Ok(ins) = mem.read32(addr) else {
                eprintln!("Failed to read memory when compiling 0x{:08x}", addr);
                std::process::exit(1);
            };

            // Disassemble it and compile it.
            let code = dis.dispatch(ins);
            let is_compact = (ins & 3) != 3;
            if is_compact {
                // Compact instructions are 2 bytes each.
                print!("// {:08x}     {:04x} {}", addr, ins & 0xffff, code);
                addr += 2;
            } else {
                // Regular instructions are 4 bytes each.
                print!("// {:08x} {:08x} {}", addr, ins, code);
                addr += 4;
            }
            let code = block_writer.dispatch(ins);
            println!("{code}");
        }
        println!("}}");
    }
}
