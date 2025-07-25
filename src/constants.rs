use std::io;
use colored::*;
const CORRECT_ANSWER: &str = "const SECONDS_IN_MINUTE: u32 = 60;";

pub fn run_constants()
{   
    let keyword = "const".red();
    println!("---BEGINNING CONSTANTS PRACTICE---\n");
    println!("constants are defined by the keyword {keyword}\n");
    println!("here is an example of a constant\n");
    println!("{keyword} FOOTBALL_FIELD_LENGTH: u32 = 140;\n");
    println!("now you try.\n");
    loop{
        let mut input = String::new();
        println!("define a constant named SECONDS_IN_MINUTE");
        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let input = input.trim();
        if input == CORRECT_ANSWER {
            println!("correct!");
                break;
        } else {
            println!("incorrect. try again");
        }
    }

    println!("press enter to continue");
    let mut dummy = String::new();
    io::stdin()
            .read_line(&mut dummy)
            .unwrap();

}
