//2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

mod user_input;
mod math;

fn main() {
    println!("This program will find the smallest number that can be divided by each of the numbers from 1 to 'x' without remainder.");
    let number: u128 = user_input::get_user_input("Please enter a number for 'x':");
    let numbers: Vec<u128> = math::number_to_vector(number);

    let mut counter: u128 = 1;
    let mut smallest_multiple: u128 = 0;
    loop {
        if math::divisible_by_vector_values(counter, &numbers) {
            smallest_multiple = counter;
            break;
        }
        counter += 1;
    }

    println!("Smallest multiple of 1 - {} is: {}", number, smallest_multiple);
}
