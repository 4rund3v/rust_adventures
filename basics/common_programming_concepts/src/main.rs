
const MAX_TIME_TO_LIVE: u16 = 60*30;

// Function to test out how variables behave
fn variables() {
    let mut x = 10;
    println!("The value of the variable x is :{x}");
    // Trying to update the value of x
    x = 20;
    println!("The value of the variable x is : {x}");

    let a = 10;
    println!(" The initial value of a is :: {a}");

    {
        let a = a* a ;
        println!("The value of a in the inner block 1 is :: {a}");
    }
    println!("The value of a in the outer block is :: {a}");
    // let MAX_TIME_TO_LIVE = 60*60;
    // println!("The max time to live is :: {MAX_TIME_TO_LIVE}");
    // throws --> refutable pattern in local binding
}

fn main() {
    println!("Hello, world!, lets understand");
    variables();
    println!("The max time to live is :: {MAX_TIME_TO_LIVE}");
}
