## Realization 
!!slowly go thru each section to actually understand things.. I go thru the section slowly this morning and realized this is the way of learning! I understand much more and much deeper as compared to just copy pasting code, wishint that it would work...

Cargo understands Semantic Versioning (sometimes called SemVer), which is a standard for writing version numbers. This is the dependency feature of Rust, ensure the dependencies compatability..


### built-in documentation helper: 
Another neat feature of Cargo is that running the cargo doc --open command will build documentation provided by all your dependencies locally and open it in your browser. If you’re interested in other functionality in the rand crate, for example, run cargo doc --open and click rand in the sidebar on the left.

SO DAMN COOL!

### The `&`
The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references


-Rust has a strong, static type system.
-Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess
-Result type is to return the response status, use `expect` for actual error handling.



$ create build 
$ create run 



[official guide page](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

## Chapter 3 Summary
This project was a hands-on way to introduce you to many new Rust concepts: let, match, functions, the use of external crates, and more. In the next few chapters, you’ll learn about these concepts in more detail. Chapter 3 covers concepts that most programming languages have, such as variables, data types, and functions, and shows how to use them in Rust. Chapter 4 explores ownership, a feature that makes Rust different from other languages. Chapter 5 discusses structs and method syntax, and Chapter 6 explains how enums work.

