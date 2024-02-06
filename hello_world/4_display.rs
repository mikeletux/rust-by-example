use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64 
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{real:.1} + {imag:.1}i", real=self.real, imag=self.imag)
    }
}

fn main() {
    let my_structure = Structure(2);
    println!("{}", my_structure);

    let my_complex = Complex{ real: 3.3, imag: 7.2 };
    println!("Display: {}", my_complex);
    println!("Debug: {:?}", my_complex);
}