fn main() {
    let v = vec![1, 2, 3, 4, 5];
    println!("{:?}", v);
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let mut v = Vec::new();
    v.push("cjj");
    v.push("cc");
    v.push("dd");
    v.push("ee");
    println!("{:?}", v);

    // take note rust cannot handle different data types, below code will error 
    // let mut v = Vec::new();
    // v.push("cjj");
    // v.push(1);

    

    // E2. ANother error: mutable borrow and immutable borrow cannot happen under the same scope!! 
    // let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0]; // immmutable borrow

    // v.push(6); // mutable borrow

    // println!("The first element is: {first}");


    // Immutable looping 
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }


    // Mutable looping 
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    


    // This is to store different types in vector 
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}
