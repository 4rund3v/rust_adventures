fn print_string(str_ref: &str){
    println!("The string to print is :: {}", str_ref );
}
fn main() {
    let greeting: String = String::from("Hi there");
    print_string(&greeting[0..2]);
    print_string(&greeting[2..]);
}
