mod decoded;

use std::collections::HashMap;
use std::f32::consts::E;

use decoded::*;

use arviss::{
    decoding::Reg, platforms::basic::*, Address, DispatchRv32ic, HandleRv32c, HandleRv32i,
};
use load_dll::block_finder::BlockFinder;
use load_dll::read_instruction::*;

pub struct Instruction {
    pub decoded: Decoded, // The decoded instruction.
    pub next_pc: Address, // The value of PC after the instruction assuming it doesn't branch.
}

fn execute<T>(cpu: &mut T, ins: &Instruction) -> Address
where
    T: Fetch + Trap + XRegisters + Memory,
{
    // Here's what we *think* the next pc will be. It could be modified by a jump.
    cpu.set_next_pc(ins.next_pc);

    match &ins.decoded {
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
        Decoded::Jalr(c) => match c.func {
            JalrFunc::Jalr => cpu.jalr(c.rd, c.rs1, c.iimm),
            JalrFunc::CJr => cpu.c_jr(c.rs1), // TODO: perhaps this doesn't belong here.
            JalrFunc::CJalr => cpu.c_jalr(c.rs1), // TODO: perhaps this doesn't belong here.
        },
        Decoded::Store(c) => match c.func {
            StoreFunc::Sb => cpu.sb(c.rs1, c.rs2, c.simm),
            StoreFunc::Sh => cpu.sh(c.rs1, c.rs2, c.simm),
            StoreFunc::Sw => cpu.sw(c.rs1, c.rs2, c.simm),
        },
        Decoded::Auipc(c) => cpu.auipc(c.rd, c.uimm),
        Decoded::Lui(c) => cpu.lui(c.rd, c.uimm),
        Decoded::Jal(c) => match c.func {
            JalFunc::Jal => cpu.jal(c.rd, c.jimm),
            JalFunc::CJal => cpu.c_jal(c.jimm), // TODO: perhaps this doesn't belong here.
        },
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

    // Update PC to reflect reality.
    cpu.transfer()
}

type DecodedBlock = Vec<Instruction>;

pub struct DecodingCompiler {
    block_map: HashMap<Address, DecodedBlock>,
}

impl DecodingCompiler {
    pub fn new() -> Self {
        Self {
            block_map: HashMap::new(),
        }
    }

    pub fn get(&self, addr: Address) -> Option<&DecodedBlock> {
        self.block_map.get(&addr)
    }

    pub fn compile(&mut self, image: &[u8]) {
        // Find the basic blocks in the image.
        let mut block_finder = BlockFinder::with_mem(image);
        let blocks = match block_finder.find_blocks(0) {
            Ok(blocks) => blocks,
            Err(err) => {
                eprintln!("ERROR: {}", err);
                std::process::exit(1);
            }
        };

        // Decode each block.
        let mut decoder = InstructionDecoder {};
        for block in blocks {
            let mut addr = block.start;
            let mut decoded_block = Vec::new();
            while addr < block.end {
                // Decode the instruction.
                let ins = read_instruction(image, addr).unwrap(); // TODO: Don't unwrap.
                let decoded = decoder.dispatch(ins);

                // Determine what next_pc should be.
                let is_compact = (ins & 3) != 3;
                if is_compact {
                    // Compact instructions are 2 bytes each.
                    addr += 2;
                } else {
                    // Regular instructions are 4 bytes each.
                    addr += 4;
                }
                let result = Instruction {
                    decoded,
                    next_pc: addr,
                };
                decoded_block.push(result);
            }
            self.block_map.insert(block.start, decoded_block);
        }

        // At this point we have a block map consisting of decoded blocks.
    }
}

pub fn main() {
    // Load the image into a buffer.
    let path = "images/hello_world.rv32ic";
    // let path = "images/hello_world.rv32i";
    let Ok(file_data) = std::fs::read(path) else {
        eprintln!("Failed to read file: `{}`", path);
        std::process::exit(1);
    };
    let image = file_data.as_slice();

    // Create the reference simulator and copy the image from the buffer into simulator memory.
    let mut reference_cpu = Rv32iCpu::<BasicMem>::new();
    reference_cpu
        .write_bytes(0, image)
        .expect("Failed to initialize memory in reference cpu.");

    // Create the simulator that we want to test. Give it the same image.
    let mut test_cpu = Rv32iCpu::<BasicMem>::new();
    test_cpu
        .write_bytes(0, image)
        .expect("Failed to initialize memory in test cpu.");

    // Create a decoding compiler.
    let mut decoding_compiler = DecodingCompiler::new();
    decoding_compiler.compile(image);

    // Run until we've run out of basic blocks or we're trapped.
    let mut addr = 0;
    while let Some(decoded_block) = decoding_compiler.get(addr) {
        for decoded in decoded_block {
            addr = execute(&mut test_cpu, &decoded);
        }
        if test_cpu.is_trapped() {
            break;
        }
    }

    match test_cpu.trap_cause() {
        Some(TrapCause::Breakpoint) => {}
        Some(cause) => println!("Test CPU: {:?} at 0x{:08x}", cause, test_cpu.pc()),
        None => {}
    }
}
