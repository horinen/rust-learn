fn main() 
{
    let vowel = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    let s = String::from("initial contents");

    let mut new_word_flag = true;

    let mut count_num = 0;
    let mut word_start:i32;
    let mut word_stop:i32;

    for c in s.chars() 
    {
        count_num += 1;
        if c == ' '
        {
            if new_word_flag == false
            {
                new_word_flag = true;
                word_stop = count_num - 1;
                print_str = s[word_start..word_stop];
            }
            continue;
        }
        else if new_word_flag == true
        {
            new_word_flag = false;
            word_start = count_num;

            let mut juage = 1;  //vowel
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
                
            }
        }
    }

    println!("Hello, world!");
}
