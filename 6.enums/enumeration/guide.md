struct vs enum 

I think this is a valid question to ask, at first glance they look super similar to me.. 

In the notes, the author shows that using an enum is much more concise than struct. 

Notes: 
```
Here, it’s also easier to see another detail of how enums work: the name of each enum variant that we define also becomes a function that constructs an instance of the enum

you can put any kind of data inside an enum variant: strings, numeric types, or structs

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```
^It's easy to create function with enum. 

Struct and enum can achieve the same purpose, but enum can put things in a more concise manner as mentioned above.. multiple lines of declaration with struct can be declared in one block using enum. 

Just like struct, we can also use `impl` with enum, to implement functions. 

# Rust's null 
Rust has an enum that can encode the concept of a value being present or absent. This enum is Option<T>. 

Notes: 
```
enum Option<T> {
    None,
    Some(T),
}

Its variants are also included in the prelude: you can use Some and None directly without the Option:: prefix. The Option<T> enum is still just a regular enum, and Some(T) and None are still variants of type Option<T>.

```


some differences of Option<T> and T (basic data type)
```
in short, because Option<T> and T (where T can be any type) are different types, the compiler won’t let us use an Option<T> value as if it were definitely a valid value. 

let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```
^ compiler will throw error, tho they are Option<i8> and <i8> actually different types... 

## The weird null handling in Rust 
This weird logic is to handle the million dollar null issue basically. 

Note, explanation: 
```
In other words, you have to convert an Option<T> to a T before you can perform T operations with it. Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.

Eliminating the risk of incorrectly assuming a not-null value helps you to be more confident in your code. In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value Option<T>. Then, when you use that value, you are required to explicitly handle the case when the value is null. Everywhere that a value has a type that isn’t an Option<T>, you can safely assume that the value isn’t null. This was a deliberate design decision for Rust to limit null’s pervasiveness and increase the safety of Rust code.
```
^gotta convert back to <T> before one can use it. 

Now that I think of python None variable, I think it kinda make sense... interesting, really!

## `match` is the control flow here
Notes: 
```
The match expression is a control flow construct that does just this when used with enums: it will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value.
```






9.7.23 
had a crazy dream, was running around looking for the train to take, was literally in the open space (outdoor).. i think i was with jiaxin.. then we went to a house that has all kinda weird n scary stuff, its like escape room lol 

Regarding exception handling.. 
SL: exception is not control flow.. 
It's shouldn't be used to deal with a known issue.. for example I know that the query would fail and affect the subsequent queries, I should handle the error at the beginning instead of re-connecting to the db to resolve the error. 

I agree actually, `exception` is the last resolution and shouldn't be used as control flow, I will keep this in mind.  