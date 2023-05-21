use std::fs::File;
use std::io::prelude::*;
use std::ops::{Index, IndexMut};

use arviss::backends::memory::basic::*;
use arviss::{
    disassembler::Disassembler, Address, DispatchRv32ic, HandleRv32c, HandleRv32i, MemoryResult,
};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Block {
    start: Address, // Address of the first instruction in the basic block.
    end: Address,   // Address of the last instruction in the basic block.
}

const OPEN_BLOCK_SENTINEL: Address = 0;

impl Block {
    fn new(start: Address) -> Self {
        Block {
            start,
            end: OPEN_BLOCK_SENTINEL,
        }
    }
}

struct BlockFinder<M>
where
    M: Memory,
{
    addr: Address,
    mem: M,
    eom: Address,
    known_blocks: Vec<Block>,
    open_blocks: Vec<usize>,
    current_block: usize,
}

impl<M> BlockFinder<M>
where
    M: Memory,
{
    pub fn with_mem(mem: M, len: usize) -> Self {
        let addr = 0;
        let eom = addr + (len as Address);
        Self {
            addr: 0,
            mem,
            eom,
            known_blocks: Vec::new(),
            open_blocks: Vec::new(),
            current_block: 0,
        }
    }

    fn next(&mut self) -> MemoryResult<u32> {
        self.mem.read32(self.addr)
    }

    fn start_block(&mut self, addr: Address) {
        // Ignore addresses that are outside of the address range.
        if addr >= self.eom {
            return;
        }

        // Only add previously unknown blocks.
        let is_unknown = self.known_blocks.iter().all(|b| b.start != addr);
        if is_unknown {
            // Start a new block.
            self.known_blocks.push(Block::new(addr));
            let index = self.known_blocks.len() - 1;
            self.open_blocks.push(index);

            // If the new block splits an existing block then terminate the existing block at the address immediately
            // before the new block.
            let splits_block = self
                .known_blocks
                .iter_mut()
                .find(|b| b.start < addr && addr <= b.end);
            if let Some(block) = splits_block {
                // Scan the block to find the end, as we need to account for variable sized instructions.
                // TODO: would this be easier with an open range?
                let mut end: Address = block.start;
                let mut instruction_size = 0;
                while end < addr {
                    let ins = self.mem.read32(end).unwrap(); // TODO: Lose the unwrap and handle the error.
                    instruction_size = if (ins & 3) == 3 { 4 } else { 2 };
                    end += instruction_size;
                }
                block.end = end - instruction_size;
            }
        }
    }

    fn end_block(&mut self, addr: Address) {
        let block = self.known_blocks.index_mut(self.current_block);
        block.end = addr;
    }

    fn run(&mut self, addr: Address) {
        self.start_block(addr);
        while let Some(current_block) = self.open_blocks.pop() {
            self.current_block = current_block;
            let mut block = self.known_blocks.index(self.current_block);
            self.addr = block.start;
            while self.addr < self.eom && block.end == OPEN_BLOCK_SENTINEL {
                let ins = self.next().unwrap(); // TODO: Lose the unwrap and handle the error.
                self.dispatch(ins);
                let instruction_size = if (ins & 3) == 3 { 4 } else { 2 };
                self.addr = self.addr.wrapping_add(instruction_size);
                block = self.known_blocks.index(self.current_block);
            }
        }
        self.known_blocks.sort_unstable();
    }
}

