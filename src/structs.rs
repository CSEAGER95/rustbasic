pub fn run_structs()
{
    println!("---STRUCTS PRACTICE---");
    println!("
        struct User {{
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64,
        }}

        here is a struct this block of code creates an object called a user that has the listed properties.

            let user1 = User {{
                active: true,
                username: String::from(\"someusername123\"),
                email: String::from(\"someone@example.com\"),
                sign_in_count: 1,
            }};

            this creates a User object named user1 with the following fields.

            an objects field can be accessed like so

            user1.email = String::from(\"anotheremail@example.com\");

        fn build_user(email: String, username: String) -> User {{
            User {{
                active: true,
                username: username,
                email: email,
                sign_in_count: 1,
            }}
        }}

        here, we build a method to create a struct. this method takes in a two strings and spits out a user.
        this is the shorthand equivelant.

        fn build_user(email: String, username: String) -> User {{
        User {{
                active: true,
                username, (as opposed to username: username,)
                email, (in lieu of email: email,)
                sign_in_count: 1,
            }}
        }}

            let user2 = User {{
                email: String::from(\"another@example.com\"),
                ..user1
            }};
        based on the information of the object named user1, this replicates the fields in user2 changing only what is listed.

        

        "
    );

}
