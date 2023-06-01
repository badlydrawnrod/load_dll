use crate::read_instruction::*;
use arviss::{Address, DispatchRv32ic, HandleRv32c, HandleRv32i};
use std::ops::{Index, IndexMut};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Block {
    pub start: Address, // Address of the first instruction in the basic block.
    pub end: Address, // Address of the instruction following the last instruction in the basic block.
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

pub struct BlockFinder<'a> {
    addr: Address,
    mem: &'a [u8],
    known_blocks: Vec<Block>,
    open_blocks: Vec<usize>,
    current_block: usize,
}

#[derive(Error, Debug)]
pub enum BlockFinderError {
    #[error("memory read failed at 0x{addr:08x}")]
    MemoryReadFailed { addr: Address },
}

impl<'a> BlockFinder<'a> {
    pub fn with_mem(mem: &'a [u8]) -> Self {
        Self {
            addr: 0,
            mem,
            known_blocks: Vec::new(),
            open_blocks: Vec::new(),
            current_block: 0,
        }
    }

    #[inline]
    fn next_instruction(&mut self) -> Result<u32, BlockFinderError> {
        read_instruction(self.mem, self.addr)
            .map_err(|addr| BlockFinderError::MemoryReadFailed { addr })
    }

    fn start_block(&mut self, addr: Address) {
        // Ignore addresses that are outside of the address range.
        if addr as usize >= self.mem.len() {
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
                .find(|b| b.start < addr && addr < b.end);
            if let Some(block) = splits_block {
                block.end = addr;
            }
        }
    }

    fn end_block(&mut self, addr: Address) {
        let block = self.known_blocks.index_mut(self.current_block);
        block.end = addr;
    }

    pub fn find_blocks(&mut self, addr: Address) -> Result<Vec<Block>, BlockFinderError> {
        self.start_block(addr);
        while let Some(current_block) = self.open_blocks.pop() {
            self.current_block = current_block;
            let mut block = self.known_blocks.index(self.current_block);
            self.addr = block.start;
            while (self.addr as usize) < self.mem.len() && block.end == OPEN_BLOCK_SENTINEL {
                let ins = self.next_instruction()?;
                self.dispatch(ins);
                let instruction_size = if (ins & 3) == 3 { 4 } else { 2 };
                self.addr = self.addr.wrapping_add(instruction_size);
                block = self.known_blocks.index(self.current_block);
            }
        }
        self.known_blocks.sort_unstable();
        Ok(std::mem::take(&mut self.known_blocks))
    }

    fn conditional_jump(&mut self, branch_taken: Address, branch_not_taken: Address) {
        self.end_block(branch_not_taken);
        self.start_block(branch_not_taken);
        self.start_block(branch_taken);
    }

    fn direct_jump(&mut self, next_instruction: Address, target: Address) {
        self.end_block(next_instruction);
        self.start_block(target);
    }

    fn new_block(&mut self, next_instruction: Address) {
        self.end_block(next_instruction);
        self.start_block(next_instruction);
    }
}

impl HandleRv32i for BlockFinder<'_> {
    type Item = ();

    fn illegal(&mut self, _ins: u32) -> Self::Item {}

    fn beq(
        &mut self,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        self.conditional_jump(self.addr.wrapping_add(bimm), self.addr + 4);
    }

    fn bne(
        &mut self,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        self.conditional_jump(self.addr.wrapping_add(bimm), self.addr + 4);
    }

    fn blt(
        &mut self,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        self.conditional_jump(self.addr.wrapping_add(bimm), self.addr + 4);
    }

    fn bge(
        &mut self,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        self.conditional_jump(self.addr.wrapping_add(bimm), self.addr + 4);
    }

    fn bltu(
        &mut self,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        self.conditional_jump(self.addr.wrapping_add(bimm), self.addr + 4);
    }

    fn bgeu(
        &mut self,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        self.conditional_jump(self.addr.wrapping_add(bimm), self.addr + 4);
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
        self.new_block(self.addr + 4);
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
        self.direct_jump(self.addr + 4, self.addr.wrapping_add(jimm));
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
        self.new_block(self.addr + 4);
    }

    fn ebreak(&mut self) -> Self::Item {
        self.new_block(self.addr + 4);
    }
}

impl HandleRv32c for BlockFinder<'_> {
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
        self.direct_jump(self.addr + 2, self.addr.wrapping_add(imm));
    }

    fn c_beqz(&mut self, _rs1p: arviss::decoding::Reg, imm: u32) -> Self::Item {
        self.conditional_jump(self.addr.wrapping_add(imm), self.addr + 2);
    }

    fn c_bnez(&mut self, _rs1p: arviss::decoding::Reg, imm: u32) -> Self::Item {
        self.conditional_jump(self.addr.wrapping_add(imm), self.addr + 2);
    }

    fn c_jr(&mut self, _rs1n0: arviss::decoding::Reg) -> Self::Item {
        self.new_block(self.addr + 2);
    }

    fn c_jalr(&mut self, _rs1n0: arviss::decoding::Reg) -> Self::Item {
        self.new_block(self.addr + 2);
    }

    fn c_ebreak(&mut self) -> Self::Item {
        self.new_block(self.addr + 2);
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

    fn c_jal(&mut self, imm: u32) -> Self::Item {
        self.direct_jump(self.addr + 2, self.addr.wrapping_add(imm));
    }

    fn c_srli(&mut self, _rdrs1p: arviss::decoding::Reg, _imm: u32) -> Self::Item {}

    fn c_srai(&mut self, _rdrs1p: arviss::decoding::Reg, _imm: u32) -> Self::Item {}

    fn c_slli(&mut self, _rdrs1n0: arviss::decoding::Reg, _imm: u32) -> Self::Item {}
}
