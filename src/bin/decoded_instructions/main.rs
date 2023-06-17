use arviss::{
    decoding::Reg,
    disassembler::{self, Disassembler},
    platforms::basic::*,
    DispatchRv32ic, HandleRv32c, HandleRv32i,
};

struct InstructionDecoder;

#[derive(Debug)]
struct IllegalOp {
    ins: u32,
}

#[derive(Debug)]
enum BranchFunc {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

#[derive(Debug)]
struct BranchOp {
    func: BranchFunc,
    rs1: Reg,
    rs2: Reg,
    bimm: u32,
}

#[derive(Debug)]
enum LoadFunc {
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
}

#[derive(Debug)]
struct LoadOp {
    func: LoadFunc,
    rd: Reg,
    rs1: Reg,
    iimm: u32,
}

#[derive(Debug)]
enum AluImmFunc {
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
}

#[derive(Debug)]
struct AluImmOp {
    func: AluImmFunc,
    rd: Reg,
    rs1: Reg,
    iimm: u32,
}

#[derive(Debug)]
struct JalrOp {
    rd: Reg,
    rs1: Reg,
    iimm: u32,
}

#[derive(Debug)]
enum StoreFunc {
    Sb,
    Sh,
    Sw,
}

#[derive(Debug)]
struct StoreOp {
    func: StoreFunc,
    rs1: Reg,
    rs2: Reg,
    simm: u32,
}

#[derive(Debug)]
struct AuipcOp {
    rd: Reg,
    uimm: u32,
}

#[derive(Debug)]
struct LuiOp {
    rd: Reg,
    uimm: u32,
}

#[derive(Debug)]
struct JalOp {
    rd: Reg,
    jimm: u32,
}

#[derive(Debug)]
enum AluFunc {
    Add,
    Sub,
    Slt,
    Sltu,
    Xor,
    Or,
    And,
}

#[derive(Debug)]
struct AluOp {
    func: AluFunc,
    rd: Reg,
    rs1: Reg,
    rs2: Reg,
}

#[derive(Debug)]
enum ShiftFunc {
    Sll,
    Srl,
    Sra,
}

#[derive(Debug)]
struct ShiftOp {
    func: ShiftFunc,
    rd: Reg,
    rs1: Reg,
    rs2: Reg,
}

#[derive(Debug)]
enum ShiftImmFunc {
    Slli,
    Srli,
    Srai,
}

#[derive(Debug)]
struct ShiftImmOp {
    func: ShiftImmFunc,
    rd: Reg,
    rs1: Reg,
    shamt: u32,
}

#[derive(Debug)]
enum Decoded {
    Illegal(IllegalOp),
    Branch(BranchOp),
    Load(LoadOp),
    AluImmediate(AluImmOp),
    Shift(ShiftOp),
    Jalr(JalrOp),
    Store(StoreOp),
    Auipc(AuipcOp),
    Lui(LuiOp),
    Jal(JalOp),
    Alu(AluOp),
    ShiftImm(ShiftImmOp),
    Fence,
    Ecall,
    Ebreak,
    Nop,
}

impl HandleRv32i for InstructionDecoder {
    type Item = Decoded;

    fn illegal(&mut self, ins: u32) -> Self::Item {
        Decoded::Illegal(IllegalOp { ins })
    }

    fn beq(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        Decoded::Branch(BranchOp {
            func: BranchFunc::Beq,
            rs1,
            rs2,
            bimm,
        })
    }

    fn bne(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        Decoded::Branch(BranchOp {
            func: BranchFunc::Bne,
            rs1,
            rs2,
            bimm,
        })
    }

    fn blt(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        Decoded::Branch(BranchOp {
            func: BranchFunc::Blt,
            rs1,
            rs2,
            bimm,
        })
    }

    fn bge(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        Decoded::Branch(BranchOp {
            func: BranchFunc::Bge,
            rs1,
            rs2,
            bimm,
        })
    }

    fn bltu(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        Decoded::Branch(BranchOp {
            func: BranchFunc::Bltu,
            rs1,
            rs2,
            bimm,
        })
    }

    fn bgeu(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        Decoded::Branch(BranchOp {
            func: BranchFunc::Bgeu,
            rs1,
            rs2,
            bimm,
        })
    }

    fn lb(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        Decoded::Load(LoadOp {
            func: LoadFunc::Lb,
            rd,
            rs1,
            iimm,
        })
    }

