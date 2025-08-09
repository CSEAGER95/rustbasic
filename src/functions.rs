use std::io;

pub fn run_functions()
{
    println!("--- BEGINNING FUNCTIONS PRACTICE ---");
    println!("
        the basic function starts with the keyword fn
        followed by the function name and a set of curly brackets.
        
        fn function_name() {{
            //code
        }}

        these functions are called within other functions like so

        function_name();

        if a function has parameters they are defined within
        the parentheses.

        fn function_name(x: i32) {{
            println!(\"{{}} x val\");
        }}
        
        multiple parameters are seperated with a comma

        fn square(x: i32, y: i32) {{
            //code
        }}

        return values are signified with a ->

        fn five() -> i32 {{
            5
        }}

        fn function_name(parameter: i32) -> i32 {{
            return parameter;
        }}

    ");

    println!("write a function header for a method named checker that takes a
        an input named input of i32 and returns a value of i32");

    loop{
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .unwrap();
        let input = input.trim();

        if inputchecker(input) {
            println!("correct! press enter to continue");
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

fn inputchecker(param: &str) -> bool {
    let correct = String::from("fn checker(input: i32) -> i32");
    return correct == param;
}
