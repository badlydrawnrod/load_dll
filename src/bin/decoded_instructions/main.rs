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
enum SetConditionalFunc {
    Slt,
    Sltu,
}

#[derive(Debug)]
struct SetConditionalOp {
    func: SetConditionalFunc,
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
enum DecodedInstruction {
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
    CondSet(SetConditionalOp),
    ShiftImm(ShiftImmOp),
    Fence,
    Ecall,
    Ebreak,
    Nop,
}

impl HandleRv32i for InstructionDecoder {
    type Item = DecodedInstruction;

    fn illegal(&mut self, ins: u32) -> Self::Item {
        DecodedInstruction::Illegal(IllegalOp { ins })
    }

    fn beq(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        DecodedInstruction::Branch(BranchOp {
            func: BranchFunc::Beq,
            rs1,
            rs2,
            bimm,
        })
    }

    fn bne(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        DecodedInstruction::Branch(BranchOp {
            func: BranchFunc::Bne,
            rs1,
            rs2,
            bimm,
        })
    }

    fn blt(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        DecodedInstruction::Branch(BranchOp {
            func: BranchFunc::Blt,
            rs1,
            rs2,
            bimm,
        })
    }

    fn bge(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        DecodedInstruction::Branch(BranchOp {
            func: BranchFunc::Bge,
            rs1,
            rs2,
            bimm,
        })
    }

    fn bltu(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        DecodedInstruction::Branch(BranchOp {
            func: BranchFunc::Bltu,
            rs1,
            rs2,
            bimm,
        })
    }

    fn bgeu(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        DecodedInstruction::Branch(BranchOp {
            func: BranchFunc::Bgeu,
            rs1,
            rs2,
            bimm,
        })
    }

    fn lb(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::Load(LoadOp {
            func: LoadFunc::Lb,
            rd,
            rs1,
            iimm,
        })
    }

    fn lh(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::Load(LoadOp {
            func: LoadFunc::Lh,
            rd,
            rs1,
            iimm,
        })
    }

    fn lw(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::Load(LoadOp {
            func: LoadFunc::Lw,
            rd,
            rs1,
            iimm,
        })
    }

    fn lbu(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::Load(LoadOp {
            func: LoadFunc::Lbu,
            rd,
            rs1,
            iimm,
        })
    }

    fn lhu(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::Load(LoadOp {
            func: LoadFunc::Lhu,
            rd,
            rs1,
            iimm,
        })
    }

    fn addi(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmOp {
            func: AluImmFunc::Addi,
            rd,
            rs1,
            iimm,
        })
    }

