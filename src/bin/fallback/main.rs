mod fallback_compiler;

use fallback_compiler::*;

use arviss::Address;
use arviss::{platforms::basic::*, DispatchRv32ic};
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

    // Create the compiler. This particular compiler does not load everything as we're using it to test falling back to
    // interpreting.
    let mut compiler = FallbackCompiler::new(dir);

    // Load the image into a buffer and compile it.
    let path = "images/hello_world.rv32ic";
    let Ok(file_data) = std::fs::read(path) else {
        eprintln!("Failed to read file: `{}`", path);
        std::process::exit(1);
    };
    let image = file_data.as_slice();
    let text_size = image.len() - 4;

    compiler.compile(&image[0..text_size]);

    // Copy the image into simulator memory.
    let mut mem = BasicMem::new();
    if let Err(addr) = mem.write_bytes(0, image) {
        eprintln!("Failed to initialize memory at: 0x{:08x}", addr);
        std::process::exit(1);
    };

    // Create a simulator and run it by calling the compiled functions, falling back to interpreting when we don't know
    // about the given basic block.
    let mut addr: Address = 0;
    let mut cpu = Cpu::with_mem(mem);

    while !cpu.is_trapped() {
        match compiler.get(addr) {
            // Basic block found. Call the native code.
            Some(func) => {
                func(&mut cpu);
                addr = cpu.transfer();
            }
            // Basic block not found. Fall back to interpreting.
            None => {
                while !cpu.is_trapped() {
                    // Fetch.
                    let ins = cpu.fetch().unwrap();
                    if compiler.get(cpu.pc()).is_some() {
                        addr = cpu.pc();
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
}
