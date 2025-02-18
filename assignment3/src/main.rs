fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret_number = 42;
    let mut guess = 30;
    let mut attempts = 0;

    loop {
        attempts += 1;
        let result = check_guess(guess, secret_number);

        if result == 0 {
            println!("Correct! The secret number is {}", secret_number);
            break;
        } else if result == 1 {
            println!("{} is too high.", guess);
        } else {
            println!("{} is too low.", guess);
        }

        // Adjust the guess for the next iteration
        if guess < secret_number {
            guess += 5;
        } else {
            guess -= 3;
        }
    }

    println!("You found the secret number in {} attempts!", attempts);
}
