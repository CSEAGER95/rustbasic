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
        "decimal".blue(),
        "hex".blue(),

        "octal".blue(),
        "binary".blue(),
        "byte".blue(),
        "0x".green(),

        "0o".green(),
        "0b".green(),
        "b".green(),
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

    println!("
        {}              98_222 numbers can have underscores as a visual seperator. 98_222 is the same as 98222\n
        {}	        {}ff prepend hexadeciamal numbers with 0x\n
        {}	        {}77 prepend octal numbers with 0o\n
        {}	        {}1111_0000 prepend binary numbers with 0b\n
        {} (u8 only)	{}'A' prepend Byte with b. this displays the byte value of the ascii character 'A'",
        dtypes.10,
        dtypes.11,
        dtypes.15,
        dtypes.12,
        dtypes.16,
        dtypes.13,
        dtypes.17,
        dtypes.14,
        dtypes.18,
    );
    println!("unless explicitly mentioned rust defaults to a {dtypes.7} decimal.\n");
    println!(
        "let x = 2.0; // f64

        let y: {dtypes.6} = 3.0; // f32
        ");
}
