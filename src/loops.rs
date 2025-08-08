use colored::*;

pub fn run_loops(){
    let kwords = (
        "loop".red(),
        "break".blue(),
        "for".red(),
        "while".red(),
        "loop labels".green(),
    );
    println!("---BEGINNING LOOP PRACTICE---");
    println!("  
        use {} as the key words to instantiate a loop
        
        {} {{//code to be repeated}};

        {} is the keyword to fall out of a loop when a condition is met
    
        this will assign a value of 20 to a variable. notice the use of a counter variable that increases by one for each iteration of the loop

        let mut counter = 0;

        let result = {} {{
            counter += 1;

            if counter == 10 {{
                {} counter * 2;
            }}
        }};

        {} can be used to break out of specific loops when working with nested loops.

        here is an example 

        let mut count = 0;
        'counting_up: {} {{
            let mut remaining = 10;

            {} {{
                if remaining == 9 {{
                    {};
                }}
                if count == 2 {{
                    {} 'counting_up;
                }}
                remaining -= 1;
            }}

            count += 1;
        }}

        this series of nested loops counts down from 10 - 9 iterating the count up by one each time remaining reaches 9
        when count reaches 2 the inner nested loop is broken. notice the ' that signifies the start of a loop label name.

        ",
        kwords.0,
        kwords.0,
        kwords.1,
        kwords.0,
        kwords.1,
        kwords.4,
        kwords.0,
        kwords.0,
        kwords.1,
        kwords.1,
    );

    println!("{} loops",kwords.3);

    println!("
        {} loop syntax is as follows.
        {} number != 0 {{number-=1;}}

        here's a concise for loop that offers a quick way to iterate through a list

        let a = [10, 20, 30, 40, 50];

        {} i in a {{
            println!(\"the value is: {{i}}\");
        }}


        countdown from 4 using a range and rev method

        {} number in (1..4).rev() {{
            println!(\"{{number}}!\");
        }}


        notice the range indicated by the parentheses. this covers 1-4 inclusive. the rev() method sends it from 4-1 inclusive.
    
        ",
        kwords.3,
        kwords.3,
        kwords.2,
        kwords.2,
    );

}
