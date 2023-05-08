use libloading::{Library, Symbol};
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::process::Command;
use tempdir::TempDir;

type BigComplicatedCalculationFunc = extern "C" fn(i64, i64) -> i64;

pub fn main() -> Result<(), io::Error> {
    // Open a temporary directory that will be cleaned up at the end.
    let dir = TempDir::new("rhtest")?;

    // Create a Rust source file in that temporary directory.
    let text = concat!(
        "use arviss;\n",
        "use arviss::HandleRv32i;\n",
        "use arviss::platforms::basic::*;\n",
        "use arviss::decoding::Reg;\n",
        "\n",
        "#[no_mangle]\n",
        "pub extern \"C\" fn big_complicated_calculation(a: i64, b: i64) -> i64 {\n",
        "    a + b\n",
        "}\n",
        "\n",
        "#[no_mangle]\n",
        "pub extern \"C\" fn run_one(cpu: &mut Rv32iCpu::<BasicMem>) {\n",
        "    cpu.add(Reg::from(1), Reg::from(2), Reg::from(3));\n",
        "    cpu.add(Reg::from(2), Reg::from(1), Reg::from(3));\n",
        "    cpu.add(Reg::from(3), Reg::from(1), Reg::from(2));\n",
        "}\n"
    );
    let file_path = dir.path().join("demo.rs");
    println!("{:?}", file_path);
    let mut f = File::create(file_path)?;
    writeln!(f, "{}", text)?;
    f.sync_all()?;

    // Compile it to a .so.
    let filename = dir.path().join("demo.rs").to_string_lossy().to_string();
    let mut command = Command::new("rustc");
    let run = command
        .current_dir(dir.path())
        .arg("--edition=2021")
        .arg("--crate-type")
        .arg("cdylib")
        .arg("--extern")
        .arg("arviss=/home/rod/projects/learn_rust/100days/load_dll/target/release/deps/libarviss-3f92a38f6024ae90.rlib")
        .arg("-C")
        .arg("opt-level=2")
        .arg("-C")
        .arg("strip=debuginfo")
        .arg(filename)
        .status()?;
    assert!(run.success());

    
    // Load the library and call a function in it.
    let library_path = dir.path().join("libdemo.so");
    println!("Look in {:?} for the output", library_path);
    unsafe {
        let lib = Library::new(library_path).unwrap();
        let add_func: Symbol<BigComplicatedCalculationFunc> =
            lib.get(b"big_complicated_calculation").unwrap();
        println!("big_complicated_calculation(2, 2) = {}", add_func(2, 2));
    }

    // Give the user (me) an opportunity to disassemble the binary.
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }

    Ok(())
}
