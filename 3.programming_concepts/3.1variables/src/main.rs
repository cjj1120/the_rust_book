fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");


    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The constant THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");


    let y = 5;

    let y = y + 1;

    {
        let y = x * 2;
        println!("The value of x in the inner scope is: {y}"); // this is a local scope, things that happen here remain here..
    }

    println!("The value of x is: {y}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

}