fn hello_world() {
    let mut greeting: String = String::from("Hello from the memory management project.");
    println!("The greeting is :: {}", greeting);
    // ToDo Learn the difference between {} and {:?} in the print statements
    // println!("The greeting is :: {:?}", greeting); is adding an `"string here"` the extra quotes.
    greeting.push_str(
        " The memory management is basically understanding the variable 
    	scope and what memory lies on the stack and queue.",
    );
    println!("The greeting is :: {}", greeting);
}

fn main() {
    hello_world();
}
