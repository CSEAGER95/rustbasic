use std::io;

pub fn run_conditionalstatements()
{
    const CORRECT_ANSWER: &str = "let language = if a == b {\"rust\"} else {\"java\"};";
    println!("---BEGINNING CONDITIONAL STATEMENTS PRACTICE---");

    println!("
            
            writing a series of nested if/else statements

            if number % 4 == 0 {{
                println!(\"number is divisible by 4\");
            }} else if number % 3 == 0 {{
                println!(\"number is divisible by 3\");
            }} else if number % 2 == 0 {{
                println!(\"number is divisible by 2\");
            }} else {{
                println!(\"number is not divisible by 4, 3, or 2\");
            }}

            writing a conditional statement on one line

            let number = if condition {{ 5 }} else {{ \"six\" }};
    ");
    println!("write a one line statement defining a parameter called language to be rust if a = b or java if not.");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .unwrap();
        let input = input.trim();

        if input == CORRECT_ANSWER {
            println!("correct press enter to continue");
            let mut dummy = String::new();
            io::stdin()
                .read_line(&mut dummy)
                .unwrap();
            break;
        } else {
            println!("incorrect, try again.");
        }
    }
}