    fn lh(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        Decoded::Load(LoadOp {
            func: LoadFunc::Lh,
            rd,
            rs1,
            iimm,
        })
    }

    fn lw(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        Decoded::Load(LoadOp {
            func: LoadFunc::Lw,
            rd,
            rs1,
            iimm,
        })
    }

    fn lbu(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        Decoded::Load(LoadOp {
            func: LoadFunc::Lbu,
            rd,
            rs1,
            iimm,
        })
    }

    fn lhu(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        Decoded::Load(LoadOp {
            func: LoadFunc::Lhu,
            rd,
            rs1,
            iimm,
        })
    }

    fn addi(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        Decoded::AluImmediate(AluImmOp {
            func: AluImmFunc::Addi,
            rd,
            rs1,
            iimm,
        })
    }

    fn slti(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        Decoded::AluImmediate(AluImmOp {
            func: AluImmFunc::Slti,
            rd,
            rs1,
            iimm,
        })
    }

    fn sltiu(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        Decoded::AluImmediate(AluImmOp {
            func: AluImmFunc::Sltiu,
            rd,
            rs1,
            iimm,
        })
    }

    fn xori(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        Decoded::AluImmediate(AluImmOp {
            func: AluImmFunc::Xori,
            rd,
            rs1,
            iimm,
        })
    }

    fn ori(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        Decoded::AluImmediate(AluImmOp {
            func: AluImmFunc::Ori,
            rd,
            rs1,
            iimm,
        })
    }

    fn andi(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        Decoded::AluImmediate(AluImmOp {
            func: AluImmFunc::Andi,
            rd,
            rs1,
            iimm,
        })
    }

    fn jalr(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        Decoded::Jalr(JalrOp { rd, rs1, iimm })
    }

    fn sb(&mut self, rs1: Reg, rs2: Reg, simm: u32) -> Self::Item {
        Decoded::Store(StoreOp {
            func: StoreFunc::Sb,
            rs1,
            rs2,
            simm,
        })
    }

    fn sh(&mut self, rs1: Reg, rs2: Reg, simm: u32) -> Self::Item {
        Decoded::Store(StoreOp {
            func: StoreFunc::Sh,
            rs1,
            rs2,
            simm,
        })
    }

    fn sw(&mut self, rs1: Reg, rs2: Reg, simm: u32) -> Self::Item {
        Decoded::Store(StoreOp {
            func: StoreFunc::Sw,
            rs1,
            rs2,
            simm,
        })
    }

    fn auipc(&mut self, rd: Reg, uimm: u32) -> Self::Item {
        Decoded::Auipc(AuipcOp { rd, uimm })
    }

    fn lui(&mut self, rd: Reg, uimm: u32) -> Self::Item {
        Decoded::Lui(LuiOp { rd, uimm })
    }

    fn jal(&mut self, rd: Reg, jimm: u32) -> Self::Item {
        Decoded::Jal(JalOp { rd, jimm })
    }

    fn add(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        Decoded::Alu(AluOp {
            func: AluFunc::Add,
            rd,
            rs1,
            rs2,
        })
    }

    fn sub(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        Decoded::Alu(AluOp {
            func: AluFunc::Sub,
            rd,
            rs1,
            rs2,
        })
    }

    fn sll(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        Decoded::Shift(ShiftOp {
            func: ShiftFunc::Sll,
            rd,
            rs1,
            rs2,
        })
    }

    fn slt(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        Decoded::Alu(AluOp {
            func: AluFunc::Slt,
            rd,
            rs1,
            rs2,
        })
    }

    fn sltu(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        Decoded::Alu(AluOp {
            func: AluFunc::Sltu,
            rd,
            rs1,
            rs2,
        })
    }

    fn xor(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        Decoded::Alu(AluOp {
            func: AluFunc::Xor,
            rd,
            rs1,
            rs2,
        })
    }

    fn srl(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        Decoded::Shift(ShiftOp {
            func: ShiftFunc::Srl,
            rd,
            rs1,
            rs2,
        })
    }

    fn sra(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        Decoded::Shift(ShiftOp {
            func: ShiftFunc::Sra,
            rd,
            rs1,
            rs2,
        })
    }

    fn or(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        Decoded::Alu(AluOp {
            func: AluFunc::Or,
            rd,
            rs1,
            rs2,
        })
    }

    fn and(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        Decoded::Alu(AluOp {
            func: AluFunc::And,
            rd,
            rs1,
            rs2,
        })
    }

