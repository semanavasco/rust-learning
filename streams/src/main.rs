use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        println!("=== Example 1 ===");
        example1().await;
        println!("=== Example 2 ===");
        example2().await;
        println!("=== Example 3 ===");
        example3().await;
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
fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for message in messages {
        tx.send(format!("Message: {message}")).unwrap();
    }

    ReceiverStream::new(rx)
}

/// Example of using a ReceiverStream
async fn example3() {
    let mut messages = get_messages();

    while let Some(message) = messages.next().await {
        println!("{message}");
    }
}