impl<M> HandleRv32i for BlockFinder<M>
where
    M: Memory,
{
    type Item = ();

    fn illegal(&mut self, _ins: u32) -> Self::Item {}

    fn beq(
        &mut self,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        self.end_block(self.addr);
        self.start_block(self.addr + 4);
        self.start_block(self.addr.wrapping_add(bimm));
    }

    fn bne(
        &mut self,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        self.end_block(self.addr);
        self.start_block(self.addr + 4);
        self.start_block(self.addr.wrapping_add(bimm));
    }

    fn blt(
        &mut self,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        self.end_block(self.addr);
        self.start_block(self.addr + 4);
        self.start_block(self.addr.wrapping_add(bimm));
    }

    fn bge(
        &mut self,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        self.end_block(self.addr);
        self.start_block(self.addr + 4);
        self.start_block(self.addr.wrapping_add(bimm));
    }

    fn bltu(
        &mut self,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        self.end_block(self.addr);
        self.start_block(self.addr + 4);
        self.start_block(self.addr.wrapping_add(bimm));
    }

    fn bgeu(
        &mut self,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        self.end_block(self.addr);
        self.start_block(self.addr + 4);
        self.start_block(self.addr.wrapping_add(bimm));
    }

    fn lb(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _iimm: u32,
    ) -> Self::Item {
    }

    fn lh(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _iimm: u32,
    ) -> Self::Item {
    }

    fn lw(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _iimm: u32,
    ) -> Self::Item {
    }

    fn lbu(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _iimm: u32,
    ) -> Self::Item {
    }

    fn lhu(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _iimm: u32,
    ) -> Self::Item {
    }

    fn addi(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _iimm: u32,
    ) -> Self::Item {
    }

    fn slti(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _iimm: u32,
    ) -> Self::Item {
    }

    fn sltiu(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _iimm: u32,
    ) -> Self::Item {
    }

    fn xori(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _iimm: u32,
    ) -> Self::Item {
    }

    fn ori(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _iimm: u32,
    ) -> Self::Item {
    }

    fn andi(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _iimm: u32,
    ) -> Self::Item {
    }

    fn jalr(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _iimm: u32,
    ) -> Self::Item {
        self.end_block(self.addr);
        self.start_block(self.addr + 4);
    }

    fn sb(
        &mut self,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
        _simm: u32,
    ) -> Self::Item {
    }

    fn sh(
        &mut self,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
        _simm: u32,
    ) -> Self::Item {
    }

    fn sw(
        &mut self,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
        _simm: u32,
    ) -> Self::Item {
    }

    fn auipc(&mut self, _rd: arviss::decoding::Reg, _uimm: u32) -> Self::Item {}

    fn lui(&mut self, _rd: arviss::decoding::Reg, _uimm: u32) -> Self::Item {}

    fn jal(&mut self, _rd: arviss::decoding::Reg, jimm: u32) -> Self::Item {
        self.end_block(self.addr);
        self.start_block(self.addr + 4);
        self.start_block((self.addr).wrapping_add(jimm));
    }

    fn add(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
    ) -> Self::Item {
    }

    fn sub(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
    ) -> Self::Item {
    }

    fn sll(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
    ) -> Self::Item {
    }

    fn slt(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
    ) -> Self::Item {
    }

    fn sltu(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
    ) -> Self::Item {
    }

    fn xor(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
    ) -> Self::Item {
    }

    fn srl(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
    ) -> Self::Item {
    }

    fn sra(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
    ) -> Self::Item {
    }

    fn or(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
    ) -> Self::Item {
    }

    fn and(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
    ) -> Self::Item {
    }

    fn slli(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _shamt: u32,
    ) -> Self::Item {
    }

    fn srli(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _shamt: u32,
    ) -> Self::Item {
    }

    fn srai(
        &mut self,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
        _shamt: u32,
    ) -> Self::Item {
    }

    fn fence(
        &mut self,
        _fm: u32,
        _rd: arviss::decoding::Reg,
        _rs1: arviss::decoding::Reg,
    ) -> Self::Item {
    }

    fn ecall(&mut self) -> Self::Item {
        self.end_block(self.addr);
        self.start_block(self.addr + 4);
    }

    fn ebreak(&mut self) -> Self::Item {
        self.end_block(self.addr);
        self.start_block(self.addr + 4);
    }
}

