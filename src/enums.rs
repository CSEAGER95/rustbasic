pub fn run_enums() 
{
    println!("---ENUMS PRACTICE---");
    println!("
            enum IpAddrKind {{
                V4,
                V6,
            }}

            struct IpAddr {{
                kind: IpAddrKind,
                address: String,
            }}

            let home = IpAddr {{
                kind: IpAddrKind::V4,
                address: String::from(\"127.0.0.1\"),
            }};

            an example of an enum being created, then it's enum type being used as a param in a struct
            followed by the calling of the enum to create an object.
        "
    );
}
