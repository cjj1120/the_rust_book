// fn main() {

//     // Accessing tuple (compound type), destructure the variable
//     println!("Hello, world!");
//     let tup = (500, 6.4, 1);

//     let (_x, y, _z) = tup;

//     println!("The value of y is: {y}");



//     // Second approach: dot access
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;

//     let six_point_four = x.1;

//     let one = x.2;

//     println!("x0: {five_hundred}, x1:{six_point_four}, x2: {one}");


//     // Array Section 
//     let a: [i32; 5] = [1, 2, 3, 4, 5];

// }




// if u type the index, u are able to retrieve the value.
// but when the input index is out of range, it will panick and throw an error.. 

// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!("The value of the element at index {index} is: {element}");
// }


// # 3.3 Functions
//------------------------------
// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }



// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {y}");
// }


// Functions with return values 

fn five() -> i32 {
    "test"; // if this is return, will cause error due to expected return type 
    5 // also make sure an expression is returned, we can't return statement (with ;) will cause error. 
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}