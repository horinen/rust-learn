use std::io;

fn main() 
{
    let vowel = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let s1 = "ay";
    let s0 = "hay";

    let mut s = String::new();

    let mut new_word_flag = true;

    let mut print_str = String::new();;

    let mut juage = 1;  //1 = consonant  0 = vowel

    let mut first_char  = ' ';

    println!("please input string");

    io::stdin().read_line(&mut s);

    for c in s.chars() 
    {
        if true //Determine whether it is a letter or a space //Too lazy to implement this function
        {
            if c == ' '
            {
                if new_word_flag == false
                {
                    new_word_flag = true;
                    print_str.push( '-' );
                    if juage == 1
                    {
                        print_str.push( first_char );
                        print_str.push_str( s1 );
                    }
                    else
                    {
                        print_str.push_str( s0 );
                    }
                    print_str.push( ' ' );
                }
                continue;
            }
            else if new_word_flag == true
            {
                new_word_flag = false;

                juage = 1;
                
                for i in vowel
                {
                    if c == i
                    {
                        juage = 0;
                        break;
                    }
                }

                if juage == 0 
                {
                    print_str.push( c );
                }
                else
                {
                    first_char = c;
                }
            }
            else
            {
                print_str.push( c );
            }
        }
    }

    print_str.push( '-' );
    if juage == 1
    {
        print_str.push( first_char );
        print_str.push_str( s1 );
    }
    else
    {
        print_str.push_str( s0 );
    }
    print_str.push( ' ' );

    println!("{}",print_str);
}
