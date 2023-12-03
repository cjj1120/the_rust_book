# Collections in Rust 

some examples: 
- vectors 
- hash map 
- string (collection of characters)


## Nitty gritty reference approaches 
There are 2 different appraoches to refer to an element: 
```
let third: &i32 = &v[2];
let third: Option<&i32> = v.get(2);

the first [] method will cause the program to panic because it references a nonexistent element. This method is best used when you want your program to crash if there’s an attempt to access an element past the end of the vector.

 When the get method is passed an index that is outside the vector, it returns None without panicking. You would use this method if accessing an element beyond the range of the vector may happen occasionally under normal circumstance

```
Now think about it, this behavior is actually also the same in Python!  


## Mutable and immutable borrow of same variable cannot happen under the same scope 
E2 section in the code. 
```
The code in Listing 8-6 might look like it should work: why should a reference to the first element care about changes at the end of the vector? This error is due to the way vectors work: because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.
```


## INternal representation of String 
```
A String is a wrapper over a Vec<u8>.

Another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust’s perspective: as bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters).

```


# Hashmap 

```
Just like vectors, hash maps store their data on the heap. This HashMap has keys of type String and values of type i32. Like vectors, hash maps are homogeneous: all of the keys must have the same type as each other, and all of the values must have the same type.

```

