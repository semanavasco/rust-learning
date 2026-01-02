use std::{pin::pin, time::Duration};

use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        println!("=== Example 1 ===");
        example1().await;
        println!("=== Example 2 ===");
        example2().await;
        println!("=== Example 3 ===");
        example3().await;
        println!("=== Example 4 ===");
        example4().await;
        println!("=== Example 5 ===");
        example5().await;
    });
}

/// First example on streams and their similarity to iterators
async fn example1() {
    let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = values.iter().map(|n| n * 2);
    let mut stream = trpl::stream_from_iter(iter);

    while let Some(value) = stream.next().await {
        println!("The value was: {value}");
    }
}

/// More complex example showcasing utility methods usage (filter)
async fn example2() {
    let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = values.iter().map(|n| n * 2);
    let stream = trpl::stream_from_iter(iter);

    let mut filtered = stream.filter(|n| n % 3 == 0 || n % 5 == 0);

    while let Some(value) = filtered.next().await {
        println!("The value was: {value}");
    }
}

/// Helper function to create a stream containing the 10 first letters of the alphabet as messages
/// Messages have a delay of 100 or 300 depending on whether they are even or not
fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for (i, msg) in messages.into_iter().enumerate() {
            let time_to_sleep = if i % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_err) = tx.send(format!("Message: '{msg}'")) {
                eprintln!("Cannot send message '{msg}': {send_err}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

/// Example of using a ReceiverStream
async fn example3() {
    let mut messages = get_messages();

    while let Some(message) = messages.next().await {
        println!("{message}");
    }
}

/// Adding a time limit on the items in a stream
async fn example4() {
    let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

    while let Some(result) = messages.next().await {
        match result {
            Ok(message) => println!("{message}"),
            Err(reason) => eprintln!("Problem: {reason}"),
        }
    }
}

/// Helper function to create a stream with a counter that will be emitted every millisecond.
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;

        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            if let Err(send_err) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_err}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

/// Merged messages and intervals.
/// Showcases the use of merge, throttle and take.
async fn example5() {
    let messages = get_messages().timeout(Duration::from_millis(200));
    let intervals = get_intervals()
        .map(|c| format!("Interval: {c}"))
        .throttle(Duration::from_millis(100))
        .timeout(Duration::from_secs(10));
    let merged = messages.merge(intervals).take(20);
    let mut stream = pin!(merged);

    while let Some(result) = stream.next().await {
        match result {
            Ok(message) => println!("{message}"),
            Err(reason) => eprintln!("Problem: {reason}"),
        }
    }
}
