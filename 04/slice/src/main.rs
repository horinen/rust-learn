fn first_word(s: &String) -> usize 
{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() 
    {
        if item == b' ' 
        {
            return i;
        }
    }

    s.len()
}

fn main() 
{
    let s = String::from( "123" );

    let len = first_word( &s );
}
