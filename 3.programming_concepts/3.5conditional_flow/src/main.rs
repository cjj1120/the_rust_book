// fn main() {
//     let number = 3;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

//. Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean. You must be explicit 


// IF is an expression 
// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {number}");
// }

// Section LOOPS 
// --------------------

// fn main() {
//     let  x = 5; 
//     let mut count = 0;
//     loop {
//         println!("again!");
//         count = count + 1;
//         if count > x {
//             break; 
//         }
//     }
// }




// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2; // this is being returned.. 
//         }
//     };

//     println!("The result is {result}");
// }


// ineer loop 

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break; // this only breaks the inner loop 
//             }
//             if count == 2 {
//                 break 'counting_up; // this will break the outer loop, take note 'counting_loop' is the same name as the outer loop. 
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }

// While loop 

// fn main(){
//     let mut number = 3; 

//     while != 0{
//         println!("number {number}");
//         number -=1;         
//     }
//     println!("LIFTOFF!!!")
// }

// WHILE looping thru a list 
// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", a[index]);

//         index += 1;
//     }
// }

// FOR loop way to loop thru a list 
// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is: {element}");
//     }
// }

// using range, rev etc...

// fn main() {
//     for number in (1..4).rev() { //include first and exclude last element just right python 
//         println!("{number}!");
//     }
//     println!("LIFTOFF!!!");
// }


// PRACTICES 
// --------------

use std::io;

fn farenheit_to_celcius(value : i32) -> i32{ // make sure to annotate both input and output.. 
    // (32°F − 32) × 5/9 = 0°C
    (value - 32) * (5/9) 
}

fn main(){
    let mut temperature_value: i32;
    io::stdin()
        .read_line(&mut temperature_value)
        .expect("Failed to read line");

    let y = farenheit_to_celcius(temperature_value)
    println!(y)
}