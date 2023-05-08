use libloading::{Library, Symbol};
use std::env;

type BigComplicatedCalculationFunc = extern "C" fn(i64, i64) -> i64;
type FuncReturningFunc = extern "C" fn() -> BigComplicatedCalculationFunc;

pub fn main() {
    let library_path = env::args().nth(1).expect("USAGE: loader <dll/.so>");
    println!("Loading {}", library_path);
    unsafe {
        let lib = Library::new(library_path).unwrap();
        let add_func: Symbol<BigComplicatedCalculationFunc> =
            lib.get(b"big_complicated_calculation").unwrap();
        println!("big_complicated_calculation(2, 2) = {}", add_func(2, 2));

        let func_returning_func: Symbol<FuncReturningFunc> =
            lib.get(b"return_another_function").unwrap();
        let func_to_call = func_returning_func();
        println!("func_to_call(3, 5) = {}", func_to_call(3, 5));
    }
}
