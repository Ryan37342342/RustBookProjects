use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    // generater range is from <start> to and including <end>
    let secert_number = rand:: thread_rng().gen_range(1..=100);

    loop{
        println!("Please input your guess.");

        // create number guess as string 
        let mut guess = String::new();
    
        io::stdin()
            // read string in from command line 
            .read_line(&mut guess)
            .expect("Failed to read line");
        //use shadowing ?? and trim whitespace etc and store in an int 32 variable also called guess
        // parse returns an enum that has two varients ok and err
        let guess : u32 = match guess.trim().parse(){
            //handle each possible returned result from parse t
            Ok(num) => num,
            // underscore is catch all so catch all possible returned errors
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
    
        match guess.cmp(&secert_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
   
}

