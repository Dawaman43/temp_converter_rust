use std::io;

fn main(){
    println!(" celsius to fahrenheit converter");
    println!("Enter celsius value");

    let mut celsius = String::new();

    io::stdin()
        .read_line(&mut celsius)
        .expect("Please enter the celsius value");

    let celsius : f32= celsius.trim().parse().expect("Please enter correct number format");

    let fahrenheit = (celsius * (9.0 / 5.0)) + 32.0;

    println!(" Celsius = {celsius} equal to when converted is Fahrenheit = {fahrenheit}") ;
}