    fn slli(&mut self, rd: Reg, rs1: Reg, shamt: u32) -> Self::Item {
        Decoded::ShiftImm(ShiftImmOp {
            func: ShiftImmFunc::Slli,
            rd,
            rs1,
            shamt,
        })
    }

    fn srli(&mut self, rd: Reg, rs1: Reg, shamt: u32) -> Self::Item {
        Decoded::ShiftImm(ShiftImmOp {
            func: ShiftImmFunc::Srli,
            rd,
            rs1,
            shamt,
        })
    }

    fn srai(&mut self, rd: Reg, rs1: Reg, shamt: u32) -> Self::Item {
        Decoded::ShiftImm(ShiftImmOp {
            func: ShiftImmFunc::Srai,
            rd,
            rs1,
            shamt,
        })
    }

    fn fence(&mut self, _fm: u32, _rd: Reg, _rs1: Reg) -> Self::Item {
        Decoded::Fence
    }

    fn ecall(&mut self) -> Self::Item {
        Decoded::Ecall
    }

    fn ebreak(&mut self) -> Self::Item {
        Decoded::Ebreak
    }
}

impl HandleRv32c for InstructionDecoder {
    type Item = Decoded;

    fn c_addi4spn(&mut self, rdp: Reg, imm: u32) -> Self::Item {
        Decoded::AluImmediate(AluImmOp {
            func: AluImmFunc::Addi,
            rd: rdp,
            rs1: Reg::SP,
            iimm: imm,
        })
    }

    fn c_lw(&mut self, rdp: Reg, rs1p: Reg, imm: u32) -> Self::Item {
        Decoded::Load(LoadOp {
            func: LoadFunc::Lw,
            rd: rdp,
            rs1: rs1p,
            iimm: imm,
        })
    }

    fn c_sw(&mut self, rs1p: Reg, rs2p: Reg, imm: u32) -> Self::Item {
        Decoded::Store(StoreOp {
            func: StoreFunc::Sw,
            rs1: rs1p,
            rs2: rs2p,
            simm: imm,
        })
    }

