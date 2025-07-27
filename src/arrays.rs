use std::io;

pub fn run_arrays()
{
    const CORRECT: [&str; 2] = [
        "let a: [u32; 3] = [2, 4, 6];",
        "let a = [5; 4];"
    ];
    println!("---BEGINNING ARRAYS PRACTICE---");

    println!("
        there are three ways to create arrays arrays
        
        let a = [1, 2, 3, 4, 5];

        let a: [i32; 5] = [1, 2, 3, 4, 5];

        let a = [0; 5];

        the second version defines the type and the size of the array,

        the third array defines the value to be duplicated and then the length of the array
        [0, 0, 0, 0, 0,]

        accessing elements in an array is basic. a[0]
        \n
    ");
    loop{
        let mut input = String::new();
        println!("create an array of type u32 and size 3 of all the eve numbers between 1 and 7");
        io::stdin()
            .read_line(&mut input)
            .unwrap();
        let input = input.trim();
        if input == CORRECT[0]{
            println!("CORRECT");
            break;
        } else {    
            println!("incorrect, try again.");
        }
    }
    loop{
        let mut input = String::new();
        println!("creat an array of size 4 with all 5s");
        io::stdin()
            .read_line(&mut input)
            .unwrap();
        let input = input.trim();
        if input == CORRECT[1]{
            println!("CORRECT");
            break;
        } else {
            println!("incorrect, try again.");
        }
    }
}
