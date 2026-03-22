use std::io;

fn read_line() -> String {
    let mut string = String::new();

    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read line");

    string
}

fn main() {
    println!("Enter temperature in F.");

    let temp_f = read_line();
    
    let temp_f: f64 = temp_f
        .trim()
        .parse()
        .expect("Temperature entered was not a number.");
    
    let temp_c = (temp_f - 32.0) * 5.0 / 9.0;

    println!("Temperature {temp_f} Fahrenheit is {temp_c:.1} Celsius.");
    
    // let mut retry = String::new();
    // println("Convert another temperature?");
    // io::stdin()
}