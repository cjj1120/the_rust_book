fn main() {
    println!("Hello, world!");
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");



    let mut s = String::from("lo");
    s.push('l');
    println!("s is {s}"); 



    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3 is {s3}");



    // Another way to concatenate string 
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);    

    
    // one character = 1 byte, for latin words. 
    let ps = &s[0..4];
    println!("{}", ps);


    for c in s.chars(){
        println!("{c}");
    }

    


}
