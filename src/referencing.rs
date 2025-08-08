pub fn run_referencing()
{
    println!("---REFERENCING PRACTICE---");
    println!("
        pass a value by reference by using the & symbol
        fn main() {{
            let mut s = String::from(\"hello\");

            change(&mut s);
        }}

        fn change(some_string: &mut String) {{
            some_string.push_str(\", world\");
        }}

        

       
        "
    );
}
