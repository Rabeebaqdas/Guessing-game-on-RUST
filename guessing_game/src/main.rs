use std::io;
use std::cmp::Ordering;
use rand::{thread_rng,Rng};
fn main() {
    println!("Welcome to the Guessing Game!");
    let secret = rand::thread_rng().gen_range(1..101);
    println!("Secret number is {}",secret); 
    loop {
        println!("Please Enter Any Number:");
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Ooops! something went wrong :(");
        println!("You entered {}", num);
       let num:u32 = num.trim().parse().expect("This is not an integer");
    match num.cmp(&secret) {
        Ordering::Less => println!("Too short"),
        Ordering::Greater => println!("Too long"),
        Ordering::Equal => {
            println!("You Win :)");
            break;
    }
    } 
    }
  

}
