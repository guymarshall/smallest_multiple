pub fn number_to_vector(number: u128) -> Vec<u128> {
    let mut numbers: Vec<u128> = Vec::new();

    for i in 1..number + 1 {
        numbers.push(i);
    }

    return numbers;
}

pub fn divisible_by_vector_values(number: u128, values: &Vec<u128>) -> bool {
    for value in values.iter() {
        if number % value != 0 {
            return false;
        }
    }

    return true;
}

pub fn calculate_smallest_multiple(numbers: Vec<u128>) -> u128 {
    let mut counter: u128 = 1;
    let smallest_multiple: u128;

    loop {
        if divisible_by_vector_values(counter, &numbers) {
            smallest_multiple = counter;
            break;
        }
        counter += 1;
    }

    return smallest_multiple;
}