use std::fs::File;
use std::io;
use std::io::Write;

pub fn write_a_text_file() {
    let content = read_from_stdin();

    // Expect is syntactic sugar for what's below
    let mut some_file = File::create("some_text_file.txt").expect("Couldn't create file");

    // This match is typically needed to ensure we handle all possible cases from fns that return
    // Result<T, E>. But you can also use the expect shorthand which just panics with the message you send
    // if there's an error.
    match some_file.write(content.as_bytes()) {
        Ok(size) => {}
        Err(e) => println!("Ate an error from writing to the file."),
    }
}

fn read_from_stdin() -> String {
    // // Reading input from Stdin
    println!("Enter anything:");
    let mut input: String = String::new();
    match io::stdin().read_line(&mut input) {
        // Readline returns an enum we can match on
        // We can bind values for handling enums variants with values.
        // Here inputSize is bound  for use in our handling function.
        Ok(inputSize) => {
            println!("You entered: {} bytes", inputSize);
        }
        Err(_) => {} // Eat the error
    }
    return input;
}
