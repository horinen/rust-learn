use std::io;

#[allow(non_snake_case)]

fn main() 
{
    let FAHRE:u8 = 0;
    let CELSIUS:u8 = 1;
    let UNKNOW:u8 = 2;

    let mut charge_type:u8 = FAHRE;
    
    loop
    {
        let mut guess = String::new();
        let mut temperature = String::new();

        println!( "Please select Fahrenheit or Celsius" );

        io::stdin( ).read_line(&mut guess).expect("Failed to read line");

        let guess = guess.trim();

        if guess == "Fahrenheit"
        {
            charge_type = FAHRE;
        }
        else if guess == "Celsius"
        {
            charge_type = CELSIUS;
        }
        else
        {
            charge_type = UNKNOW;
        }

        if charge_type == UNKNOW
        {
            println!( "Unknow type" );
            continue;
        }
        else
        {
            println!( "Please enter the temperature" );
        }

        io::stdin( ).read_line(&mut temperature).expect("Failed to read line");
        let temperature: f32 = temperature.trim().parse().expect("Please type a number!");
        
        if charge_type == FAHRE
        {
            let temperature:f32 = temperature * 1.8 + 32.0;
            println!( "Celsius temperature is {}", temperature );
        }
        else
        {
            let temperature:f32 = ( temperature - 32.0 ) / 1.8;
            println!( "Fahrenheit temperature is {}", temperature );
        }

    }

    println!( "Hello, world!" );
}
