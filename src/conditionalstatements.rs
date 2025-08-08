pub fn run_conditionalstatements()
{
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
}
