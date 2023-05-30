use arviss::backends::memory::basic::*;
use arviss::{disassembler::Disassembler, DispatchRv32ic};

use load_dll::block_finder::*;

pub fn main() {
    // Load the image into a buffer.
    let path = "images/hello_world.rv32ic";
    let Ok(file_data) = std::fs::read(path) else {
        eprintln!("Failed to read file: `{}`", path);
        std::process::exit(1);
    };
    let image = file_data.as_slice();

    // Find the basic blocks in the image.
    let text_size = image.len() - 4; // TODO: The image needs to tell us how big its text and initialized data are.
    let mut block_finder = BlockFinder::with_mem(&image[..text_size]);
    let blocks = match block_finder.find_blocks(0) {
        Ok(blocks) => blocks,
        Err(err) => {
            eprintln!("ERROR: {}", err);
            std::process::exit(1);
        }
    };

    // Copy the image into memory.
    let mut mem = BasicMem::new();
    if let Err(addr) = mem.write_bytes(0, image) {
        eprintln!("Failed to initialize memory at: 0x{:08x}", addr);
        std::process::exit(1);
    };

    // Disassemble each block for visual evidence that it's working.
    let mut dis = Disassembler;
    println!("addr     instr    code");
    for block in blocks {
        println!(
            "; --------------- Basic block: {:08x} - {:08x}",
            block.start, block.end
        );
        let mut addr = block.start;
        while addr < block.end {
            let Ok(ins) = mem.read32(addr) else {
                eprintln!("Failed to read memory when disassembling 0x{:08x}", addr);
                std::process::exit(1);
            };
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
