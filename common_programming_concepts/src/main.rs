// fn main() {
//     let x = 5;
//     println!("the value of x is:{x}")
//     x = 6;
// }

// example of a const 
// note that consts are not able to have the mutation key word added 
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// Example of shadowing, note that the shadowing is localized only 
fn main(){
    let x = 5;
    let x = x+1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}") 
    }
    println!("The value of x is: {x}");

    //example of a tuple declaration
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // deconstruction of tuple  
    let (x, y, z) = tup;
    // access by entryy 
    let five_hundred = tup.0;
    tup.0 = 8;
    tup.1+=5;
    // example of an array note that these are fixed length in rust
    // declaration is [type; array length]
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // or [value; number of value] 
    let a = [3;5]; // [3,3,3,3,3]
    // accessing is as expected 
    let b = a[0];

    // example if else 
    let number = 5
    // each one of these choices is called an arm
    if number < 5 {
        println!("condition was true");
    } else if number <10 {
        println!("number is bigger then 10");
    }else {
        println!("number failed  to match a condintion")
    }

    // can use if else in a statement note: types must be same in both arms 
    let condition = true;
    let number  = if condition {5} else {6};
   
}
 // to return a value from a function  declare the type, the last value is returned by default
 // also note that there is no semi colon 
 /* This is a multi line comment*/
fn five() -> i32 {
    5
}
// example function that will return the value calculated in the loop 
fn calculate_value() {
    let mut counter = 0;
    // get the result of the loop
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn nested_loop_label() {
    let mut count = 0;
    //outer loop with the label counting up
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // break out of outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
// an example for loop 
fn example_for_loop() {
    let a = [10, 20, 30, 40, 50];
    // going forward in the element
    for element in a {
        println!("the value is: {element}");
    }
    // for number in range 1 -> 4 and   traverse in reverse ie 4,3,2,1
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

