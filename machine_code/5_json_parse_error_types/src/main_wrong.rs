use std::fs::File;

use serde::Deserialize;

//----------------------------------------------------
enum EventType {
    Email, //this email has nothing to do with struct Email - doesn't enforce
    Push,
}

trait EventHandler {
    fn printer(&self);
}

struct Email {
    to: String,
    subject: String,
}

impl EventHandler for Email {
    fn printer(&self) {
        println!("Sending Email");
        println!("To: {}", self.to);
        println!("Subject: {}", self.subject);
        println!();
    }
}

struct Push {
    device_id: String,
    title: String,
    body: String,
}

impl EventHandler for Push {
    fn printer(&self) {
        println!("Sending Push Notification");
        println!("Device: {}", self.device_id);
        println!("Title: {}", self.title);
        println!("Body: {}", self.body);
        println!();
    }
}

//----------------------------------------------------

#[derive(Debug, Deserialize)]
struct RawEvent {
    #[serde(rename = "type")]
    event_type: String,
    to: Option<String>,
    subject: Option<String>,
    device_id: Option<String>,
    title: Option<String>,
    body: Option<String>,
}

fn build_handler(event: RawEvent) -> Option<Box<dyn EventHandler>> {
    let event_type = match event.event_type.as_str() {
        "email" => EventType::Email,
        "push" => EventType::Push,
        other => {
            eprintln!("skip: unknown event type: {other}");
            return None;
        }
    };

    match event_type {
        EventType::Email => Some(Box::new(Email {
            to: event.to?,
            subject: event.subject?,
        })),
        EventType::Push => Some(Box::new(Push {
            device_id: event.device_id?,
            title: event.title?,
            body: event.body?,
        })),
    }
}

fn main() {
    let file = File::open("inp.json").expect("failed to open inp.json");
    let events: Vec<RawEvent> = serde_json::from_reader(file).expect("invalid JSON input");

    let mut handlers: Vec<Box<dyn EventHandler>> = Vec::new();
    let mut skipped = 0;

    for event in events {
        match build_handler(event) {
            Some(handler) => handlers.push(handler),
            None => skipped += 1,
        }
    }

    for handler in &handlers {
        handler.printer(); //dynamic dispatch
    }

    println!("Processed: {}, Skipped: {}", handlers.len(), skipped);
}
