use std::ops::RemAssign;

use arviss::{decoding::Reg, platforms::basic::*, DispatchRv32ic, HandleRv32i};

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

enum AluType {
    Addi,
    Slti,
    Sltiu,
    Xori,
    Ori,
    Andi,
}

struct AluImmediateInstruction {
    alu: AluType,
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

enum DecodedInstruction {
    Illegal(IllegalInstruction),
    Branch(BranchInstruction),
    Load(LoadInstruction),
    AluImmediate(AluImmediateInstruction),
    Jalr(JalrInstruction),
    Store(StoreInstruction),
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
            alu: AluType::Addi,
            rd,
            rs1,
            iimm,
        })
    }

    fn slti(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmediateInstruction {
            alu: AluType::Slti,
            rd,
            rs1,
            iimm,
        })
    }

    fn sltiu(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmediateInstruction {
            alu: AluType::Sltiu,
            rd,
            rs1,
            iimm,
        })
    }

    fn xori(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmediateInstruction {
            alu: AluType::Xori,
            rd,
            rs1,
            iimm,
        })
    }

    fn ori(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmediateInstruction {
            alu: AluType::Ori,
            rd,
            rs1,
            iimm,
        })
    }

    fn andi(&mut self, rd: Reg, rs1: Reg, iimm: u32) -> Self::Item {
        DecodedInstruction::AluImmediate(AluImmediateInstruction {
            alu: AluType::Andi,
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
        todo!()
    }

    fn lui(&mut self, rd: Reg, uimm: u32) -> Self::Item {
        todo!()
    }

    fn jal(&mut self, rd: Reg, jimm: u32) -> Self::Item {
        todo!()
    }

    fn add(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        todo!()
    }

    fn sub(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        todo!()
    }

    fn sll(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        todo!()
    }

    fn slt(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        todo!()
    }

    fn sltu(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        todo!()
    }

    fn xor(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        todo!()
    }

    fn srl(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        todo!()
    }

    fn sra(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        todo!()
    }

    fn or(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        todo!()
    }

    fn and(&mut self, rd: Reg, rs1: Reg, rs2: Reg) -> Self::Item {
        todo!()
    }

    fn slli(&mut self, rd: Reg, rs1: Reg, shamt: u32) -> Self::Item {
        todo!()
    }

    fn srli(&mut self, rd: Reg, rs1: Reg, shamt: u32) -> Self::Item {
        todo!()
    }

    fn srai(&mut self, rd: Reg, rs1: Reg, shamt: u32) -> Self::Item {
        todo!()
    }

    fn fence(&mut self, fm: u32, rd: Reg, rs1: Reg) -> Self::Item {
        todo!()
    }

    fn ecall(&mut self) -> Self::Item {
        todo!()
    }

    fn ebreak(&mut self) -> Self::Item {
        todo!()
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
