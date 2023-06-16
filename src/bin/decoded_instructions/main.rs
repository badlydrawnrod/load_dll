use arviss::{decoding::Reg, platforms::basic::*, DispatchRv32ic, HandleRv32c, HandleRv32i};

struct InstructionDecoder<M: Memory>(Rv32iCpu<M>);

struct IllegalInstruction {
    ins: u32,
}

enum BranchType {
    Beq,
    Bne,
    Blt,
    Bge,
    Bltu,
    Bgeu,
}

struct BranchInstruction {
    branch_type: BranchType,
    rs1: Reg,
    rs2: Reg,
    bimm: u32,
}

enum LoadType {
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
}

struct LoadInstruction {
    width: LoadType,
    rd: Reg,
    rs1: Reg,
    iimm: u32,
}

enum AluImmediateType {
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
}

struct AluImmediateInstruction {
    alu: AluImmediateType,
    rd: Reg,
    rs1: Reg,
    iimm: u32,
}

struct JalrInstruction {
    rd: Reg,
    rs1: Reg,
    iimm: u32,
}

enum StoreType {
    Sb,
    Sh,
    Sw,
}

struct StoreInstruction {
    width: StoreType,
    rs1: Reg,
    rs2: Reg,
    simm: u32,
}

struct AuipcInstruction {
    rd: Reg,
    uimm: u32,
}

struct LuiInstruction {
    rd: Reg,
    uimm: u32,
}

struct JalInstruction {
    rd: Reg,
    jimm: u32,
}

enum AluType {
    Add,
    Sub,
    Xor,
    Or,
    And,
}

struct AluInstruction {
    alu: AluType,
    rd: Reg,
    rs1: Reg,
    rs2: Reg,
}

enum ShiftType {
    Sll,
    Srl,
    Sra,
}

struct ShiftInstruction {
    shift: ShiftType,
    rd: Reg,
    rs1: Reg,
    rs2: Reg,
}

enum ConditionalSetType {
    Slt,
    Sltu,
}

struct ConditionalSetInstructional {
    cond: ConditionalSetType,
    rd: Reg,
    rs1: Reg,
    rs2: Reg,
}

enum ShiftImmediateType {
    Slli,
    Srli,
    Srai,
}

struct ShiftImmediateInstruction {
    shift: ShiftImmediateType,
    rd: Reg,
    rs1: Reg,
    shamt: u32,
}

enum DecodedInstruction {
    Illegal(IllegalInstruction),
    Branch(BranchInstruction),
    Load(LoadInstruction),
    AluImmediate(AluImmediateInstruction),
    Shift(ShiftInstruction),
    Jalr(JalrInstruction),
    Store(StoreInstruction),
    Auipc(AuipcInstruction),
    Lui(LuiInstruction),
    Jal(JalInstruction),
    Alu(AluInstruction),
    ConditionalSet(ConditionalSetInstructional),
    ShiftImmediate(ShiftImmediateInstruction),
    Fence,
    Ecall,
    Ebreak,
    Nop,
}

impl<M: Memory> HandleRv32i for InstructionDecoder<M> {
    type Item = DecodedInstruction;

    fn illegal(&mut self, ins: u32) -> Self::Item {
        DecodedInstruction::Illegal(IllegalInstruction { ins })
    }

    fn beq(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        DecodedInstruction::Branch(BranchInstruction {
            branch_type: BranchType::Beq,
            rs1,
            rs2,
            bimm,
        })
    }

    fn bne(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        DecodedInstruction::Branch(BranchInstruction {
            branch_type: BranchType::Bne,
            rs1,
            rs2,
            bimm,
        })
    }

    fn blt(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        DecodedInstruction::Branch(BranchInstruction {
            branch_type: BranchType::Blt,
            rs1,
            rs2,
            bimm,
        })
    }

    fn bge(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        DecodedInstruction::Branch(BranchInstruction {
            branch_type: BranchType::Bge,
            rs1,
            rs2,
            bimm,
        })
    }

