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
}

