/*
Some things I like about Rust up front:

It comes with a formatter: rustfmt. So everyone who writes Rust has the same formatting.
RustRover by JetBrains formats  with rustfmt with a very easy settings tweak.
I'm sure you can do similar things in most other editors. IntelliJ also has a Rust plugin.

It comes with a package manager: cargo.
Cargo is similar to NPM, and very easy to use. It's also standardized.
All the available packages are on the cargo website.
The built-in package manager is a huge win over C and C++ in my book.

You typically run Rust projects with "cargo run" but you could also build and then
execute with the rustc compiler directly with "rustc main.rs" and then "./main".
 */

use rust_cheatsheet::reading_files::write_a_text_file;
use rust_cheatsheet::some_module::some_function;
use rust_cheatsheet::some_module::some_sub_module::some_sub_fn;
use rust_cheatsheet::some_module::some_sub_module::sub_sub::SomeType;
use rust_cheatsheet::using_injection::injects;

// All rust programs start with a main function. Unless it's a library.
fn main() {
    some_function();
    some_sub_fn();
    let a_struct = SomeType {};

    // Some primitive data type examples
    let my_age: u8 = 28; // There are also 64-bit and 8-bit and 128-bit etc...
    let walt_disney_age: i128 = 122; // Rust convention is to use snake_case.
    let is_older_than_walt: bool = false;
    let disney_viewers_as_us_pop_percentage: f64 = 70.0;

    // You can create your own data types with Structs.
    // Structs essentially take the place of classes.
    // Allowing you to combine data state and functionality
    let mut my_rectangle = Rectangle {
        width: 30,
        height: 30,
    };

    // Access Struct methods with dot syntax.
    let area = my_rectangle.area();

    // Use the :: syntax to reference associated functions
    // Associated struct functions are like static methods in Java.
    let mut aSquare = Rectangle::square(15);
    // ... and enum variants
    // ... and modules and sub modules. The :: syntax kinda gets used everywhere.
    let myIpKind = IpAddrKind::V4;

    // This is how you print
    print!("Hello, world!");
    // You can print placeholders like so:
    println!("My age is: {}", my_age);
    // And more complex types like:
    println!("{:?}", my_rectangle);

    // A bit about ownership and borrowing...
    // This system is how Rust prevents us from writing memory errors
    let gets_owned: Rectangle = Rectangle::square(10);
    let gets_borrowed: &Rectangle = &Rectangle {
        width: 5,
        height: 5,
    };
    // This variable is marked as mutable!! Very important.
    let gets_mutated: &mut Rectangle = &mut Rectangle {
        width: 5,
        height: 5,
    };

    // If a function takes ownership of a variable,
    // the caller can no longer use it after calling that function
    owns(gets_owned);
    // This print line results in a compilation error "value borrowed after move". It got moved
    // to ownership of the "owns" function.
    // println!("{:?}", gets_owned);

    borrows(gets_borrowed);
    // The borrows function didn't take ownership.
    // It only borrowed the value, which means it can read it, but it can't mutate it.
    println!("{:?}", gets_borrowed); // So, it's still safe to use that value afterwards.

    mutates(gets_mutated);
    println!("{:?}", gets_mutated); // The values are changed!

    // LOOPS!
    let mut loops = 0;
    loop {
        if loops == 3 {
            break;
        }
        loops += 1;
    }

    while loops < 4 {
        loops += 1;
    }

    let some_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for num in some_array {
        println!("{}", num)
    }

    write_a_text_file();

    // Check out this funciton for an example of how to do dependency injection
    // with dynamic dispatch.
    injects();
}

// This function "borrows" the parameter passed in.
// It's not allowed to modify what gets passed but takes the parameter as a reference.
// The caller can continue to use the parameter after calling this function.
fn borrows(rect: &Rectangle) -> u32 {
    rect.width // The last expression in a block is an implicit return
}

// This function takes ownership of the parameter.
// The caller can no longer use rect after calling this function.
fn owns(rect: Rectangle) -> u32 {
    return rect.width; // You can also use return statements.
}

// This function borrows a mutable reference
// Which allows it to change the variable passed in.
// Remember, it's passed by reference!! The caller's state will change after calling this.
fn mutates(rect: &mut Rectangle) {
    rect.add_one()
}

// Definitions get hoisted in Rust programs!
// So I can put the code I don't want the reader to focus on at the
// bottom of the file the way I like it.

// A struct is defined as follows....
#[derive(Debug)] // This line lets us print Rectangles.
struct Rectangle {
    width: u32,
    height: u32,
}

// Structs that have methods on them get a separate code block like this.
impl Rectangle {
    fn area(self: &mut Self) -> u32 {
        return self.width * self.height;
    }

    fn add_one(self: &mut Self) {
        self.width += 1;
        self.height += 1;
    }

    // This method doesn't take self. It's like a static method. You access it with Rectangle::square()
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// Access different variants of an enum like: IpAddrKind::V4
enum IpAddrKind {
    V4,
    V6,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    // Match statements ensure that you hit all possible enum values before compiling.
    // These get used everywhere, because lots of stuff in the std lib returns enums.
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