    fn bltu(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        DecodedInstruction::Branch(BranchInstruction {
            branch_type: BranchType::Bltu,
            rs1,
            rs2,
            bimm,
        })
    }

    fn bgeu(&mut self, rs1: Reg, rs2: Reg, bimm: u32) -> Self::Item {
        DecodedInstruction::Branch(BranchInstruction {
            branch_type: BranchType::Bgeu,
            rs1,
            rs2,
            bimm,
        })
    }

    fn lb(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::Load(LoadInstruction {
            width: LoadType::Lb,
            rd,
            rs1,
            iimm,
        })
    }

    fn lh(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::Load(LoadInstruction {
            width: LoadType::Lh,
            rd,
            rs1,
            iimm,
        })
    }

    fn lw(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::Load(LoadInstruction {
            width: LoadType::Lw,
            rd,
            rs1,
            iimm,
        })
    }

    fn lbu(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::Load(LoadInstruction {
            width: LoadType::Lbu,
            rd,
            rs1,
            iimm,
        })
    }

    fn lhu(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::Load(LoadInstruction {
            width: LoadType::Lhu,
            rd,
            rs1,
            iimm,
        })
    }

    fn addi(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmediateInstruction {
            alu: AluImmediateType::Addi,
            rd,
            rs1,
            iimm,
        })
    }

    fn slti(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmediateInstruction {
            alu: AluImmediateType::Slti,
            rd,
            rs1,
            iimm,
        })
    }

    fn sltiu(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmediateInstruction {
            alu: AluImmediateType::Sltiu,
            rd,
            rs1,
            iimm,
        })
    }

    fn xori(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmediateInstruction {
            alu: AluImmediateType::Xori,
            rd,
            rs1,
            iimm,
        })
    }

    fn ori(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmediateInstruction {
            alu: AluImmediateType::Ori,
            rd,
            rs1,
            iimm,
        })
    }

    fn andi(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmediateInstruction {
            alu: AluImmediateType::Andi,
            rd,
            rs1,
            iimm,
        })
    }

    fn jalr(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::Jalr(JalrInstruction { rd, rs1, iimm })
    }

    fn sb(&mut self, rs1: Reg, rs2: Reg, simm: u32) -> Self::Item {
        DecodedInstruction::Store(StoreInstruction {
            width: StoreType::Sb,
            rs1,
            rs2,
            simm,
        })
    }

    fn sh(&mut self, rs1: Reg, rs2: Reg, simm: u32) -> Self::Item {
        DecodedInstruction::Store(StoreInstruction {
            width: StoreType::Sh,
            rs1,
            rs2,
            simm,
        })
    }

    fn sw(&mut self, rs1: Reg, rs2: Reg, simm: u32) -> Self::Item {
        DecodedInstruction::Store(StoreInstruction {
            width: StoreType::Sw,
            rs1,
            rs2,
            simm,
        })
    }

    fn auipc(&mut self, rd: Reg, uimm: u32) -> Self::Item {
        DecodedInstruction::Auipc(AuipcInstruction { rd, uimm })
    }

    fn lui(&mut self, rd: Reg, uimm: u32) -> Self::Item {
        DecodedInstruction::Lui(LuiInstruction { rd, uimm })
    }

    fn jal(&mut self, rd: Reg, jimm: u32) -> Self::Item {
        DecodedInstruction::Jal(JalInstruction { rd, jimm })
    }

    fn add(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        DecodedInstruction::Alu(AluInstruction {
            alu: AluType::Add,
            rd,
            rs1,
            rs2,
        })
    }

    fn sub(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        DecodedInstruction::Alu(AluInstruction {
            alu: AluType::Sub,
            rd,
            rs1,
            rs2,
        })
    }

    fn sll(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        DecodedInstruction::Shift(ShiftInstruction {
            shift: ShiftType::Sll,
            rd,
            rs1,
            rs2,
        })
    }

    fn slt(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        DecodedInstruction::ConditionalSet(ConditionalSetInstructional {
            cond: ConditionalSetType::Slt,
            rd,
            rs1,
            rs2,
        })
    }

    fn sltu(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        DecodedInstruction::ConditionalSet(ConditionalSetInstructional {
            cond: ConditionalSetType::Sltu,
            rd,
            rs1,
            rs2,
        })
    }

    fn xor(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        DecodedInstruction::Alu(AluInstruction {
            alu: AluType::Xor,
            rd,
            rs1,
            rs2,
        })
    }

    fn srl(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        DecodedInstruction::Shift(ShiftInstruction {
            shift: ShiftType::Srl,
            rd,
            rs1,
            rs2,
        })
    }

    fn sra(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        DecodedInstruction::Shift(ShiftInstruction {
            shift: ShiftType::Sra,
            rd,
            rs1,
            rs2,
        })
    }

    fn or(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        DecodedInstruction::Alu(AluInstruction {
            alu: AluType::Or,
            rd,
            rs1,
            rs2,
        })
    }

    fn and(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        DecodedInstruction::Alu(AluInstruction {
            alu: AluType::And,
            rd,
            rs1,
            rs2,
        })
    }

    fn slli(&mut self, rd: Reg, rs1: Reg, shamt: u32) -> Self::Item {
        DecodedInstruction::ShiftImmediate(ShiftImmediateInstruction {
            shift: ShiftImmediateType::Slli,
            rd,
            rs1,
            shamt,
        })
    }

    fn srli(&mut self, rd: Reg, rs1: Reg, shamt: u32) -> Self::Item {
        DecodedInstruction::ShiftImmediate(ShiftImmediateInstruction {
            shift: ShiftImmediateType::Srli,
            rd,
            rs1,
            shamt,
        })
    }

    fn srai(&mut self, rd: Reg, rs1: Reg, shamt: u32) -> Self::Item {
        DecodedInstruction::ShiftImmediate(ShiftImmediateInstruction {
            shift: ShiftImmediateType::Srai,
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

impl<M: Memory> HandleRv32c for InstructionDecoder<M> {
    type Item = DecodedInstruction;

    fn c_addi4spn(&mut self, rdp: Reg, imm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmediateInstruction {
            alu: AluImmediateType::Addi,
            rd: rdp,
            rs1: Reg::SP,
            iimm: imm,
        })
    }

    fn c_lw(&mut self, rdp: Reg, rs1p: Reg, imm: u32) -> Self::Item {
        DecodedInstruction::Load(LoadInstruction {
            width: LoadType::Lw,
            rd: rdp,
            rs1: rs1p,
            iimm: imm,
        })
    }

    fn c_sw(&mut self, rs1p: Reg, rs2p: Reg, imm: u32) -> Self::Item {
        DecodedInstruction::Store(StoreInstruction {
            width: StoreType::Sw,
            rs1: rs1p,
            rs2: rs2p,
            simm: imm,
        })
    }

    fn c_sub(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item {
        DecodedInstruction::Alu(AluInstruction {
            alu: AluType::Sub,
            rd: rdrs1p,
            rs1: rdrs1p,
            rs2: rs2p,
        })
    }

    fn c_xor(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item {
        DecodedInstruction::Alu(AluInstruction {
            alu: AluType::Xor,
            rd: rdrs1p,
            rs1: rdrs1p,
            rs2: rs2p,
        })
    }

    fn c_or(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item {
        DecodedInstruction::Alu(AluInstruction {
            alu: AluType::Or,
            rd: rdrs1p,
            rs1: rdrs1p,
            rs2: rs2p,
        })
    }

    fn c_and(&mut self, rdrs1p: Reg, rs2p: Reg) -> Self::Item {
        DecodedInstruction::Alu(AluInstruction {
            alu: AluType::And,
            rd: rdrs1p,
            rs1: rdrs1p,
            rs2: rs2p,
        })
    }

    fn c_nop(&mut self, _imm: u32) -> Self::Item {
        DecodedInstruction::Nop
    }

    fn c_addi16sp(&mut self, imm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmediateInstruction {
            alu: AluImmediateType::Addi,
            rd: Reg::SP,
            rs1: Reg::SP,
            iimm: imm,
        })
    }

    fn c_andi(&mut self, rsrs1p: Reg, imm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmediateInstruction {
            alu: AluImmediateType::Andi,
            rd: rsrs1p,
            rs1: rsrs1p,
            iimm: imm,
        })
    }

    fn c_addi(&mut self, rdrs1n0: Reg, imm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmediateInstruction {
            alu: AluImmediateType::Andi,
            rd: rdrs1n0,
            rs1: rdrs1n0,
            iimm: imm,
        })
    }

    fn c_li(&mut self, rd: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::AluImmediate(AluImmediateInstruction {
            alu: AluImmediateType::Addi,
            rd,
            rs1: Reg::ZERO,
            iimm: imm,
        })
    }

    fn c_lui(&mut self, rdn2: Reg, imm: u32) -> Self::Item {
        DecodedInstruction::Lui(LuiInstruction {
            rd: rdn2,
            uimm: imm,
        })
    }

    fn c_j(&mut self, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::Jal(JalInstruction {
            rd: Reg::ZERO,
            jimm: imm,
        })
    }

    fn c_beqz(&mut self, rs1p: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::Branch(BranchInstruction {
            branch_type: BranchType::Beq,
            rs1: rs1p,
            rs2: Reg::ZERO,
            bimm: imm,
        })
    }

    fn c_bnez(&mut self, rs1p: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::Branch(BranchInstruction {
            branch_type: BranchType::Bne,
            rs1: rs1p,
            rs2: Reg::ZERO,
            bimm: imm,
        })
    }

    fn c_jr(&mut self, rs1n0: Reg) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::Jalr(JalrInstruction {
            rd: Reg::ZERO,
            rs1: rs1n0,
            iimm: 0,
        })
    }

    fn c_jalr(&mut self, rs1n0: Reg) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::Jalr(JalrInstruction {
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
        DecodedInstruction::Alu(AluInstruction {
            alu: AluType::Add,
            rd,
            rs1: Reg::ZERO,
            rs2: rs2n0,
        })
    }

    fn c_add(&mut self, rdrs1: Reg, rs2n0: Reg) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::Alu(AluInstruction {
            alu: AluType::Add,
            rd: rdrs1,
            rs1: rdrs1,
            rs2: rs2n0,
        })
    }

    fn c_lwsp(&mut self, rdn0: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::Load(LoadInstruction {
            width: LoadType::Lw,
            rd: rdn0,
            rs1: Reg::SP,
            iimm: imm,
        })
    }

    fn c_swsp(&mut self, rs2: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::Store(StoreInstruction {
            width: StoreType::Sw,
            rs1: Reg::SP,
            rs2,
            simm: imm,
        })
    }

    fn c_jal(&mut self, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::Jal(JalInstruction {
            rd: Reg::RA,
            jimm: imm,
        })
    }

    fn c_srli(&mut self, rdrs1p: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::ShiftImmediate(ShiftImmediateInstruction {
            shift: ShiftImmediateType::Srli,
            rd: rdrs1p,
            rs1: rdrs1p,
            shamt: imm,
        })
    }

    fn c_srai(&mut self, rdrs1p: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::ShiftImmediate(ShiftImmediateInstruction {
            shift: ShiftImmediateType::Srai,
            rd: rdrs1p,
            rs1: rdrs1p,
            shamt: imm,
        })
    }

    fn c_slli(&mut self, rdrs1n0: Reg, imm: u32) -> Self::Item {
        // TODO: dedicated instruction.
        DecodedInstruction::ShiftImmediate(ShiftImmediateInstruction {
            shift: ShiftImmediateType::Slli,
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

    // Run until we can run no more.
    while !cpu.is_trapped() {
        // Fetch.
        let ins = cpu.fetch().unwrap();

        // Decode and dispatch.
        cpu.dispatch(ins);
    }

    match cpu.trap_cause() {
        Some(TrapCause::Breakpoint) => {}
        Some(cause) => println!("{:?} at 0x{:08x}", cause, cpu.pc()),
        None => {}
    }
}
