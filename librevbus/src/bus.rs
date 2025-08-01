use crate::event::Event;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

type Subscriber = mpsc::UnboundedSender<Event>;

pub struct EventBus {
    subscribers: Arc<Mutex<HashMap<String, Vec<Subscriber>>>>,
}

impl EventBus {
    pub fn new() -> Self {
        EventBus {
            subscribers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn subscribe(&self, topic: &str) -> mpsc::UnboundedReceiver<Event> {
        let (tx, rx) = mpsc::unbounded_channel();

        let mut subs = self.subscribers.lock().unwrap();
        subs.entry(topic.to_string())
            .or_insert_with(Vec::new)
            .push(tx);
        rx
    }

    pub fn publish(&self, event: Event) {
        let subs = self.subscribers.lock().unwrap();
        if let Some(receivers) = subs.get(&event.topic) {
            for tx in receivers {
                let _ = tx.send(event.clone()); // ignore send error (receiver dropped)
            }
        }
    }
}
