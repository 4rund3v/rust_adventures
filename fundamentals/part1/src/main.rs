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
// Use the `use` keyword to import the fmt module from std.
use std::fmt;

#[derive(Debug)]
struct Expense {
    expense_type: String,
    amount: i16,
    reason: String,
}

impl fmt::Display for Expense {
    // This enables the use of the {} for the Expense structure.
    // The fmt display method expects the following format
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<Expense: {} price:[{:0>7}] for reason: {:?}>",
            self.expense_type, self.amount, self.reason
        )
    }
}

fn calculate_day_expenses() {
    let daily_expenses: [Expense; 5] = [
        Expense {
            expense_type: String::from("expense"),
            amount: 40,
            reason: String::from("Sugarcane Juice"),
        },
        Expense {
            expense_type: String::from("expense"),
            amount: 1132,
            reason: String::from("Big Bazaar"),
        },
        Expense {
            expense_type: String::from("expense"),
            amount: 69,
            reason: String::from("Dmart Hooks"),
        },
        Expense {
            expense_type: String::from("expense"),
            amount: 30,
            reason: String::from("Shuttle Cock"),
        },
        Expense {
            expense_type: String::from("expense"),
            amount: 100,
            reason: String::from("Bannana and Grapes."),
        },
    ];
    let mut daily_sum: i32 = 0;
    for expense in daily_expenses {
        println!("The day expense to be processed is :: {}", expense);
        // unpacking the struct into variables
        let Expense {
            expense_type: day_expense_type,
            amount: day_amount,
            reason: day_reason,
        } = expense;
        daily_sum += day_amount as i32;
        println!("The {day_expense_type} reason is :: {day_reason} and it costed :: {day_amount}");
    }
    println!("The day expenses calculated is :: {daily_sum}");
}

fn main() {
    println!("Calculating the day expenses ");
    calculate_day_expenses();
}
