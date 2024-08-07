const MAX_TIME_TO_LIVE: u16 = 60 * 30;

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
        let a = a * a;
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
    let circumfrence: u16 = 21_999;
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
    const PI: f32 = 3.141357786;
    let circumfrence: f32 = (2 as f32) * PI * (radius as f32);
    let area: f32 = PI * ((radius * radius) as f32);
    println!("The circle radius is : {radius}");
    println!("The circumfrence is  : {circumfrence}");
    println!("The circle area  is  : {area}");
    let theta_deg: f32 = 60.00000000;
    let theta_rad: f32 = theta_deg * (PI / (180 as f32));
    println!("The theta in degree is {theta_deg} and in radians is : {theta_rad}");
    println!("\n\n\n");
}

fn rhombus_details() {
    println!("\n\n The rhombus tuple info is : \n");
    let p1: (i16, u16) = (-6, 0);
    let p2: (i16, u16) = (0, 8);
    let p3: (i16, u16) = (8, 0);
    let p4: (u16, i16) = (0, -10);

    let diagonal_squared: i16 =
        i16::pow((p1.0 as i16) - p3.0, 2) + i16::pow((p1.1 as i16) - (p3.1 as i16), 2);
    let diagonal: f32 = (diagonal_squared as f32).sqrt();
    println!(
        "The points of the rhombus is :: 
        P1 : {:?}
        P2 : {:?}
        P3 : {:?}
        P4 : {:?}
        Diagonal length is : {:?}",
        p1, p2, p3, p4, diagonal
    );
    let p1_copy: (i16, u16) = p1;
    println!("The copy of the p1 is :{:?}", p1_copy);
    // Destructuring an tuple
    let (p1_x, p1_y) = p1;
    println!(" The x and y are :: ( {p1_x}, {p1_y})");
    println!("\n\n\n");

    let point_arr = [p1.0 as i32, p1.1 as i32, p2.0 as i32, p2.1 as i32];
    println!("The point array is :: {:?}", point_arr);
    let point2_arr: [i32; 4] = [p3.0 as i32, p3.1 as i32, p4.0 as i32, p4.1 as i32];
    println!("The point 2  array is :: {:?}", point2_arr);
    let p3_point = point2_arr[3];
    println!("The p3 point is :: {p3_point}");
}

fn distance_between_points(px: (i32, i32), py: (i32, i32)) -> f32 {
    println!("The distance between points {:?} to {:?}", px, py);
    let dist_between_xy: f32 =
        f32::sqrt((i32::pow(py.0 - px.0, 2) + i32::pow(py.1 - px.1, 2)) as f32);
    println!("The distance calculated is :: {dist_between_xy}");
    dist_between_xy
}

fn geometry_methods(
    mut point_u: (i32, i32),
    point_v: (i32, i32),
    point_x: (i32, i32),
    point_y: (i32, i32),
) {
    point_u.0 = point_u.0 + point_u.0;
    println!(
        "The points provided are  
        Point u : {:?}
        Point v : {:?}
        Point x : {:?}
        Point y : {:?}
        ",
        point_u, point_v, point_x, point_y
    );
    let calculated_dist: f32 = distance_between_points(point_u, point_y);
    println!(
        "The distance calucated between the points is  {:?}",
        calculated_dist
    );
    trignomentry_understanding();
}

fn trignomentry_understanding() {
    let triangle: (i32, i32, i32) = (3, 4, 5);
    const PI_VALUE: f32 = 3.141592653594245;
    let mut hypotenuse: f32 = PI_VALUE;
    println!("The initial value of the hypotenuse is :: {hypotenuse}");
    println!("The value of Pi is :: {PI_VALUE}");
    if triangle.0 > triangle.1 {
        hypotenuse = f32::sqrt((i32::pow(triangle.0, 2) + i32::pow(triangle.1, 2)) as f32);
    } else if triangle.0 < 3 {
        hypotenuse = f32::sqrt((i32::pow(triangle.0, 2) + i32::pow(triangle.2, 2)) as f32);
    } else {
        hypotenuse = f32::sqrt((i32::pow(triangle.1, 2) + i32::pow(triangle.2, 2)) as f32);
    }
    println!(
        "The hypotenuse of the triangle : {:?} is {}",
        triangle, hypotenuse
    );
    if hypotenuse == triangle.2 as f32 {
        println!("The hypotenuse was calculated correctly. {:?}", hypotenuse);
    }
}

fn understanding_loops() {
    let mut counter: i32 = 17;
    let mut balance: i32 = 1000;
    let remaining = loop {
        counter -= 1;
        balance -= counter * 4;
        println!("The balance and counter are :: {:?} & {}", balance, counter);
        if balance < 1 {
            println!("Insufficient balance to continue, {balance}");
            break balance;
        } else if counter <= 1 {
            break balance; // the semicolon here is kinda optional, since its the last statement
        }
    };
    println!(
        "The remaining amount is {:?} from balance : {balance}",
        remaining
    );

    let mut cosmos: i32 = 384;
    'whiler: while cosmos > 16 {
        cosmos = cosmos / 2;
        println!("The value of cosmos is :: {cosmos}");
        if cosmos % 8 == 0 {
            println!(
                "The cosmos is divisible by 4, {} --> {}",
                cosmos,
                cosmos / 8
            );
            break 'whiler;
        }
    }

    const FIBBO_NUMS: [i32; 10] = [0, 1, 1, 2, 3, 6, 8, 13, 21, 34];
    let mut fib_index: usize = 1;
    let mut fib_sum: i32;

    'fibb_checker: while fib_index < 9 {
        fib_index += 1;
        println!("Checking the fibbonacci number at index : {}", fib_index);
        if fib_index > 1 {
            fib_sum = FIBBO_NUMS[fib_index - 1] + FIBBO_NUMS[fib_index - 2];
            if FIBBO_NUMS[fib_index] == fib_sum {
                println!(
                    "The fibbonacci number at : index[{}] is correct : {}",
                    fib_index, FIBBO_NUMS[fib_index]
                );
                continue;
            } else {
                println!(
                    "Invalid fibbonacci number at index {} and value : expected:{} found:{}",
                    fib_index, fib_sum, FIBBO_NUMS[fib_index]
                );
                break 'fibb_checker;
            }
        }
    }

    'fibb_printer: for element in FIBBO_NUMS {
        println!("The element in fibbonacci number is :: {element}");
    }

    'fibb_checker_for: for fib_index in (2..9) {
        println!("Checking the fibbonacci number at index : {}", fib_index);
        if fib_index > 1 {
            fib_sum = FIBBO_NUMS[fib_index - 1] + FIBBO_NUMS[fib_index - 2];
            if FIBBO_NUMS[fib_index] == fib_sum {
                println!(
                    "The fibbonacci number at : index[{}] is correct : {}",
                    fib_index, FIBBO_NUMS[fib_index]
                );
                continue;
            } else {
                println!(
                    "Invalid fibbonacci number at index {} and value : expected:{} found:{}",
                    fib_index, fib_sum, FIBBO_NUMS[fib_index]
                );
                break 'fibb_checker_for;
            }
        }
    }
}

fn main() {
    println!("Hello, world!, lets understand rust");
    variables();
    println!("Understanding the variable datatypes");
    understanding_variables_data_types();
    println!("The max time to live is :: {MAX_TIME_TO_LIVE}");
    println!("The functions block starts here:: ");
    let points = [(32, 36), (40, 50), (38, 44), (48, 58)];
    println!("The points defined are {:?}", points);
    geometry_methods(points[0], points[1], points[2], points[3]);
    println!("The points defined are {:?}", points);
    understanding_loops();
}
