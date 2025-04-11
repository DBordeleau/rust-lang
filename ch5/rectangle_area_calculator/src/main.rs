use std::io;

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn isBiggerThan(&self, &other) -> bool { // returns true if the other rectangle instance has smaller area than current rectangle
        (self.width * self.height) > (other.width * other.height)
    }
}

fn main() {
    let mut rect = Rectangle {
        width: 0,
        height: 0
    };

    let mut user_width = String::new();
    let mut user_height = String::new();

    println!("Enter the width in pixels: ");

    io::stdin()
        .read_line(&mut user_width)
        .expect("Failed to read line");

    rect.width = match user_width.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid width. Width must be an integer!");
            return;
        },
    };
    
    println!("Enter the height in pixels: ");
    
    io::stdin()
        .read_line(&mut user_height)
        .expect("Failed to read line");

    rect.height = match user_height.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid height. Height must be an integer!");
            return;
        },
    };

    println!("The area of the rectangle is {} square pixels.", rect.area());
}

/*
Solution without structs

fn main() {
    let mut width1 = String::new();
    let mut height1 = String::new();

    println!("Enter the width in pixels: ");

    io::stdin()
        .read_line(&mut width1)
        .expect("Failed to read line");

    let width1: u32 = match width1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid width. Width must be an integer!");
            return;
        },
    };
    
    println!("Enter the height in pixels: ");
    
    io::stdin()
        .read_line(&mut height1)
        .expect("Failed to read line");

    let height1: u32 = match height1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid height. Height must be an integer!");
            return;
        },
    };

    println!("The area of the rectangle is {} square pixels.", area(width1, height1));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
    
*/
