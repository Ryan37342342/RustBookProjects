
//Convert temperatures between Fahrenheit and Celsius.
// formula C = (f-31)/(9/5)

use std::{io, process::exit};

//Generate the nth Fibonacci number.

//Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
fn main(){
    // create holder for temp 
    let mut temp = String::new();
    // read in temp from command line 
 
        println!("enter a temperature in fahrenheit ...");
        io::stdin()
        .read_line(&mut temp)
        .expect("failed to read line");
    // read in the number as a float
    let temp: f32  = match temp.trim().parse() {
        Ok(temp ) => temp,
        // catch all errors and fail
        Err(_) =>  {
            println!("Failed to match a number to input: {temp} exiting... ");
            exit(1)
        }
    };
   
    
    // convert to celsius
    println!("fahrenheit: {temp}");
    let cel = fahrenheit_to_celsius(temp);
    println!("celsius {cel}");

    // generate fibonacci sequence number 
    // check that the number is bigger than 2 
    if temp >= 3.0{
       let fib = generate_fibonacci_sequence(temp);
       println!("the {temp} number  of the fibonacci sequence is {fib}")
    }
    else {
        println!("Number {temp} is too small to calculate fibonacci sequence number, must be bigger than 3")
    }

    print_lyrics();
}


fn fahrenheit_to_celsius(temp:f32) -> f32 {
   let temp: f32 = {
    (temp-31.0)/1.8
   };
   temp
}
/// this is a method to return the nth number in the fibonacci sequence
fn generate_fibonacci_sequence(num:f32) -> f32 {
   //formula = fn = fn+1 + fn -2 
    let fib: f32 = {
        (num+1.0) + (num -2.0)
        };
    return fib;
}


fn print_lyrics(){
    let gifts = ["a partridge in a pear tree","Two turtle doves","Three French hens", "Four calling birds", "Five golden rings",  "Six geese a-laying",
    "Seven swans a-swimming","Eight maids a-milking","Nine ladies dancing","Ten lords a-leaping","Eleven pipers piping",
    "Twelve drummers drumming"];
    let mut verse =1;
    // for each verse in the song 
    while verse<=12{   
        println!("On the {} day of Christmas my true love gave to me...",verse);
        // print the verses out 
        for index in (0..verse).rev(){
            // handle last case and first case seperately
            if index == 0 && verse !=1{
                println!("and {}!", gifts[index]);
            }else if index==0 {
                println!("{}!",gifts[index])
            }
            else{
                println!("{},",gifts[index]);
            }
        }
        println!();
        verse+=1;
    }
}
