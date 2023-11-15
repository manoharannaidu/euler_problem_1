// Euler Problem 1

fn main() {
    let muls_of_three = 1000 / 3;
    let muls_of_five = 999 / 5;
    let muls_of_fifteen = 1000 / 15;
    let sum_of_threes = muls_of_three * (muls_of_three + 1) * 3 / 2;
    let sum_of_fives = muls_of_five * (muls_of_five + 1) * 5 / 2;
    let sum_of_fifteens = muls_of_fifteen * (muls_of_fifteen + 1) * 15 / 2;
    println!("{}", (sum_of_fives + sum_of_threes - sum_of_fifteens));
}
