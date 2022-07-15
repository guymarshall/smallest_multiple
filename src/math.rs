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