impl<M> HandleRv32c for BlockFinder<M>
where
    M: Memory,
{
    type Item = ();

    fn c_addi4spn(&mut self, _rdp: arviss::decoding::Reg, _imm: u32) -> Self::Item {}

    fn c_lw(
        &mut self,
        _rdp: arviss::decoding::Reg,
        _rs1p: arviss::decoding::Reg,
        _imm: u32,
    ) -> Self::Item {
    }

    fn c_sw(
        &mut self,
        _rs1p: arviss::decoding::Reg,
        _rs2p: arviss::decoding::Reg,
        _imm: u32,
    ) -> Self::Item {
    }

    fn c_sub(
        &mut self,
        _rdrs1p: arviss::decoding::Reg,
        _rs2p: arviss::decoding::Reg,
    ) -> Self::Item {
    }

    fn c_xor(
        &mut self,
        _rdrs1p: arviss::decoding::Reg,
        _rs2p: arviss::decoding::Reg,
    ) -> Self::Item {
    }

    fn c_or(&mut self, _rdrs1p: arviss::decoding::Reg, _rs2p: arviss::decoding::Reg) -> Self::Item {
    }

    fn c_and(
        &mut self,
        _rdrs1p: arviss::decoding::Reg,
        _rs2p: arviss::decoding::Reg,
    ) -> Self::Item {
    }

    fn c_nop(&mut self, _imm: u32) -> Self::Item {}

    fn c_addi16sp(&mut self, _imm: u32) -> Self::Item {}

    fn c_andi(&mut self, _rsrs1p: arviss::decoding::Reg, _imm: u32) -> Self::Item {}

    fn c_addi(&mut self, _rdrs1n0: arviss::decoding::Reg, _imm: u32) -> Self::Item {}

    fn c_li(&mut self, _rd: arviss::decoding::Reg, _imm: u32) -> Self::Item {}

    fn c_lui(&mut self, _rdn2: arviss::decoding::Reg, _imm: u32) -> Self::Item {}

    fn c_j(&mut self, imm: u32) -> Self::Item {
        self.end_block(self.addr);
        self.start_block(self.addr + 2);
        self.start_block((self.addr).wrapping_add(imm));
    }

    fn c_beqz(&mut self, _rs1p: arviss::decoding::Reg, imm: u32) -> Self::Item {
        self.end_block(self.addr);
        self.start_block(self.addr + 2);
        self.start_block(self.addr.wrapping_add(imm));
    }

    fn c_bnez(&mut self, _rs1p: arviss::decoding::Reg, imm: u32) -> Self::Item {
        self.end_block(self.addr);
        self.start_block(self.addr + 2);
        self.start_block(self.addr.wrapping_add(imm));
    }

    fn c_jr(&mut self, _rs1n0: arviss::decoding::Reg) -> Self::Item {
        self.end_block(self.addr);
        self.start_block(self.addr + 2);
    }

    fn c_jalr(&mut self, _rs1n0: arviss::decoding::Reg) -> Self::Item {
        self.end_block(self.addr);
        self.start_block(self.addr + 2);
    }

    fn c_ebreak(&mut self) -> Self::Item {
        self.end_block(self.addr);
        self.start_block(self.addr + 2);
    }

    fn c_mv(&mut self, _rd: arviss::decoding::Reg, _rs2n0: arviss::decoding::Reg) -> Self::Item {}

    fn c_add(
        &mut self,
        _rdrs1: arviss::decoding::Reg,
        _rs2n0: arviss::decoding::Reg,
    ) -> Self::Item {
    }

    fn c_lwsp(&mut self, _rdn0: arviss::decoding::Reg, _imm: u32) -> Self::Item {}

    fn c_swsp(&mut self, _rs2: arviss::decoding::Reg, _imm: u32) -> Self::Item {}

    fn c_jal(&mut self, _imm: u32) -> Self::Item {
        todo!()
    }

    fn c_srli(&mut self, _rdrs1p: arviss::decoding::Reg, _imm: u32) -> Self::Item {}

    fn c_srai(&mut self, _rdrs1p: arviss::decoding::Reg, _imm: u32) -> Self::Item {}

    fn c_slli(&mut self, _rdrs1n0: arviss::decoding::Reg, _imm: u32) -> Self::Item {}
}

pub fn main() {
    // Load the image into a buffer.
    let mut f = File::open("images/hello_world.rv32ic").expect("Failed to open image."); // TODO: Lose the expect and handle the error.
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("Failed to load image."); // TODO: Lose the expect and handle the error.

    // Copy the image into memory.
    let mut mem = BasicMem::new();
    let image = buffer.as_slice();
    mem.write_bytes(0, image)
        .expect("Failed to initialize memory.");

    // Find the basic blocks in the image.
    let text_size = buffer.len() - 4; // TODO: The image needs to tell us how big its text and initialized data are.

    let mut block_finder = BlockFinder::<BasicMem>::with_mem(mem, text_size);
    block_finder.run(0);

    // Disassemble each block.
    let mut dis = Disassembler;
    println!("addr     instr    code");
    for block in &block_finder.known_blocks {
        println!(
            "; --------------- Basic block: {:08x} - {:08x}",
            block.start, block.end
        );
        let mut addr = block.start;
        while addr <= block.end {
            let ins = block_finder.mem.read32(addr).unwrap(); // TODO: lose the unwrap and handle the error.
            let code = dis.dispatch(ins);
            let is_compact = (ins & 3) != 3;
            if is_compact {
                // Compact instructions are 2 bytes each.
                println!("{:08x}     {:04x} {}", addr, ins & 0xffff, code);
                addr += 2;
            } else {
                // Regular instructions are 4 bytes each.
                println!("{:08x} {:08x} {}", addr, ins, code);
                addr += 4;
            }
        }
    }
}
