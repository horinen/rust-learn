use rand::Rng;
use std::collections::HashMap;

fn main() 
{
    let mut v = Vec::new();

    while v.len() != 50
    {
        v.push( rand::thread_rng().gen_range(1..11) );
    }

    let mut i = 0;

    for i in &v
    {
        print!("{} ",i);
    }

    print!("\n\n");

    while i < 49
    {
        let mut j = 0;
        while j < 49
        {   
            if v[j] > v[j+1]
            {
                let mid = v[j+1];
                v[j+1] = v[j];
                v[j] = mid;
            }
            j += 1;
        }
        i += 1;
    }

    for i in &v
    {
        print!("{} ",i);
    }

    print!("{} ",v[25]);

    print!("\n\n");

    let mut map = HashMap::new();

    for i in &v
    {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut max_count = 0;
    let mut max_number = 0;

    println!("{:?}", map);

    print!("\n\n");

    for i in map
    {
        if i.1 > max_count
        {
            max_number = *i.0;
            max_count = i.1;
            
        }
    }

    println!("{:?}", max_number);
}
