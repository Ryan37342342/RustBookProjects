
//Convert temperatures between Fahrenheit and Celsius.
// formula C = (f-31)/(9/5)

//Generate the nth Fibonacci number.
//Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
fn main(){
    // create holder for temp 
    let mut temp = String::new();
    // read in temp from command line 
    io::stdin()
        .read_line(&mut temp)
    let temp = 32.0;
    println!("fahrenheit: {temp}");
    let temp = fahrenheit_to_celsius(temp);
    println!("celsius {temp}");
}


fn fahrenheit_to_celsius(temp:f32) -> f32 {
   let temp = {
    (temp-31.0)/1.8
   };
   temp
}
