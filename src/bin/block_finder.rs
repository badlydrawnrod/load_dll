use std::fs::File;
use std::io::prelude::*;
use std::ops::{Index, IndexMut};

use arviss::{Address, DispatchRv32i, HandleRv32i, MemoryResult};

use arviss::backends::memory::basic::*;

struct Block {
    start: Address,
    end: Address,
}

impl Block {
    fn new(start: Address) -> Self {
        Block {
            start,
            end: 0,
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
        Self {
            addr: 0,
            mem,
            eom: len as Address, // TODO: this assumes starting at zero - not necessarily the case.
            known_blocks: Vec::new(),
            open_blocks: Vec::new(),
            current_block: 0,
        }
    }

    fn addr(&self) -> Address {
        self.addr
    }

    fn next(&mut self) -> MemoryResult<u32> {
        let result = self.mem.read32(self.addr);
        self.addr = self.addr.wrapping_add(4);
        result
    }

    fn start_block(&mut self, addr: Address) {
        if addr >= self.eom {
            println!(
                "not adding block at: {:08x} as it's off the end of the image",
                addr
            );
            return;
        }
        let is_known = self.known_blocks.iter().any(|b| b.start == addr);
        // TODO: what if it splits a known block?
        if !is_known {
            println!("starting block at: {:08x}", addr);
            self.known_blocks.push(Block::new(addr));
            let index = self.known_blocks.len() - 1;
            self.open_blocks.push(index);
        }
    }

    fn end_block(&mut self, addr: Address) {
        println!("ending block at: {:08x}", addr);
        let block = self.known_blocks.index_mut(self.current_block);
        block.end = addr;
    }

    fn run(&mut self, addr: Address) {
        self.start_block(addr);
        while !self.open_blocks.is_empty() {
            self.current_block = self.open_blocks.pop().unwrap();
            println!("Current block is now {}", self.current_block);
            let block = self.known_blocks.index(self.current_block);
            self.addr = block.start.wrapping_add(4);
            loop {
                let addr = self.addr();
                if addr >= self.eom {
                    break;
                }
                // print!("0x{:08x} (eom = 0x{:08x})", addr, self.eom);
                let ins = self.next().unwrap();
                self.dispatch(ins);
                let block = self.known_blocks.index(self.current_block);
                if block.end != 0 {
                    break;
                }
            }
        }
    }
}

impl<M> HandleRv32i for BlockFinder<M>
where
    M: Memory,
{
    type Item = ();

    fn illegal(&mut self, _ins: u32) -> Self::Item {
        println!("Illegal instruction!");
    }

    fn beq(
        &mut self,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        println!("beq");
        self.end_block(self.addr - 4); // TODO: traditional fetch?
        self.start_block(self.addr);
        self.start_block((self.addr - 4).wrapping_add(bimm));
    }

    fn bne(
        &mut self,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        println!("bne");
        self.end_block(self.addr - 4); // TODO: traditional fetch?
        self.start_block(self.addr);
        self.start_block((self.addr - 4).wrapping_add(bimm));
    }

    fn blt(
        &mut self,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        println!("blt");
        self.end_block(self.addr - 4); // TODO: traditional fetch?
        self.start_block(self.addr);
        self.start_block((self.addr - 4).wrapping_add(bimm));
    }

    fn bge(
        &mut self,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        println!("bge");
        self.end_block(self.addr - 4); // TODO: traditional fetch?
        self.start_block(self.addr);
        self.start_block((self.addr - 4).wrapping_add(bimm));
    }

    fn bltu(
        &mut self,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        println!("bltu");
        self.end_block(self.addr - 4); // TODO: traditional fetch?
        self.start_block(self.addr);
        self.start_block((self.addr - 4).wrapping_add(bimm));
    }

    fn bgeu(
        &mut self,
        _rs1: arviss::decoding::Reg,
        _rs2: arviss::decoding::Reg,
        bimm: u32,
    ) -> Self::Item {
        println!("bgeu");
        self.end_block(self.addr - 4); // TODO: traditional fetch?
        self.start_block(self.addr);
        self.start_block((self.addr - 4).wrapping_add(bimm));
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
        println!("jalr");
        self.end_block(self.addr - 4); // TODO: traditional fetch?
        self.start_block(self.addr);
        // TODO: handle branch ... except that it's indirect so we have no idea.
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
        println!("jal");
        self.end_block(self.addr - 4); // TODO: traditional fetch?
        self.start_block(self.addr);
        self.start_block((self.addr - 4).wrapping_add(jimm));
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
        println!("ecall");
        self.end_block(self.addr - 4); // TODO: traditional fetch?
        self.start_block(self.addr);
    }

    fn ebreak(&mut self) -> Self::Item {
        println!("ebreak");
        self.end_block(self.addr - 4); // TODO: traditional fetch?
        self.start_block(self.addr);
    }
}

pub fn main() {
    // Load the image into a buffer.
    let mut f = File::open("images/hello_world.rv32i").expect("Failed to open image.");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("Failed to load image.");

    // Copy the image into memory.
    let mut mem = BasicMem::new();
    let image = buffer.as_slice();
    mem.write_bytes(0, image)
        .expect("Failed to initialize memory.");

    // Find the blocks in the image.
    let mut block_finder = BlockFinder::<BasicMem>::with_mem(mem, buffer.len());

    block_finder.run(0);
    // loop {
    //     let addr = block_finder.addr();
    //     let ins = block_finder.next().unwrap();
    //     print!("0x{:08x} ", addr);
    //     if addr == 0 {
    //         println!("START");
    //         block_finder.start_block(addr); // TODO: traditional fetch?
    //     }
    //     block_finder.dispatch(ins);
    // }
}
