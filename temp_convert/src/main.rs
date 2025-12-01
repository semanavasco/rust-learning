use std::process::exit;

fn main() {
    println!("Temperature converter : what do you want to convert ?");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");

    let mut response = String::new();

    println!("Type your answer :");

    std::io::stdin()
        .read_line(&mut response)
        .expect("Couldn't read your response.");

    let response: u8 = response
        .trim()
        .parse()
        .expect("Couldn't parse your response.");

    match response {
        1 => {
            println!("Type the temperature in Celsius :");
        }
        2 => {
            println!("Type the temperature in Fahrenheit :");
        }
        _ => exit(1),
    }

    let mut temperature = String::new();

    std::io::stdin()
        .read_line(&mut temperature)
        .expect("Couldn't read the temperature.");

    let temperature: f64 = temperature
        .trim()
        .parse()
        .expect("Couldn't parse the temperature.");

    let temperature = if response == 1 {
        cf(temperature)
    } else {
        fc(temperature)
    };

    println!("The temperature is equal to {temperature}.");
}

fn cf(temp: f64) -> f64 {
    temp * (9.0 / 5.0) + 32.0
}

fn fc(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}
