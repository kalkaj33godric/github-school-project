fn main() {
    let numbers = vec![1, 2, 3];
    let mut largest = numbers[0];
    for number in numbers {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);
}
