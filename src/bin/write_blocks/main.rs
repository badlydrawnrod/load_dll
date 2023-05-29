use arviss::platforms::basic::*;
use arviss::Address;
use libloading::{Library, Symbol};
use load_dll::block_finder::*;
use load_dll::block_writer::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::Command;
use tempdir::TempDir;

pub fn main() {
    // Load the image into a buffer.
    let path = "images/hello_world.rv32ic";
    let Ok(file_data) = std::fs::read(path) else {
        eprintln!("Failed to read file: `{}`", path);
        std::process::exit(1);
    };
    let image = file_data.as_slice();

    // Copy the image into memory.
    let mut mem = BasicMem::new();
    if let Err(addr) = mem.write_bytes(0, image) {
        eprintln!("Failed to initialize memory at: 0x{:08x}", addr);
        std::process::exit(1);
    };

    // Find the basic blocks in the image.
    let text_size = image.len() - 4; // TODO: The image needs to tell us how big its text and initialized data are.
    let mut block_finder = BlockFinder::<BasicMem>::with_mem(&mem, text_size);
    let blocks = match block_finder.find_blocks(0) {
        Ok(blocks) => blocks,
        Err(err) => {
            eprintln!("ERROR: {}", err);
            std::process::exit(1);
        }
    };

    // Open a temporary directory that will be cleaned up at the end.
    let Ok(dir) = TempDir::new("rhtest") else {
        eprintln!("Failed to create temporary directory");
        std::process::exit(1);
    };

    // Create a file in that directory.
    let file_path = dir.path().join("demo.rs");
    let Ok(mut f) = File::create(file_path) else {
        eprintln!("Failed to create file");
        std::process::exit(1);
    };

    // Generate a Rust module containing source for each basic block.
    let mut block_writer = BlockWriter::new(&mem);
    if let Err(err) = block_writer.write_blocks(&mut f, &blocks) {
        eprintln!("Failed to write blocks: {err}");
        std::process::exit(1);
    }

    if let Err(err) = f.sync_all() {
        eprintln!("Failed to sync: {err}");
        std::process::exit(1);
    }

    // Compile the module to a .so.
    let filename = dir.path().join("demo.rs").to_string_lossy().to_string();
    let mut command = Command::new("rustc");
    let Ok(run) = command
        .current_dir(dir.path())
        .arg("--edition=2021")
        .arg("--crate-type")
        .arg("cdylib")
        .arg("--extern")
        .arg("arviss=/home/rod/projects/learn_rust/100days/load_dll/target/release/deps/libarviss-3f92a38f6024ae90.rlib")
        // .arg("arviss=/home/rod/projects/learn_rust/100days/load_dll/target/debug/deps/libarviss-fa3eb26a5be62bea.rlib")
        .arg("-C")
        .arg("opt-level=2")
        .arg("-C")
        .arg("strip=symbols")
        .arg(filename)
        .status() else {
            eprintln!("Failed to compile");
            std::process::exit(1);
        };
    assert!(run.success());

    // Create a simulator.
    type Cpu = Rv32iCpu<BasicMem>;
    type ArvissFunc = extern "C" fn(&mut Cpu);

    let mut cpu = Cpu::with_mem(mem);

    // Load the library.
    let library_path = dir.path().join("libdemo.so");

    unsafe {
        let lib = Library::new(library_path).unwrap();

        // Load the functions that we previously generated and put them in a map indexed by start address.
        let mut block_map = HashMap::new();
        for block in blocks {
            let symbol = format!("block_{:08x}_{:08x}", block.start, block.end);
            let run_one: Symbol<ArvissFunc> = lib.get(symbol.as_bytes()).unwrap();
            block_map.insert(block.start, run_one);
        }

        // Run the compiled code that we loaded from the DLL against our simulator.
        let mut addr: Address = 0;
        while !cpu.is_trapped() {
            // println!("cpu.pc = 0x{:08x}", addr);
            let run_one = block_map.get(&addr).unwrap();
            run_one(&mut cpu);
            addr = cpu.transfer();
        }
        println!("Trapped at 0x{:08x}", addr);
    }

    // Give the user (me) an opportunity to disassemble the binary.
    println!(
        "Look in {:?} for the generated code and library",
        dir.path()
    );
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}
