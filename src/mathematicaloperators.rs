use std::io;
use colored::*;
    
pub fn run_mathematicaloperators()
{
    let dtypes = (
        "i8".red(),
        "i32".red(),
        "i64".red(),
        "i128".red(),
        "u8".red(),
        "u128".red(),
        "f32".red(),
        "f64".red(),
        "isize".red(),
        "usize".red(),
    );
    println!("---BEGINNING MATHEMATICAL OPERATORS PRACTICE---\n");
    println!("integer types\n
    ____________________\n
    {}, {}, {} ... {} are all signed integer types\n
    {} -> {} are all unsigned (only positive)\n
    {} and {} default to the computer's architecture size\n\n
    ____________________\n
    floating point types\n
    {} and {} are the only two floating point types\n\n", 
    dtypes.0,
    dtypes.1,
    dtypes.2,
    dtypes.3,
    dtypes.4,
    dtypes.5,
    dtypes.8,
    dtypes.9,
    dtypes.6,
    dtypes.7,
    );
}
