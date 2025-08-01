pub mod bus;
pub mod event;

pub use bus::EventBus;
pub use event::Event;

#[cfg(test)]
mod tests {
    use super::event::Event;
    use crate::bus::EventBus;
    use serde_json;
    #[test]
    fn test_create_event() {
        let topic = "test.topic";
        let payload = "{\"hello\": \"world\"}";
        let event = Event::new(topic, payload);

        assert_eq!(event.topic, topic);
        assert_eq!(event.payload, payload);
        assert!(event.timestamp > 0);

        println!("{:?}", event);
    }

    #[test]
    fn test_event_serialization() {
        let event = Event::new("system.reboot", "Rebooting node #42");

        let json = serde_json::to_string(&event).expect("Serialization failed");
        println!("Serialized: {}", json);

        let deserialized: Event = serde_json::from_str(&json).expect("Deserialization failed");

        assert_eq!(event.id, deserialized.id);
        assert_eq!(event.topic, deserialized.topic);
        assert_eq!(event.payload, deserialized.payload);
        assert_eq!(event.timestamp, deserialized.timestamp);
    }

    #[tokio::test]
    async fn test_publish_and_subscribe() {
        let bus = EventBus::new();
        let mut rx = bus.subscribe("test.topic");

        let event = crate::event::Event::new("test.topic", "Hello World!");
        bus.publish(event.clone());

        let received = rx.recv().await.expect("Should receive event");
        assert_eq!(received.topic, event.topic);
        assert_eq!(received.payload, event.payload);
    }
}
