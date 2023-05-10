use arviss::decoding::Reg;
use arviss::platforms::basic::*;
use libloading::{Library, Symbol};
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::process::Command;
use tempdir::TempDir;

// Each basic block returns a function with the same signature as itself.
pub type ArvissFn = extern "C" fn(&mut Rv32iCpu<BasicMem>) -> ArvissFnS;

// But that would lead to infinite recursion in the type system, so break the cycle with a newtype.
#[repr(transparent)]
pub struct ArvissFnS(pub ArvissFn);

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
        "pub type ArvissFn = extern \"C\" fn(&mut Rv32iCpu<BasicMem>) -> ArvissFnS;\n",
        "#[repr(transparent)]\n",
        "pub struct ArvissFnS(ArvissFn);\n",
        "\n",
        "#[no_mangle]\n",
        "pub extern \"C\" fn run_one(cpu: &mut Rv32iCpu::<BasicMem>) -> ArvissFnS {\n",
        "    cpu.add(Reg::from(1), Reg::from(1), Reg::from(2));\n",
        "    ArvissFnS(run_one)\n",
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

    // Create a simulator.
    let mut cpu = Rv32iCpu::<BasicMem>::new();
    cpu.wx(Reg::from(1), 0);
    cpu.wx(Reg::from(2), 1);

    let library_path = dir.path().join("libdemo.so");
    println!(
        "Look in {:?} for the generated code and library",
        dir.path()
    );

    // Load the library.
    let lib = unsafe {
        let lib = Library::new(library_path).unwrap();
        lib
    };

    // Get the function to call, i.e., the compiled code.
    let mut func = unsafe {
        let run_one: Symbol<ArvissFn> = lib.get(b"run_one").unwrap();
        *run_one // Dereference because `run_one` is a `Symbol<ArvissFn>` but we just want an `ArvissFn`.
    };

    // Run the compiled code that we loaded from the DLL against our simulator a few times.
    println!("Before: {}", cpu.rx(Reg::from(1)));
    for _ in 0..10 {
        let result = func(&mut cpu);
        func = result.0;
    }
    println!(" After: {}", cpu.rx(Reg::from(1)));
    assert_eq!(10, cpu.rx(Reg::from(1)));

    // Give the user (me) an opportunity to disassemble the binary.
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }

    Ok(())
}