    fn c_sub(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item {
        Decoded::Alu(AluOp {
            func: AluFunc::Sub,
            rd: rdrs1p,
            rs1: rdrs1p,
            rs2: rs2p,
        })
    }

    fn c_xor(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item {
        Decoded::Alu(AluOp {
            func: AluFunc::Xor,
            rd: rdrs1p,
            rs1: rdrs1p,
            rs2: rs2p,
        })
    }

    fn c_or(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item {
        Decoded::Alu(AluOp {
            func: AluFunc::Or,
            rd: rdrs1p,
            rs1: rdrs1p,
            rs2: rs2p,
        })
    }

    fn c_and(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item {
        Decoded::Alu(AluOp {
            func: AluFunc::And,
            rd: rdrs1p,
            rs1: rdrs1p,
            rs2: rs2p,
        })
    }

    fn c_nop(&mut self, _imm: u32) -> Self::Item {
        Decoded::Nop
    }

    fn c_addi16sp(&mut self, imm: u32) -> Self::Item {
        Decoded::AluImmediate(AluImmOp {
            func: AluImmFunc::Addi,
            rd: Reg::SP,
            rs1: Reg::SP,
            iimm: imm,
        })
    }

    fn c_andi(&mut self, rsrs1p: Reg, imm: u32) -> Self::Item {
        Decoded::AluImmediate(AluImmOp {
            func: AluImmFunc::Andi,
            rd: rsrs1p,
            rs1: rsrs1p,
            iimm: imm,
        })
    }

    fn c_addi(&mut self, rdrs1n0: Reg, imm: u32) -> Self::Item {
        Decoded::AluImmediate(AluImmOp {
            func: AluImmFunc::Andi,
            rd: rdrs1n0,
            rs1: rdrs1n0,
            iimm: imm,
        })
    }

    fn c_li(&mut self, rd: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        Decoded::AluImmediate(AluImmOp {
            func: AluImmFunc::Addi,
            rd,
            rs1: Reg::ZERO,
            iimm: imm,
        })
    }

    fn c_lui(&mut self, rdn2: Reg, imm: u32) -> Self::Item {
        Decoded::Lui(LuiOp {
            rd: rdn2,
            uimm: imm,
        })
    }

    fn c_j(&mut self, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        Decoded::Jal(JalOp {
            rd: Reg::ZERO,
            jimm: imm,
        })
    }

    fn c_beqz(&mut self, rs1p: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        Decoded::Branch(BranchOp {
            func: BranchFunc::Beq,
            rs1: rs1p,
            rs2: Reg::ZERO,
            bimm: imm,
        })
    }

    fn c_bnez(&mut self, rs1p: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        Decoded::Branch(BranchOp {
            func: BranchFunc::Bne,
            rs1: rs1p,
            rs2: Reg::ZERO,
            bimm: imm,
        })
    }

    fn c_jr(&mut self, rs1n0: Reg) -> Self::Item {
        // TODO: dedicated instruction.
        Decoded::Jalr(JalrOp {
            rd: Reg::ZERO,
            rs1: rs1n0,
            iimm: 0,
        })
    }

    fn c_jalr(&mut self, rs1n0: Reg) -> Self::Item {
        // TODO: dedicated instruction.
        Decoded::Jalr(JalrOp {
            rd: Reg::RA,
            rs1: rs1n0,
            iimm: 0,
        })
    }

    fn c_ebreak(&mut self) -> Self::Item {
        Decoded::Ebreak
    }

    fn c_mv(&mut self, rd: Reg, rs2n0: Reg) -> Self::Item {
        // TODO: dedicated instruction.
        Decoded::Alu(AluOp {
            func: AluFunc::Add,
            rd,
            rs1: Reg::ZERO,
            rs2: rs2n0,
        })
    }

    fn c_add(&mut self, rdrs1: Reg, rs2n0: Reg) -> Self::Item {
        // TODO: dedicated instruction.
        Decoded::Alu(AluOp {
            func: AluFunc::Add,
            rd: rdrs1,
            rs1: rdrs1,
            rs2: rs2n0,
        })
    }

    fn c_lwsp(&mut self, rdn0: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        Decoded::Load(LoadOp {
            func: LoadFunc::Lw,
            rd: rdn0,
            rs1: Reg::SP,
            iimm: imm,
        })
    }

    fn c_swsp(&mut self, rs2: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        Decoded::Store(StoreOp {
            func: StoreFunc::Sw,
            rs1: Reg::SP,
            rs2,
            simm: imm,
        })
    }

    fn c_jal(&mut self, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        Decoded::Jal(JalOp {
            rd: Reg::RA,
            jimm: imm,
        })
    }

    fn c_srli(&mut self, rdrs1p: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        Decoded::ShiftImm(ShiftImmOp {
            func: ShiftImmFunc::Srli,
            rd: rdrs1p,
            rs1: rdrs1p,
            shamt: imm,
        })
    }

    fn c_srai(&mut self, rdrs1p: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        Decoded::ShiftImm(ShiftImmOp {
            func: ShiftImmFunc::Srai,
            rd: rdrs1p,
            rs1: rdrs1p,
            shamt: imm,
        })
    }

    fn c_slli(&mut self, rdrs1n0: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        Decoded::ShiftImm(ShiftImmOp {
            func: ShiftImmFunc::Slli,
            rd: rdrs1n0,
            rs1: rdrs1n0,
            shamt: imm,
        })
    }
}

fn execute<T>(cpu: &mut T, ins: &Decoded)
where
    T: Fetch + Trap + XRegisters + Memory,
{
    match ins {
        Decoded::Illegal(c) => cpu.illegal(c.ins),
        Decoded::Branch(c) => match c.func {
            BranchFunc::Beq => cpu.beq(c.rs1, c.rs2, c.bimm),
            BranchFunc::Bne => cpu.bne(c.rs1, c.rs2, c.bimm),
            BranchFunc::Blt => cpu.blt(c.rs1, c.rs2, c.bimm),
            BranchFunc::Bge => cpu.bge(c.rs1, c.rs2, c.bimm),
            BranchFunc::Bltu => cpu.bltu(c.rs1, c.rs2, c.bimm),
            BranchFunc::Bgeu => cpu.bgeu(c.rs1, c.rs2, c.bimm),
        },
        Decoded::Load(c) => match c.func {
            LoadFunc::Lb => cpu.lb(c.rd, c.rs1, c.iimm),
            LoadFunc::Lh => cpu.lh(c.rd, c.rs1, c.iimm),
            LoadFunc::Lw => cpu.lw(c.rd, c.rs1, c.iimm),
            LoadFunc::Lbu => cpu.lbu(c.rd, c.rs1, c.iimm),
            LoadFunc::Lhu => cpu.lhu(c.rd, c.rs1, c.iimm),
        },
        Decoded::AluImmediate(c) => match c.func {
            AluImmFunc::Addi => cpu.addi(c.rd, c.rs1, c.iimm),
            AluImmFunc::Slti => cpu.slti(c.rd, c.rs1, c.iimm),
            AluImmFunc::Sltiu => cpu.sltiu(c.rd, c.rs1, c.iimm),
            AluImmFunc::Xori => cpu.xori(c.rd, c.rs1, c.iimm),
            AluImmFunc::Ori => cpu.ori(c.rd, c.rs1, c.iimm),
            AluImmFunc::Andi => cpu.andi(c.rd, c.rs1, c.iimm),
        },
        Decoded::Shift(c) => match c.func {
            ShiftFunc::Sll => cpu.sll(c.rd, c.rs1, c.rs2),
            ShiftFunc::Srl => cpu.srl(c.rd, c.rs1, c.rs2),
            ShiftFunc::Sra => cpu.sra(c.rd, c.rs1, c.rs2),
        },
        Decoded::Jalr(c) => cpu.jalr(c.rd, c.rs1, c.iimm),
        Decoded::Store(c) => match c.func {
            StoreFunc::Sb => cpu.sb(c.rs1, c.rs2, c.simm),
            StoreFunc::Sh => cpu.sh(c.rs1, c.rs2, c.simm),
            StoreFunc::Sw => cpu.sw(c.rs1, c.rs2, c.simm),
        },
        Decoded::Auipc(c) => cpu.auipc(c.rd, c.uimm),
        Decoded::Lui(c) => cpu.lui(c.rd, c.uimm),
        Decoded::Jal(c) => cpu.jal(c.rd, c.jimm),
        Decoded::Alu(c) => match c.func {
            AluFunc::Add => cpu.add(c.rd, c.rs1, c.rs2),
            AluFunc::Sub => cpu.sub(c.rd, c.rs1, c.rs2),
            AluFunc::Slt => cpu.slt(c.rd, c.rs1, c.rs2),
            AluFunc::Sltu => cpu.sltu(c.rd, c.rs1, c.rs2),
            AluFunc::Xor => cpu.xor(c.rd, c.rs1, c.rs2),
            AluFunc::Or => cpu.or(c.rd, c.rs1, c.rs2),
            AluFunc::And => cpu.and(c.rd, c.rs1, c.rs2),
        },
        Decoded::ShiftImm(c) => match c.func {
            ShiftImmFunc::Slli => cpu.slli(c.rd, c.rs1, c.shamt),
            ShiftImmFunc::Srli => cpu.srli(c.rd, c.rs1, c.shamt),
            ShiftImmFunc::Srai => cpu.srai(c.rd, c.rs1, c.shamt),
        },
        Decoded::Fence => cpu.fence(0, Reg::ZERO, Reg::ZERO), // TODO: does this seem right?
        Decoded::Ecall => cpu.ecall(),
        Decoded::Ebreak => cpu.ebreak(),
        Decoded::Nop => cpu.c_nop(0), // TODO: does this seem right?
    }
}

pub fn main() {
    // Load the image into a buffer.
    // let path = "images/hello_world.rv32ic";
    let path = "images/hello_world.rv32i";
    let Ok(file_data) = std::fs::read(path) else {
        eprintln!("Failed to read file: `{}`", path);
        std::process::exit(1);
    };
    let image = file_data.as_slice();

    // Create a simulator and copy the image from the buffer into simulator memory.
    let mut cpu = Rv32iCpu::<BasicMem>::new();
    cpu.write_bytes(0, image)
        .expect("Failed to initialize memory.");

    let mut decoder = InstructionDecoder {};
    let mut disassembler = Disassembler {};

    // Run until we can run no more.
    while !cpu.is_trapped() {
        // Fetch.
        let ins = cpu.fetch().unwrap();

        // Decode and dispatch.
        // cpu.dispatch(ins);
        let disassembled = disassembler.dispatch(ins);
        let decoded = decoder.dispatch(ins);
        execute(&mut cpu, &decoded);
        println!("Dis: {} Dec: {:?}", disassembled, decoded);
    }

    match cpu.trap_cause() {
        Some(TrapCause::Breakpoint) => {}
        Some(cause) => println!("{:?} at 0x{:08x}", cause, cpu.pc()),
        None => {}
    }
}
