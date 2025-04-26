extern crate rand; // Add this line to declare the external crate
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    const NUM_PRODUCERS: usize = 2;
    const NUM_CONSUMERS: usize = 3;

    // Create a channel for sending numbers
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    // Create 2 producer threads
    let mut producers = Vec::new();
    for id in 0..NUM_PRODUCERS {
        let tx_clone = tx.clone();
        producers.push(thread::spawn(move || {
            producer(id, tx_clone, ITEM_COUNT / NUM_PRODUCERS);
        }));
    }

    // Create 3 consumer threads
    let mut consumers = Vec::new();
    for id in 0..NUM_CONSUMERS {
        let rx_clone = Arc::clone(&rx);
        consumers.push(thread::spawn(move || {
            consumer(id, rx_clone);
        }));
    }

    // Wait for all producers to finish
    for producer in producers {
        producer.join().unwrap();
    }

    // Send termination signals for each consumer
    for _ in 0..NUM_CONSUMERS {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    // Wait for all consumers to finish
    for consumer in consumers {
        consumer.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

// Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();
    for _ in 0..item_count {
        let num = rng.gen_range(1..=100);
        println!("Producer {} generated: {}", id, num);
        tx.send(num).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    println!("Producer {} finished producing.", id);
}

// Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let message = rx.lock().unwrap().recv().unwrap();
        if message == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal.", id);
            break;
        }
        println!("Consumer {} processed: {}", id, message);
        thread::sleep(Duration::from_millis(150));
    }
}
