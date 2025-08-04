// temperature converter

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn run_temperature_converter() {
    println!("\n== Assignment 1: Temperature Converter ==");

    let mut temp_f = 87.0; 
    let mut count = 0;

    while count < 6 {
        let c = fahrenheit_to_celsius(temp_f);
        println!("{:.1}°F = {:.2}°C", temp_f, c);
        temp_f += 1.0;
        count += 1;
    }
}

// num analyzer

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn run_number_analyzer() {
    println!("\n== Assignment 2: Number Analyzer ==");

    let nums = [4, 9, 5, 19, 13, 15, 1, 18, 23, 26];

    for i in 0..nums.len() {
        let num = nums[i];

        if is_even(num) {
            println!("{} is even", num);
        } else {
            println!("{} is odd", num);
        }

        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        }
    }

    let mut sum = 0;
    let mut i = 0;

    while i < nums.len() {
        sum += nums[i];
        i += 1;
    }

    println!("Sum of all numbers: {}", sum);

    let mut largest = nums[0];
    let mut index = 0;

    loop {
        if nums[index] > largest {
            largest = nums[index];
        }

        index += 1;

        if index >= nums.len() {
            break;
        }
    }

    println!("Largest number: {}", largest);
}

//guessing game

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess < secret {
        -1
    } else if guess > secret {
        1
    } else {
        0
    }
}

fn run_guessing_game() {
    println!("\n== Assignment 3: Guessing Game ==");

    let secret = 17;
    let mut guess = 10;
    let mut attempts = 0;

    loop {
        attempts += 1;

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("You guessed it! {} is correct!", guess);
            break;
        } else if result == -1 {
            println!("{} is too low.", guess);
            guess += 1;
        } else {
            println!("{} is too high.", guess);
            guess -= 1;
        }
    }

    println!("It took you {} guesses!", attempts);
}


fn main() {
    run_temperature_converter();
    run_number_analyzer();
    run_guessing_game();
}
