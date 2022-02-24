use std::collections::HashMap;
use std::io;

struct personnel
{
    staff:String,
    department:String,
}

fn main() 
{
    let mut depart_list:HashMap< String, String > = HashMap::new();

    loop
    {
        let mut choose = String::new();
        println!("Please choose what to do.\n1: increase staff\n2: get list");
        io::stdin().read_line(&mut choose).expect("Failed to read line");
        let choose: u32 = choose.trim().parse().expect("Please type a number!");

        if choose == 1 
        {
            write_to_list( & mut depart_list );
        }
        else if choose == 2
        {
            read_list( & depart_list );
        }
        else
        {
            println!("Unknow choose");
        }
    }
}

fn write_to_list( depart_list:& mut HashMap< String, String > )
{
    let mut department = String::new();
    let mut staff = String::new();

    println!("Department is :");

    io::stdin().read_line(&mut department).expect("Failed to read line");

    println!("Staff name is :");

    io::stdin().read_line(&mut staff).expect("Failed to read line");

    /*let p1 = personnel {
        staff:staff,
        department:department,
    };

    depart_list.insert(personnel.staff, personnel.department);*/
    depart_list.insert(staff, department);
}

fn read_list( depart_list:&HashMap< String, String > )
{
    println!("Please choose what to do.\n1: Get a list of all employees in a department\n2: Get a lexicographical list of all employees in each department of the company");
    
    let mut choose = String::new();

    let mut department = String::new();

    io::stdin().read_line(&mut choose).expect("Failed to read line");
    let choose: u32 = choose.trim().parse().expect("Please type a number!");

    if choose == 1 
    {
        println!("Please enter department name");
        io::stdin().read_line(&mut department).expect("Failed to read line");

        let mut flag = 1;

        for i in depart_list
        {
            if *i.1 == department 
            {
                println!( "{:?}", i );
                flag = 0;
            }
        }

        if flag == 1
        {
            println!( "no staff" );
        }
    }
    else if choose == 2
    {
        println!("{:?}", depart_list);
    }
    else
    {
        println!("Unknow choose");
    }
}