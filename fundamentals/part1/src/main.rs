/*
 * Common programming concepts
 * Understanding Ownership
 * Structs and Related Data
 * Enums and Pattern Matching
 * 
 * Scope is to build an financial tracker application
 *  - Takes expenses in a day, calculates mean median mode
 *  - Calculates the current balances/based on expenses
 */


fn calculate_day_expenses() {
    let day_expenses : [i32; 4] = [40, 69, 05, 1123];
    println!("Calculating the day expenses: {:?}", day_expenses);
    let daily_sum : i32 = {
        let mut temp_sum: i32 = 0;
        for day_expense in day_expenses {
            temp_sum += day_expense;
        }
        temp_sum
    }; 
    println!("The sum of the day expeneses is :: [{:0>6}]", daily_sum)
}




fn main() {
    println!("Calculating the day expenses ");
    calculate_day_expenses();
}
