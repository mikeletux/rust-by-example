#[derive(Debug)]
struct Structure(i32);

// Put structure Structure inside of the structure Deep
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    // Printing with `{:?}` is similar to `{}`
    println!("{:?} months in a year", 12);

    // Structure is now printable
    println!("{:?}", Structure(3));

    println!("{:?}", Deep(Structure(7)));

    // Rust supports pretty printing as well
    let name = "Peter";
    let age = 27;
    let peter = Person {name, age};

    // Pretty print
    println!("{:#?}", peter)
    
}
