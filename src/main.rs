use std::io;

fn main() {
    fahrenheit_to_celsius();
}

fn fahrenheit_to_celsius() {

    println!("Enter the temperature in Fahrenheit.");

    let mut f = String::new();
    
    io::stdin()
        .read_line(&mut f)
        .expect("Failed to readline.");

    let f: f64 = f.trim().parse().expect("Please enter numbers only!");

    let c = {
        let x: f64 = 5.0;
        x / 9.0 * (f-32.0)
    };

    println!("{f} Fahrenheit is: {c} Celsius.")
}