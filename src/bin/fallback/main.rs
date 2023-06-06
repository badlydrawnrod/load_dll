mod compiler;

use compiler::*;

use arviss::Address;
use arviss::{platforms::basic::*, DispatchRv32ic};
use std::io::{self, BufRead};
use tempdir::TempDir;

pub fn main() {
    // Open a temporary directory that will be cleaned up at the end.
    let Ok(dir) = TempDir::new("rhtest") else {
        eprintln!("Failed to create temporary directory");
        std::process::exit(1);
    };
    println!(
        "Look in {:?} for the generated code and library",
        dir.path()
    );

    // Create the compiler.
    let mut compiler = Compiler::new(dir);

    // Load the image into a buffer and compile it.
    let path = "images/hello_world.rv32ic";
    let Ok(file_data) = std::fs::read(path) else {
        eprintln!("Failed to read file: `{}`", path);
        std::process::exit(1);
    };
    let image = file_data.as_slice();
    let text_size = image.len() - 4; // TODO: The image needs to tell us how big its text and initialized data are.
    compiler.compile(&image[0..text_size]);

    // Copy the image into simulator memory.
    let mut mem = BasicMem::new();
    if let Err(addr) = mem.write_bytes(0, image) {
        eprintln!("Failed to initialize memory at: 0x{:08x}", addr);
        std::process::exit(1);
    };

    // TODO: What if we have multiple images?

    // Create a simulator and run it by calling the compiled functions.
    let mut addr: Address = 0;
    let mut cpu = Cpu::with_mem(mem);

    // TODO: there's a bug somewhere in this loop that causes it to skip instructions when switching between native
    // code and interpreting, or vice versa. Probably the latter but I haven't spotted it yet. Disassembly may help.
    while !cpu.is_trapped() {
        // Fall back to interpreting if we can't find a basic block in the map.
        match compiler.get(addr) {
            Some(func) => {
                func(&mut cpu);
                addr = cpu.transfer();
            }
            None => {
                // println!("Fallback at: 0x{:08x}", addr);

                while !cpu.is_trapped() {
                    // Fetch.
                    let ins = cpu.fetch().unwrap();
                    if let Some(_) = compiler.get(cpu.pc()) {
                        addr = cpu.pc();
                        // println!("  Native at: 0x{:08x}", addr);
                        break;
                    }

                    // Decode and dispatch.
                    cpu.dispatch(ins);
                }
            }
        };
    }

    match cpu.trap_cause() {
        Some(TrapCause::Breakpoint) => {
            println!("Simulation terminated successfully")
        }
        Some(cause) => println!("{:?} at 0x{:08x}", cause, cpu.pc()),
        None => unreachable!(),
    }

    // Give the user (me) an opportunity to disassemble the binary.
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}
