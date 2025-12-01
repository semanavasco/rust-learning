fn main() {
    println!("Nth Fibonacci calculator !");
    println!("Type a number :");

    let mut number = String::new();

    std::io::stdin()
        .read_line(&mut number)
        .expect("Couldn't read your answer.");

    let number: i32 = number.trim().parse().expect("Couldn't parse your number.");

    println!(
        "The {number} fibonacci number is : {}",
        nth_fibonacci(number)
    );
}

fn nth_fibonacci(x: i32) -> i32 {
    if x <= 1 {
        return x;
    }

    nth_fibonacci(x - 1) + nth_fibonacci(x - 2)
}