    fn slti(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmOp {
            func: AluImmFunc::Slti,
            rd,
            rs1,
            iimm,
        })
    }

    fn sltiu(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmOp {
            func: AluImmFunc::Sltiu,
            rd,
            rs1,
            iimm,
        })
    }

    fn xori(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmOp {
            func: AluImmFunc::Xori,
            rd,
            rs1,
            iimm,
        })
    }

    fn ori(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmOp {
            func: AluImmFunc::Ori,
            rd,
            rs1,
            iimm,
        })
    }

    fn andi(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmOp {
            func: AluImmFunc::Andi,
            rd,
            rs1,
            iimm,
        })
    }

    fn jalr(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::Jalr(JalrOp { rd, rs1, iimm })
    }

    fn sb(&mut self, rs1: Reg, rs2: Reg, simm: u32) -> Self::Item {
        DecodedInstruction::Store(StoreOp {
            func: StoreFunc::Sb,
            rs1,
            rs2,
            simm,
        })
    }

    fn sh(&mut self, rs1: Reg, rs2: Reg, simm: u32) -> Self::Item {
        DecodedInstruction::Store(StoreOp {
            func: StoreFunc::Sh,
            rs1,
            rs2,
            simm,
        })
    }

    fn sw(&mut self, rs1: Reg, rs2: Reg, simm: u32) -> Self::Item {
        DecodedInstruction::Store(StoreOp {
            func: StoreFunc::Sw,
            rs1,
            rs2,
            simm,
        })
    }

    fn auipc(&mut self, rd: Reg, uimm: u32) -> Self::Item {
        DecodedInstruction::Auipc(AuipcOp { rd, uimm })
    }

    fn lui(&mut self, rd: Reg, uimm: u32) -> Self::Item {
        DecodedInstruction::Lui(LuiOp { rd, uimm })
    }

    fn jal(&mut self, rd: Reg, jimm: u32) -> Self::Item {
        DecodedInstruction::Jal(JalOp { rd, jimm })
    }

    fn add(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        DecodedInstruction::Alu(AluOp {
            func: AluFunc::Add,
            rd,
            rs1,
            rs2,
        })
    }

    fn sub(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        DecodedInstruction::Alu(AluOp {
            func: AluFunc::Sub,
            rd,
            rs1,
            rs2,
        })
    }

    fn sll(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        DecodedInstruction::Shift(ShiftOp {
            func: ShiftFunc::Sll,
            rd,
            rs1,
            rs2,
        })
    }

    fn slt(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        DecodedInstruction::CondSet(SetConditionalOp {
            func: SetConditionalFunc::Slt,
            rd,
            rs1,
            rs2,
        })
    }

    fn sltu(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        DecodedInstruction::CondSet(SetConditionalOp {
            func: SetConditionalFunc::Sltu,
            rd,
            rs1,
            rs2,
        })
    }

    fn xor(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        DecodedInstruction::Alu(AluOp {
            func: AluFunc::Xor,
            rd,
            rs1,
            rs2,
        })
    }

    fn srl(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        DecodedInstruction::Shift(ShiftOp {
            func: ShiftFunc::Srl,
            rd,
            rs1,
            rs2,
        })
    }

    fn sra(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        DecodedInstruction::Shift(ShiftOp {
            func: ShiftFunc::Sra,
            rd,
            rs1,
            rs2,
        })
    }

    fn or(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        DecodedInstruction::Alu(AluOp {
            func: AluFunc::Or,
            rd,
            rs1,
            rs2,
        })
    }

    fn and(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        DecodedInstruction::Alu(AluOp {
            func: AluFunc::And,
            rd,
            rs1,
            rs2,
        })
    }

    fn slli(&mut self, rd: Reg, rs1: Reg, shamt: u32) -> Self::Item {
        DecodedInstruction::ShiftImm(ShiftImmOp {
            func: ShiftImmFunc::Slli,
            rd,
            rs1,
            shamt,
        })
    }

    fn srli(&mut self, rd: Reg, rs1: Reg, shamt: u32) -> Self::Item {
        DecodedInstruction::ShiftImm(ShiftImmOp {
            func: ShiftImmFunc::Srli,
            rd,
            rs1,
            shamt,
        })
    }

    fn srai(&mut self, rd: Reg, rs1: Reg, shamt: u32) -> Self::Item {
        DecodedInstruction::ShiftImm(ShiftImmOp {
            func: ShiftImmFunc::Srai,
            rd,
            rs1,
            shamt,
        })
    }

    fn fence(&mut self, _fm: u32, _rd: Reg, _rs1: Reg) -> Self::Item {
        DecodedInstruction::Fence
    }

    fn ecall(&mut self) -> Self::Item {
        DecodedInstruction::Ecall
    }

    fn ebreak(&mut self) -> Self::Item {
        DecodedInstruction::Ebreak
    }
}

impl HandleRv32c for InstructionDecoder {
    type Item = DecodedInstruction;

