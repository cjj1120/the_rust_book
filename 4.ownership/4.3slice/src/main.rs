// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// fn main() {
//     let mut s = String::from("hello world");

//     let word = first_word(&s); // word will get the value 5
    
//     println!("{}", word);
//     s.clear(); // this empties the String, making it equal to ""

//     println!("{}", word);
//     // word still has the value 5 here, but there's no more string that
//     // we could meaningfully use the value 5 with. word is now totally invalid!
// }



//  slicing
// fn main() {
//     let s = String::from("hello world");

//     let _hello = &s[0..5];
//     let _world = &s[6..11];
// }


// Passing slice directly without any issue 
// altho we are passing it from immutable string 
// counterintuitive.. 
//  This flexibility takes advantage of deref coercions,
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let _word_ = first_word(&my_string[0..6]);
    let _word_ = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word_ = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let _word_ = first_word(&my_string_literal[0..6]);
    let _word_ = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word_ = first_word(my_string_literal);
}