* ***Ownership*** is a set of a rules that govern how a Rust program manages memory. The compiler checks if any of the rules are violated and if they are, the program won't compile. This allows Rust to make guarantees about memory safety without the need for a garbage collector

* **Stack:** Last in first out (LIFO) part of memory. All data stored on the stack has a known fixed size. Data with an unknown size at compile time, or a dynamic size must be stored on the heap instead.

* **Heap:** Less organized. A certain amount of memory is requested when putting data on the heap. The memory allocator finds an empty spot big enough and returns a pointer with the memory address of said location. 
	* Because this pointer is a known fixed size, it can be stored on the stack.

* It is faster to put memory onto and retrieve memory from the stack, as there is obviously more work involved in allocating on the heap.

**Ownership Rules**
1. Each value in Rust has an *owner*.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

* When a variable goes out of scope, Rust calls a special *drop* function for us which returns memory to the allocator.

```rust
{
	let s = String::from("hello"); // s is valid from now on
	// do stuff with s
}   // this scope is now over 
	// rust calls drop to return memory to the allocator and s is no longer valid
```

**Data Interacting with Move**
* The following code sets both variables equal to 5. This works because integers are simple values with a known, fixed size. Both values can be pushed onto the stack.
```rust
    let x = 5;
    let y = x;
```

* But what about Strings? It looks very similar so we might assume that s2 is a copy of s1, but that's not quite what happens.
```rust
    let s1 = String::from("hello");
    let s2 = s1;
```

* A string is made of 3 parts. A pointer to its memory address, its length and its capacity. This group of data is stored on the **stack**, the memory that actually holds the contents of the string is stored on the heap.
![[Pasted image 20250409120447.png]]
* When we assign s1 to s2, the String data (the pointer, length and capacity) are copied. That is, we duplicate the data on the stack. However, we do not copy the data on the heap. That means that s1 and s2 both have pointers that point to the same data on the heap. 
* This is not a concern with simple data types which are stored entirely on the stack.
![[Pasted image 20250409120758.png]]
* When s1 and s2 go out of scope and Rust calls the drop function, both variables will try to free the same memory on the heap. This is called a **double free** error. 
* To ensure memory safety, Rust no longer considers s1 to be valid after we assign s2 = s1; This is known as a **move**.

* If we do want a deep copy of the heap data we can use a common method called `clone`.
```rust
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
```

* Just bear in mind that this is a much more expensive operation.

**Ownership & Functions**
* Passing a variable to a function will move or copy just as assignment does.
* Variables that implement the copy trait (ints, bools, floats, chars) do not go out of scope when passed to a function, as they are copied instead of moved.
```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // because i32 implements the Copy trait,
                                    // x does NOT move into the function,
    println!("{}", x);              // so it's okay to use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
```

* If a function returns a value, the returned value is moved back out to the calling function.

**References & Borrowing**
* References are a feature that allow us to use a value without transferring ownership.
* A _reference_ is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable.
* Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
```rust
// Calculate_length takes a reference to s1 as a parameter instead of taking ownership of it
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}
// Function signature specifies the parameter is a reference as well
fn calculate_length(s: &String) -> usize {
    s.len()
}
```
* The `&s1` syntax creates a reference to the value of s1. Because the reference does not own the value of s1, the value it points to will not be dropped when the reference stops being used.
* The action of creating a reference in Rust is called ***borrowing***.
* References are immutable by default but can be made mutable with the `&mut` syntax.
* If you have a mutable reference to a value, you can have no other references to that value.
```rust
// Multiple mutable references are okay, just not simultaneously
let mut s = String::from("hello");

{
	let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new mut reference now.

let r2 = &mut s;

```

* In some languages, memory may be freed that is still being pointed to by some reference. These are called ***dangling pointers/references***. 
* Rust will prevent any data with a reference from going out of scope before the reference does with a compiler error.
```rust
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // The reference is still in scope! Compiler error!
```

**Slices**
* Slices let you reference a consecutive sequence of elements in a collection rather than the entire collection. A slice is a kind of reference, so it does not have ownership.
* A slice references a part of a string and looks like this:

```rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```

* The following code iterates through a string until it finds whitespace and then slices off the first word from the string.
```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```