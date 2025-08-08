pub fn run_enumerationanditerators()
{
    println!("---ENUMERATION AND ITERATORS PRACTICE---");
    println!("

        fn first_word(s: &String) -> usize {{
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {{
                if item == b' ' {{
                    return i;
                }}
            }}

            s.len()
        }}

Because we need to go through the String element by element and check whether a value is a space, we’ll convert our String to an array of bytes using the as_bytes method.

    let bytes = s.as_bytes();

Next, we create an iterator over the array of bytes using the iter method:

    for (i, &item) in bytes.iter().enumerate()


        "
    );
}
