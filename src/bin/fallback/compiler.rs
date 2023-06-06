use arviss::platforms::basic::*;
use arviss::Address;
use libloading::{Library, Symbol};
use load_dll::block_finder::*;
use load_dll::block_writer::*;
use std::collections::HashMap;
use std::fs::File;
use std::process::Command;
use tempdir::TempDir;

pub type Cpu = Rv32iCpu<BasicMem>;
pub type ArvissFunc = extern "C" fn(&mut Cpu);

pub struct Compiler {
    temp_dir: TempDir,
    libs: Vec<Library>,
    block_map: HashMap<Address, ArvissFunc>,
}

impl Compiler {
    pub fn new(dir: TempDir) -> Self {
        Self {
            temp_dir: dir,
            libs: Vec::new(),
            block_map: HashMap::new(),
        }
    }

    pub fn get(&self, addr: Address) -> Option<&ArvissFunc> {
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

        // Create a file in the temporary directory.
        let file_path = self.temp_dir.path().join("demo.rs");
        let Ok(mut f) = File::create(file_path) else {
            eprintln!("Failed to create file");
            std::process::exit(1);
        };

        // Generate a Rust module containing source for each basic block.
        let mut block_writer = BlockWriter::new(image);
        if let Err(err) = block_writer.write_blocks(&mut f, &blocks) {
            eprintln!("Failed to write blocks: {err}");
            std::process::exit(1);
        }

        if let Err(err) = f.sync_all() {
            eprintln!("Failed to sync: {err}");
            std::process::exit(1);
        }

        // Compile the module to a .so.
        let filename = self
            .temp_dir
            .path()
            .join("demo.rs")
            .to_string_lossy()
            .to_string();
        let mut command = Command::new("rustc");
        let Ok(run) = command
            .current_dir(self.temp_dir.path())
            .arg("--edition=2021")
            .arg("--crate-type")
            .arg("cdylib")
            .arg("--extern")
            // .arg("arviss=/home/rod/projects/learn_rust/100days/load_dll/target/release/deps/libarviss-3f92a38f6024ae90.rlib")
            .arg("arviss=/home/rod/projects/learn_rust/100days/load_dll/target/debug/deps/libarviss-8804a9930697a36b.rlib")
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

        // Load the library.
        let library_path = self.temp_dir.path().join("libdemo.so");
        let lib = unsafe { Library::new(library_path).unwrap() };

        // Load the functions from the library.
        let block_map = unsafe {
            let mut block_map = HashMap::new();
            for (index, block) in blocks.iter().enumerate() {
                // Deliberately skip blocks.
                let symbol = format!("block_{:08x}_{:08x}", block.start, block.end);
                if index % 16 != 15 {
                    let basic_block_fn: Symbol<ArvissFunc> = lib.get(symbol.as_bytes()).unwrap();
                    let basic_block_fn = *basic_block_fn;
                    block_map.insert(block.start, basic_block_fn);
                }
            }
            block_map
        };

        // The compiler owns the library and the mappings.
        self.block_map.extend(block_map);
        self.libs.push(lib);
    }
}
