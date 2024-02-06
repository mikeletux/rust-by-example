fn main() {
    println!("{} days", 31);

    // Positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Named arguments
    println!("{subject} {verb} {object}",
            object="the lazy dog",
            subject="the quick brown fox",
            verb="jumps over");

    // Different formatting can be invoked
    println!("Base 10: {}", 69420);
    println!("Base 2(binary): {:b}", 69420);
    println!("Base 2(octal): {:o}",  69420);
    println!("Base 16(hex): {:x}",   69420);
    println!("Base 16(hex): {:X}",   69420);

    // Right justify
    println!("{number:>5}", number=1);

    // Pad number with extra zeroes
    println!("{number:0>5}", number=1);

    // and left-adjust
    println!("{number:0<5}", number=1);

    // format specifier appending $
    println!("{number:0>width$}", number=1, width=5);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let pi = 3.141592;
    println!("{pi:.3}");
}