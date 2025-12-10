use std::io;

fn main(){
    println!("fahrenheit to celsius cnverter");
    println!("Enter fahrenheit value");

    let mut fahrenheit = String::new();

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Please enter the fahrenheit value");

    let fahrenheit : f32= fahrenheit.trim().parse().expect("Please enter correct number format");

    let celsius = (fahrenheit - 32.0) * (5.0 / 9.0);

    println!("Fahrenheit = {fahrenheit} equal to when converted is Celsius = {celsius}") ;
}