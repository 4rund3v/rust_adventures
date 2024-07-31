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

fn ownership_concepts() {
    let gold_roger: String = String::from("I Left Everything I Gathered Together In One Place");
    // phrase_printer(phrase);
    // println!("The complete pharse is :: {phrase}");
    // Fails with the error`` borrow of moved value: `phrase`
    phrase_printer(gold_roger); // This does a move, phrase is cleaned up and sent to the phrase_printer method
                                // After adding the reference to the string
    let zoro: String = String::from("Only I Can Call My Dream Stupid!");
    phrase_printer2(&zoro);
    println!("The string after printing is {:?}", zoro);

    let mut hirlluk: String = String::from("A Man Dies When People Forget Him!");
    let hirlluk_2: &String = &hirlluk;
    let hirlluk_3: &String = &hirlluk;
    println!("The hirlluks saying is :: {} && {}", hirlluk_2, hirlluk_3);
    phrase_adder(&mut hirlluk);
    phrase_printer2(&hirlluk);

    let stripped_line: &str = full_stop_stripper(&hirlluk);
    println!("The line till full stop is {:?}", stripped_line);
    phrase_printer(hirlluk);

}

fn phrase_printer(phrase_string: String) {
    println!("Printing the string");
    for ch in phrase_string.chars() {
        print!("{}", ch);
    }
    println!("\n");
}

fn phrase_printer2(phrase_string: &String) {
    for ch in phrase_string.chars() {
        print!(".{}", ch);
    }
    println!("\n");
}

fn phrase_adder(base_phrase: &mut String) {
    base_phrase.push_str(
        "\n When do you think a person dies?
   When a bullet from a pistol pierces his heart?    No.
   When he's attacked by an incurable disease?     No.
   When he eats a soup of deadly poisonous mushrooms? No.
   A man dies when people forget him! - Hiriluk",
    );
}


fn full_stop_stripper(sentence: &String) -> &str {
    const FULL_STOP: u8 = b'.';
    let sentence_bytes = sentence.as_bytes();
    for (index, &char_elem) in sentence_bytes.iter().enumerate() {
        if char_elem == FULL_STOP {
            return &sentence[..index];
        }
    }
    return &sentence[..];
}


fn main() {
    hello_world();
    ownership_concepts();
}
