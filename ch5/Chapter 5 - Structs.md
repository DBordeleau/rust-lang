* A struct is a custom data type that allows us to organize related data together. 
* Structs in Rust somewhat resemble objects in the OOP paradigm.
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}```
* Like objects in OOP, to use a struct after we've defined it we must create an *instance* of it.
```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    // We can access the struct's attributes with dot notation
    user1.email = String::from("anotheremail@example.com");
}
```

* Like constructors in OOP, we will often write functions that will instantiate our structs for us.
```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // don't need username: username since the function parameter and struct atttribute share the same name
        email, // Same goes here, don't need email: email
        sign_in_count: 1,
    }
}
```

* Imagine we wanted to create a 2nd user who had the same field values as the first user in all fields except for email. It would be annoying to have to rewrite all these attributes. We can use `..` to specify that any fields we don't set should have the same value as a given instance, in this case, user 1.

```rust
let user2 = User {
	email: String::from("another@example.com"),
	..user1 // Sets all other fields to have the same values as user 1
};
```
* Note that the data is **moved** in this case. After we create user2 we can no longer use user1 as a whole. 
* If we were only using data types that implemented the copy trait this would not be the case.
* Rust also supports struct that look similar to tuples called ***tuple structs***. Tuple structs are structs, but their fields aren't named. 
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```
* Note that even though Color and Point are both comprised of three `i32` fields, a function that accepts a Color as a parameter could not be passed a Point instead. Point and Color are still distinct types.

* If we want to add methods to a struct Rust has a very particular syntax. We must define an implementation block with the `impl` keyword followed by the struct name and curly brackets.
```swift
impl User {
// This is where methods for the user struct would go
	fn login(someParameters) -> bool {
		// we could then call this function with dot notation like this:
		// userInstance.login(args)
	}
}
```
* If a function in an `impl` block is passed the self argument, it is a method. If it is not passed the self argument, it is an *associated function*.