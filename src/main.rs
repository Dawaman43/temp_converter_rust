use std::io;

fn main(){
    println!("Converter");
   loop {
       println!("Choose \n1. Celsius to Fahrenheit \n2. Fahrenheit to Celsius \n3. Exit");
       let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_)=>{
                println!("Please enter a valid number");
                continue;
            }
        };
        match choice {
            1 => {
                println!("Enter temperature in Celsius:");
                let mut celsius = String::new();

                io::stdin()
                    .read_line(&mut celsius)
                    .expect("Error in read line");

                let celsius: u32= match celsius.trim().parse() {
                    Ok(num) => num,
                    Err(_) =>{
                        println!("Please enter a valid number");
                        continue;
                    }
                };

                let fahrenheit = (celsius as f32 * (9.0 / 5.0)) + 32.0;
                println!("Temperature in Fahrenheit: {}", fahrenheit);

            },
            2 => {
                println!("Enter temperature in Fahrenheit:");
                let mut fahrenheit = String::new();

                io::stdin()
                    .read_line(&mut fahrenheit)
                    .expect("Error in read line");

                let fahrenheit: u32= match fahrenheit.trim().parse() {
                    Ok(num) => num,
                    Err(_) =>{
                        println!("Please enter a valid number");
                        continue;
                    }
                };

                let celsius = (fahrenheit as f32 - 32.0) * (5.0 / 9.0);
                println!("Temperature in Celsius: {}", celsius);
            },
            3 => {
                println!("Exiting...");
                break;
            },
            _ => {
                println!("Invalid choice, please select 1, 2, or 3.");
            }
        }
   }
}