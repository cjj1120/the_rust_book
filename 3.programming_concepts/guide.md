# Index
---------------------------
3.1 Variables and Mutability 
3.2 Data types 
3.3 Functions
3.4 Comments 
3.5 Control Flow


# 3.1 Variables and Mutability
------------------------------
-const type must be annotated and is not mutable. 

shadowing of a variable: 
when u re-initialize a variable, it is said that the variable is being shadowed. 

Wait a minute, so shadowing is similar to `let mut` let's read notes from the site for the differences! : 
```
NOTES 
Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name. 
```

# 3.1 Data Types 
------------------------------
Rust is a statically typed language. It MUST know all data types during compilation. 
But why `let x = 6` works without specifying the type? This is because Rust can infer (automatically) based on the value assign. 

Super cool! As for this `let guess: u32 = "42".parse().expect("Not a number!");` the return data type of parse() can be many, thus we need to specify here! 

## Scalar type 
A scalar type represents a single value.
Native scalar type: integer, floating Booleans & characters. 

- Signed vs unsigned 
singed means the integer can be negative, it can come with a `- sign`, vice versa for unsigned. 


-(2^(n - 1)) to 2^(n - 1) - 1

|Length |	Signed|	Unsigned|
|-------|---------|---------|
|8-bit	|i8	      | u8      |
|16-bit	|i16      |	u16     |
|32-bit |	i32   |	u32     |
|64-bit |	i64   |	u64     |
|128-bit|	i128  |	u128    |
|arch	|isize    |	usize   |


for 8-bit calculation: 
-(2^(8 - 1)) to 2^(8 - 1) - 1 == -128 to 127 


## Compound type 
Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

Tuple can consist of multiple data types, but array can only have one data type. 

Arrays are more useful when you know the number of elements will not need to change.

 A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size.


 ### The logic behind Rust's index out of range error 
 This is an example of Rust’s memory safety principles in action. In many low-level languages, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed. Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing. Chapter 9 discusses more of Rust’s error handling and how you can write readable, safe code that neither panics nor allows invalid memory access.


 # 3.3 Functions
------------------------------
## Statement and expression 

- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a resultant value. Let’s look at some examples.

-Expression doesnt have semicolon. 
Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function return values and expressions next.


Following code will return error, because this is a statement and it doesnt return value, this is unlike other languages where u can do x = y = 6. 
```
fn main() {
    let x = (let y = 6);
}
```

Here's the example for expression: 
-take note that there's no semicolon for expression x+1 
```
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```


Practices: 
- Convert temperatures between Fahrenheit and Celsius.
- Generate the nth Fibonacci number.
- Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.