    fn c_addi4spn(&mut self, rdp: Reg, imm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmOp {
            func: AluImmFunc::Addi,
            rd: rdp,
            rs1: Reg::SP,
            iimm: imm,
        })
    }

    fn c_lw(&mut self, rdp: Reg, rs1p: Reg, imm: u32) -> Self::Item {
        DecodedInstruction::Load(LoadOp {
            func: LoadFunc::Lw,
            rd: rdp,
            rs1: rs1p,
            iimm: imm,
        })
    }

    fn c_sw(&mut self, rs1p: Reg, rs2p: Reg, imm: u32) -> Self::Item {
        DecodedInstruction::Store(StoreOp {
            func: StoreFunc::Sw,
            rs1: rs1p,
            rs2: rs2p,
            simm: imm,
        })
    }

    fn c_sub(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item {
        DecodedInstruction::Alu(AluOp {
            func: AluFunc::Sub,
            rd: rdrs1p,
            rs1: rdrs1p,
            rs2: rs2p,
        })
    }

    fn c_xor(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item {
        DecodedInstruction::Alu(AluOp {
            func: AluFunc::Xor,
            rd: rdrs1p,
            rs1: rdrs1p,
            rs2: rs2p,
        })
    }

    fn c_or(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item {
        DecodedInstruction::Alu(AluOp {
            func: AluFunc::Or,
            rd: rdrs1p,
            rs1: rdrs1p,
            rs2: rs2p,
        })
    }

    fn c_and(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item {
        DecodedInstruction::Alu(AluOp {
            func: AluFunc::And,
            rd: rdrs1p,
            rs1: rdrs1p,
            rs2: rs2p,
        })
    }

    fn c_nop(&mut self, _imm: u32) -> Self::Item {
        DecodedInstruction::Nop
    }

    fn c_addi16sp(&mut self, imm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmOp {
            func: AluImmFunc::Addi,
            rd: Reg::SP,
            rs1: Reg::SP,
            iimm: imm,
        })
    }

    fn c_andi(&mut self, rsrs1p: Reg, imm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmOp {
            func: AluImmFunc::Andi,
            rd: rsrs1p,
            rs1: rsrs1p,
            iimm: imm,
        })
    }

    fn c_addi(&mut self, rdrs1n0: Reg, imm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmOp {
            func: AluImmFunc::Andi,
            rd: rdrs1n0,
            rs1: rdrs1n0,
            iimm: imm,
        })
    }

    fn c_li(&mut self, rd: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::AluImmediate(AluImmOp {
            func: AluImmFunc::Addi,
            rd,
            rs1: Reg::ZERO,
            iimm: imm,
        })
    }

    fn c_lui(&mut self, rdn2: Reg, imm: u32) -> Self::Item {
        DecodedInstruction::Lui(LuiOp {
            rd: rdn2,
            uimm: imm,
        })
    }

    fn c_j(&mut self, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::Jal(JalOp {
            rd: Reg::ZERO,
            jimm: imm,
        })
    }

    fn c_beqz(&mut self, rs1p: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::Branch(BranchOp {
            func: BranchFunc::Beq,
            rs1: rs1p,
            rs2: Reg::ZERO,
            bimm: imm,
        })
    }

    fn c_bnez(&mut self, rs1p: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::Branch(BranchOp {
            func: BranchFunc::Bne,
            rs1: rs1p,
            rs2: Reg::ZERO,
            bimm: imm,
        })
    }

    fn c_jr(&mut self, rs1n0: Reg) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::Jalr(JalrOp {
            rd: Reg::ZERO,
            rs1: rs1n0,
            iimm: 0,
        })
    }

    fn c_jalr(&mut self, rs1n0: Reg) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::Jalr(JalrOp {
            rd: Reg::RA,
            rs1: rs1n0,
            iimm: 0,
        })
    }

    fn c_ebreak(&mut self) -> Self::Item {
        DecodedInstruction::Ebreak
    }

    fn c_mv(&mut self, rd: Reg, rs2n0: Reg) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::Alu(AluOp {
            func: AluFunc::Add,
            rd,
            rs1: Reg::ZERO,
            rs2: rs2n0,
        })
    }

    fn c_add(&mut self, rdrs1: Reg, rs2n0: Reg) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::Alu(AluOp {
            func: AluFunc::Add,
            rd: rdrs1,
            rs1: rdrs1,
            rs2: rs2n0,
        })
    }

    fn c_lwsp(&mut self, rdn0: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::Load(LoadOp {
            func: LoadFunc::Lw,
            rd: rdn0,
            rs1: Reg::SP,
            iimm: imm,
        })
    }

    fn c_swsp(&mut self, rs2: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::Store(StoreOp {
            func: StoreFunc::Sw,
            rs1: Reg::SP,
            rs2,
            simm: imm,
        })
    }

    fn c_jal(&mut self, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::Jal(JalOp {
            rd: Reg::RA,
            jimm: imm,
        })
    }

    fn c_srli(&mut self, rdrs1p: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::ShiftImm(ShiftImmOp {
            func: ShiftImmFunc::Srli,
            rd: rdrs1p,
            rs1: rdrs1p,
            shamt: imm,
        })
    }

    fn c_srai(&mut self, rdrs1p: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::ShiftImm(ShiftImmOp {
            func: ShiftImmFunc::Srai,
            rd: rdrs1p,
            rs1: rdrs1p,
            shamt: imm,
        })
    }

    fn c_slli(&mut self, rdrs1n0: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::ShiftImm(ShiftImmOp {
            func: ShiftImmFunc::Slli,
            rd: rdrs1n0,
            rs1: rdrs1n0,
            shamt: imm,
        })
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
        cpu.dispatch(ins);
        let disassembled = disassembler.dispatch(ins);
        let decoded = decoder.dispatch(ins);
        println!("Dis: {} Dec: {:?}", disassembled, decoded);
    }

    match cpu.trap_cause() {
        Some(TrapCause::Breakpoint) => {}
        Some(cause) => println!("{:?} at 0x{:08x}", cause, cpu.pc()),
        None => {}
    }
}
