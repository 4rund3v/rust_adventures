
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

fn understanding_variables_data_types() {
    let radius: u8 = 22;
    println!("The value of the radius is :: {radius}");
    let  circumfrence: u16 = 21_999;
    println!("The circumfrence of the circle is:: {circumfrence}");
    let pi: f32 = 3.1414972;
    println!("The value of the pi is :: {pi}");
    let pi_root: f64 = (pi as f64).sqrt();
    println!("The value of the pi_root is :: {pi_root}");   
    circle_details();
    let flag: bool = true;
    println!("The value of the flag is :: {flag}");
    // Character
    let heart_eyed_cat: char = '😻';
    println!("The emoji is being printed as :: {heart_eyed_cat}");
    rhombus_details();
}

fn circle_details() {
    println!("\n\n\n");
    println!("The circle related calculations are as follows");
    let radius: u16 = 43;
    let pi: f32 = 3.1413570;
    let circumfrence: f32 = (2 as f32) * pi * (radius as f32);
    let area: f32 = pi * (( radius * radius ) as f32);
    println!("The circle radius is : {radius}");
    println!("The circumfrence is  : {circumfrence}");
    println!("The circle area  is  : {area}");
    let theta_deg: f32 = 60.00000000;
    let theta_rad: f32 = theta_deg * (pi / (180 as f32));
    println!("The theta in degree is {theta_deg} and in radians is : {theta_rad}");
    println!("\n\n\n")
}

fn rhombus_details() {
    println!("\n\n The rhombus tuple info is : \n");
    let p1: (i16, i16) = (-6, 0);
    let p2: (i16, i16) = (0, 6);
    let p3: (i16, i16) = (6, 0);
    let p4: (i16, i16) = (0, -6);

    let diagonal_squared: i16 = i16::pow(p1.0 - p3.0,2) + i16::pow(p1.1 - p3.1, 2);
    let diagonal: f32 = (diagonal_squared as f32).sqrt();
    println!("The points of the rhombus is :: 
        P1 : {:?}
        P2 : {:?}
        P3 : {:?}
        P4 : {:?}
        Diagonal length is : {:?}", p1, p2, p3, p4, diagonal);
    let p1_copy:(i16, i16) = p1;
    println!("The copy of the p1 is :{:?}", p1_copy);
    let (p1_x, p1_y) = p1;
    println!(" The x and y are :: ( {p1_x}, {p1_y})");
    println!("\n\n\n")
}

fn main() {
    println!("Hello, world!, lets understand rust");
    variables();
    println!("Understanding the variable datatypes");
    understanding_variables_data_types();
    println!("The max time to live is :: {MAX_TIME_TO_LIVE}");
}
