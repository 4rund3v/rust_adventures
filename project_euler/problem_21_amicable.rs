/*! Let d(n) be defined as the sum of proper divisors of n (
numbers less than which divide evenly into ).
If d(a) = b and d(b) = a,
 where a=/= b, then a and b are an amicable pair and each of and  are called amicable numbers.

For example, the proper divisors of 
 220: [1, 2, 110, 4, 55, 5, 44, 10, 22, 11, 20] sum is 284
 284: [1, 2, 142, 4, 71]  sum is 220

Evaluate the sum of all the amicable numbers under 10000
Difficulty rating : 5%
*/

fn calculate_divisors_for_number(num: u16) -> Vec<u16> {
	// Find the divisors for the number and see if the number 
	let mut divisors: Vec<u16> = Vec::new();
	let mut index : u16 = 1;
	divisors.push(index);
	let mut upper_bound : u16 = num;
	while index < upper_bound {
        index = index + 1;
        let mut remainder = num % index;
		if remainder == 0 {
			let quotient = num / index;
			// println!("The quotient is {num} {index} :: {quotient}");

			if !!!divisors.contains(&index) {
			    divisors.push(index); 
			}
			if index != quotient && !!!divisors.contains(&quotient) {
			    divisors.push(quotient);	
			}
			upper_bound = quotient;
		}
	}
	divisors
}


fn main() {
	const MAX_NUM: usize = 10000;
	let mut pairs: u16 = 0;
	let mut index = 0;
	let mut paired_numbers: [u8; MAX_NUM] = [0; MAX_NUM];
    let mut paired_number_sum : u32 = 0;

	for number_to_check in (1..MAX_NUM) {
		// println!("Processing the number :: {number_to_check}");
		let number_to_check = number_to_check as u16;
		if paired_numbers[number_to_check as usize] == 1 {
			continue;
		}
		let divisors = calculate_divisors_for_number(number_to_check);
		let divisor_sum: u16 = divisors.iter().sum();
		if divisor_sum > MAX_NUM as u16 {
            continue;
		}
		if divisors.len() == 2 {
			continue;
		}
		if divisor_sum != number_to_check {
		   let paired_divisors = calculate_divisors_for_number(divisor_sum);
		   let paired_divisor_sum: u16 = paired_divisors.iter().sum();
		   if paired_divisors.len() == 2 {
			   continue;
		   }
		   if paired_divisor_sum == number_to_check {
		       paired_numbers[number_to_check as usize]	= 1;
		       paired_numbers[divisor_sum as usize]	= 1;
		       println!("the amicable pair found :: ({number_to_check}, {divisor_sum})");
		       // println!("The divisors are : {:?}", divisors);
		       // println!("The pair-divisors are : {:?}", paired_divisors);
		       paired_number_sum += number_to_check as u32;
		       paired_number_sum += divisor_sum as u32;
		   }
		}
	}
    println!("the total sum of the pairs is {paired_number_sum}"); //31626
	
}