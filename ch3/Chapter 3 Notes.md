* **Variables** in Rust are **immutable by default.** We must use the `mut` keyword to initialize mutable variables.
* We can use conditional expressions when assigning variables.
```rust
// This causes an error
let x = 5;
x = 6;

// This is okay
let mut x = 5;
x = 6;

let age = 17;
let isAdult = age >= 18; // Evaluates to false
```

* `const` variables still exist and cannot be made mutable with `mut`.

* ***Shadowing* allows us to reuse variable names to maintain immutability.
```rust
// This is okay and maintains immutability!
let x = 5;
let x = 6;

// You can even change types
let x = 5;
println!("The value of x is: {}", x);
let x = "six";
println!("The value of x is: {}", x);
```

<u>Integer Types in Rust</u>

![img1](https://github.com/DBordeleau/rust-lang/blob/main/ch3/Pasted%20image%2020250408223529.png)

* **Unsigned** integers can only be positive.
* Arch integers depend on architecture, usually 32 or 64 bit.
* Integers don't have to be expressed in decimal, Rust will also recognize hex, octal, binary and byte (u8 only).
* When integers overflow they wrap back to the lowest value.
	* e.g an 8bit integer can hold a maximum value of 255. If you assign a u8 int = 256 it will wrap around and be equal to 0. 257 will be = to 1.

**Compound Data Types**
```rust
// Tuples are declared like this:
let tup = ("Bob", 29, "Liberal", 2299.50);

// We can destructure the above tuple like this:
let (name, age, politics, money) = tup;

// Or we can use dot notation to extract specific variables
let name = tup.1;

// Arrays are declared with square brackets and 0 indexed
let name_arr = ["Bob", "Mike", "Chris"];
let name = name_arr[1]; // Sets name to "Mike"
let error = name_arr[5]; // This will result in a compile error

```

**Functions**
* functions are declared using the `fn` keyword.
* return types are specified with the `->` keyword after the `()` in the function signature.
```rust
fn main() {
	my_function("simple"); // Prints "This is a simple function"
	let f = celsius_to_fahrenheit(32.5); // Returns 32.5 * 9/5 + 32
}

// Use snake_case when naming functions
fn my_function(verb: str) {
	println!("This is a {} function!", verb);
}

// Specify return type with ->
fn celsius_to_fahrenheit(c: f32) -> f32 {
	c * 9.0 / 5.0 + 32.0; // Return keyword not necessary
}
```

**Loops**
* The `loop` keyword denotes a block of code that will run until we break the loop or return.
* We can set a variable equal to a loop's return value.
```rust
let mut counter = 0;
let result = loop {
	counter +=1;
	if counter == 10;
	break counter;
}

println("The result is {}", result); // Prints 10
```

* While loops and for loops are still here
```rust
let mut num = 10;
while num != 10 {
	num -= 1;
	println!("{}...", num)
}

// We can loop through arrays with iter()
let names_arr = ["Dillon", "Daniel", "Derek"];

// Print all names in the array
for element in names_arr.iter() {
println!("{}", element);
}

// We can loop through a range with the following .. notation
// Prints numbers from 1-9 (Range is exclusive)
for number in 1..10 {
	println!("{}", number)
}
```
