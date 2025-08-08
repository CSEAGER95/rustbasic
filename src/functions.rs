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

    println!("write a function that takes parameters and has a return value
        ");
}
