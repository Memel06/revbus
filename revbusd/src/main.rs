use librevbus::{Event, EventBus};
use std::io::{self, BufRead};
use std::sync::Arc;
use tokio::spawn;

#[tokio::main]
async fn main() {
    let bus = Arc::new(EventBus::new());

    println!("Revbus CLI started.");
    println!("Commands:");
    println!("  publish <topic> <payload>");
    println!("  subscribe <topic>");
    println!("  exit\n");

    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.trim().to_string())
        .take_while(|line| line.to_lowercase() != "exit")
        .for_each(|line| handle_command(line, Arc::clone(&bus)));

    println!("Shutting down Revbus.");
}

fn handle_command(line: String, bus: Arc<EventBus>) {
    let mut parts = line.splitn(3, ' ');
    let command = parts.next();
    let topic = parts.next();
    let payload = parts.next();

    match command {
        Some("publish") => {
            topic
                .zip(payload)
                .map(|(t, p)| bus.publish(Event::new(t, p)))
                .unwrap_or_else(|| println!("Usage: publish <topic> <payload>"));
        }

        Some("subscribe") => {
            topic
                .map(|t| {
                    let mut rx = bus.subscribe(t);
                    let t = t.to_string();

                    spawn(async move {
                        println!("Subscribed to '{}'", t);
                        while let Some(event) = rx.recv().await {
                            println!("[{}] {:?}", t, event);
                        }
                    });
                })
                .unwrap_or_else(|| println!("Usage: subscribe <topic>"));
        }

        Some(cmd) => println!("Unknown command: '{}'", cmd),
        None => println!("No command provided."),
    }
}