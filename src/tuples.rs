use std::io;
use colored::*;

pub fn run_tuples()
{
    const CORRECT: (&str, &str, &str) =
        (
            "let tup: (i32, f64, u8) = (500, 6.4, 1);",
            "let tup = (500, 6.4, 1);",
            "tup.1",
        );
    let kw = (
        "tuple".red(),
    );
    println!("---BEGINNING TUPLES PRACTICE---");
    println!("
        a {} can be created like this 
        let tup: (i32, f64, u8) = (500, 6.4, 1);\n

        a {} can become bound to a variable this way.
        the first line creates a {} which is a different types of values
        the following line binds them to the name x, y, and z\n
        
        let tup = (500, 6.4, 1);
        let (x, y, z) = tup;\n

        access tuples with a '.' operator

        printlng tup.0 will print the first index of the tuple named tup.
    ",kw.0, kw.0, kw.0);
    loop{
        let mut input = String::new();
        println!("write a tuple");
        io::stdin()
            .read_line(&mut input)
            .unwrap();
        let input = input.trim();
        if input == CORRECT.0{
            println!("CORRECT!");
            break;
        } else {
            println!("incorrect, try again");
        }    
    }
    loop{
        let mut input = String::new();
        println!("bind each value in the tuple to a seperate value");
        io::stdin()
            .read_line(&mut input)
            .unwrap();
        let input = input.trim();
        if input == CORRECT.1{
            println!("CORRECT!");
            break;
        } else {
            println!("incorrect, try again.");
        }
    }
    loop{
        let mut input = String::new();
        println!("index to a tuple at index 1");
        io::stdin()
            .read_line(&mut input)
            .unwrap();
        let input = input.trim();
        if input == CORRECT.2 {
            println!("CORRECT!");
            break;
        } else {
            println!("incorrect, try again.");
        }
    }
}
