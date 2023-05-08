type BigComplicatedCalculationFunc = extern "C" fn(i64, i64) -> i64;

#[no_mangle]
pub extern "C" fn big_complicated_calculation(a: i64, b: i64) -> i64 {
    a + b
}

#[no_mangle]
pub extern "C" fn return_another_function() -> BigComplicatedCalculationFunc {
    big_complicated_calculation
}
