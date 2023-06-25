use arviss::{decoding::Reg, HandleRv32c, HandleRv32i};

pub struct InstructionDecoder;

#[derive(Debug)]
pub struct IllegalOp {
    pub ins: u32,
}

#[derive(Debug)]
pub enum BranchFunc {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

#[derive(Debug)]
pub struct BranchOp {
    pub func: BranchFunc,
    pub rs1: Reg,
    pub rs2: Reg,
    pub bimm: u32,
}

#[derive(Debug)]
pub enum LoadFunc {
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
}

#[derive(Debug)]
pub struct LoadOp {
    pub func: LoadFunc,
    pub rd: Reg,
    pub rs1: Reg,
    pub iimm: u32,
}

#[derive(Debug)]
pub enum AluImmFunc {
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
}

#[derive(Debug)]
pub struct AluImmOp {
    pub func: AluImmFunc,
    pub rd: Reg,
    pub rs1: Reg,
    pub iimm: u32,
}

#[derive(Debug)]
pub enum JalrFunc {
    Jalr,
    CJr,
    CJalr,
}

#[derive(Debug)]
pub struct JalrOp {
    pub func: JalrFunc,
    pub rd: Reg,
    pub rs1: Reg,
    pub iimm: u32,
}

#[derive(Debug)]
pub enum StoreFunc {
    Sb,
    Sh,
    Sw,
}

#[derive(Debug)]
pub struct StoreOp {
    pub func: StoreFunc,
    pub rs1: Reg,
    pub rs2: Reg,
    pub simm: u32,
}

#[derive(Debug)]
pub struct AuipcOp {
    pub rd: Reg,
    pub uimm: u32,
}

#[derive(Debug)]
pub struct LuiOp {
    pub rd: Reg,
    pub uimm: u32,
}

#[derive(Debug)]
pub enum JalFunc {
    Jal,
    CJal,
}

#[derive(Debug)]
pub struct JalOp {
    pub func: JalFunc,
    pub rd: Reg,
    pub jimm: u32,
}

#[derive(Debug)]
pub enum AluFunc {
    Add,
    Sub,
    Slt,
    Sltu,
    Xor,
    Or,
    And,
}

#[derive(Debug)]
pub struct AluOp {
    pub func: AluFunc,
    pub rd: Reg,
    pub rs1: Reg,
    pub rs2: Reg,
}

#[derive(Debug)]
pub enum ShiftFunc {
    Sll,
    Srl,
    Sra,
}

#[derive(Debug)]
pub struct ShiftOp {
    pub func: ShiftFunc,
    pub rd: Reg,
    pub rs1: Reg,
    pub rs2: Reg,
}

#[derive(Debug)]
pub enum ShiftImmFunc {
    Slli,
    Srli,
    Srai,
}

#[derive(Debug)]
pub struct ShiftImmOp {
    pub func: ShiftImmFunc,
    pub rd: Reg,
    pub rs1: Reg,
    pub shamt: u32,
}

#[derive(Debug)]
pub enum Decoded {
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
        Decoded::Jalr(JalrOp {
            func: JalrFunc::Jalr,
            rd,
            rs1,
            iimm,
        })
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
        Decoded::Jal(JalOp {
            func: JalFunc::Jal,
            rd,
            jimm,
        })
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
            func: AluImmFunc::Addi,
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
            func: JalFunc::CJal,
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
            func: JalrFunc::CJr,
            rd: Reg::ZERO,
            rs1: rs1n0,
            iimm: 0,
        })
    }

    fn c_jalr(&mut self, rs1n0: Reg) -> Self::Item {
        // TODO: dedicated instruction.
        Decoded::Jalr(JalrOp {
            func: JalrFunc::CJalr,
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
            func: JalFunc::CJal,
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
