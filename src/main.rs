//2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

mod user_input;
mod math;

fn main() {
    println!("This program will find the smallest number that can be divided by each of the numbers from 1 to 'x' without remainder.");
    let number: u128 = user_input::get_user_input("Please enter a number for 'x':");
    let numbers: Vec<u128> = math::number_to_vector(number);

    println!("Smallest multiple of all numbers between 1 and {} is: {}", number, math::calculate_smallest_multiple(numbers));
}
