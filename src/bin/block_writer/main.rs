use arviss::platforms::basic::*;
use arviss::Address;
use arviss::{disassembler::Disassembler, DispatchRv32ic, HandleRv32c, HandleRv32i};
use libloading::{Library, Symbol};
use load_dll::block_finder::*;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::process::Command;
use tempdir::TempDir;

struct BlockWriter {
    pc: Address,
    is_jump: bool,
}

impl BlockWriter {
    fn new(pc: Address) -> Self {
        BlockWriter { pc, is_jump: false }
    }
}

impl HandleRv32i for BlockWriter {
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
        format!(
            r#"
            cpu.handle_ecall();
        "#
        )
    }

    fn ebreak(&mut self) -> Self::Item {
        format!(
            r#"
            cpu.handle_ebreak();
        "#
        )
    }
}

impl HandleRv32c for BlockWriter {
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
        format!(
            r#"
            cpu.ebreak();
        "#
        )
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

    // Open a temporary directory that will be cleaned up at the end.
    let Ok(dir) = TempDir::new("rhtest") else {
        eprintln!("Failed to create temporary directory");
        std::process::exit(1);
    };

    // Create a file in that directory.
    let file_path = dir.path().join("demo.rs");
    println!("Look in {:?} for the generated code", file_path);
    let Ok(mut f) = File::create(file_path) else {
        eprintln!("Failed to create file");
        std::process::exit(1);
    };

    writeln!(f, "use arviss;").unwrap();
    writeln!(f, "use arviss::HandleRv32i;").unwrap();
    writeln!(f, "use arviss::platforms::basic::*;").unwrap();
    writeln!(f, "use arviss::decoding::Reg;").unwrap();
    writeln!(f, "type Cpu = Rv32iCpu::<BasicMem>;").unwrap();

    // Output each basic block as Rust code.
    let mut dis = Disassembler;
    let mut block_writer = BlockWriter::new(0);
    for block in blocks {
        let mut addr = block.start;
        writeln!(f, "\n#[no_mangle]").unwrap(); // TODO: don't unwrap.
        writeln!(
            f,
            "pub extern \"C\" fn block_{:08x}_{:08x}(cpu: &mut Cpu) {{",
            block.start, block.end
        )
        .unwrap(); // TODO: don't unwrap.
        while addr < block.end {
            block_writer.is_jump = false;
            block_writer.pc = addr;
            let Ok(ins) = mem.read32(addr) else {
                println!("Failed to read memory when compiling 0x{:08x}", addr);
                std::process::exit(1);
            };

            // Disassemble it and compile it.
            let code = dis.dispatch(ins);
            let is_compact = (ins & 3) != 3;
            if is_compact {
                // Compact instructions are 2 bytes each.
                writeln!(f, "// {:08x}     {:04x} {}", addr, ins & 0xffff, code).unwrap(); // Don't unwrap.
                addr += 2;
            } else {
                // Regular instructions are 4 bytes each.
                writeln!(f, "// {:08x} {:08x} {}", addr, ins, code).unwrap(); // Don't unwrap.
                addr += 4;
            }
            let code = block_writer.dispatch(ins);
            writeln!(f, "{code}").unwrap(); // TODO: don't unwrap.

            if addr >= block.end {
                writeln!(f, "// Is jump? {} ", block_writer.is_jump).unwrap(); // TODO: don't unwrap
            }

            if addr >= block.end && !block_writer.is_jump {
                // We only do this for non-jumps, because jumps do it themselves.
                writeln!(f, "cpu.set_next_pc(0x{addr:08x});").unwrap(); // TODO: don't unwrap
            }
        }
        writeln!(f, "}}").unwrap(); // TODO: don't unwrap.;
    }

    if let Err(err) = f.sync_all() {
        eprintln!("Failed to sync: {err}");
        std::process::exit(1);
    }

    // Compile it to a .so.
    let filename = dir.path().join("demo.rs").to_string_lossy().to_string();
    let mut command = Command::new("rustc");
    let Ok(run) = command
        .current_dir(dir.path())
        .arg("--edition=2021")
        .arg("--crate-type")
        .arg("cdylib")
        .arg("--extern")
        .arg("arviss=/home/rod/projects/learn_rust/100days/load_dll/target/debug/deps/libarviss-fa3eb26a5be62bea.rlib")
        .arg("-C")
        .arg("opt-level=2")
        .arg("-C")
        .arg("strip=debuginfo")
        .arg(filename)
        .status() else {
            eprintln!("Failed to compile");
            std::process::exit(1);
        };
    assert!(run.success());

    // Create a simulator.
    type Cpu = Rv32iCpu<BasicMem>;
    type ArvissFunc = extern "C" fn(&mut Cpu);

    let mut cpu = Cpu::new();

    // Load the library.
    let library_path = dir.path().join("libdemo.so");
    println!(
        "Look in {:?} for the generated code and library",
        dir.path()
    );

    unsafe {
        let lib = Library::new(library_path).unwrap();

        // Run the compiled code that we loaded from the DLL against our simulator.
        let run_one: Symbol<ArvissFunc> = lib.get(b"block_00000000_00000024").unwrap();
        run_one(&mut cpu);
        cpu.transfer();
        println!("cpu.pc = {}", cpu.pc());
    }

    // Give the user (me) an opportunity to disassemble the binary.
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}
