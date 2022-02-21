use std::io;

#[allow(non_snake_case)]

fn main() 
{
    loop
    {
        let mut print_num = String::new();

        let mut i:u32 = 0;
        let mut Fibonacci_sequence_1:u32 = 1;
        let mut Fibonacci_sequence_2:u32 = 1;
        let mut Fibonacci_sequence_3:u32 = 1;

        println!( "Please enter the amount of data to print" );

        io::stdin( ).read_line(&mut print_num).expect("Failed to read line");
        let print_num:u32 = print_num.trim().parse().expect("Please type a number!");

        if print_num == 0
        {
            continue
        }
        else if print_num == 1 
        {
            print!( "1" );
            continue
        }
        else if print_num == 2
        {
            print!( "1 1" );
            continue
        }
        else
        {
            print!( "1 1" );
        }

        while i < print_num - 2
        {
            Fibonacci_sequence_3 = Fibonacci_sequence_1 + Fibonacci_sequence_2;
            print!( "\t{}",Fibonacci_sequence_3 );

            Fibonacci_sequence_1 = Fibonacci_sequence_2;
            Fibonacci_sequence_2 = Fibonacci_sequence_3;

            if ( ( i + 2 ) % 7 ) == 0
            {
                print!( "\n" );
            }

            i += 1;
        }
    }
}
