fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers = [15, 8, 23, 42, 5, 30, 19, 60, 75, 100];

    for &num in numbers.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else if is_even(num) {
            println!("{}: Even", num);
        } else {
            println!("{}: Odd", num);
        }
    }

    let mut sum = 0;
    let mut index = 0;
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("Sum of numbers: {}", sum);

    let mut max = numbers[0];
    for &num in numbers.iter() {
        if num > max {
            max = num;
        }
    }
    println!("Largest number: {}", max);
}
