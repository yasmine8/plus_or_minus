use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut strokes = 1;
    println!("the secret number is {secret_number}");
    loop {
        println!("please insert a number between 1 and 100:");
        let mut supposition = String::new();
        
        io::stdin()
            .read_line(&mut supposition)
            .expect("Failed to read user input");
    
        let supposition: u32 = match supposition.trim().parse() {
            Ok(number) => number,
            Err(_) => continue
        };
        println!("Your number is: {supposition}");
        
        match supposition.cmp(&secret_number){
            Ordering::Greater => println!("It's less !"),
            Ordering::Less => println!("It's Greater !"),
            Ordering::Equal => {
                println!("You won with {strokes} strokes!");
                break;
            },
        }
        strokes +=1;

    }      
}
