use std::fs::File;
use serde::Deserialize;

//-------------------------------------------------------------------

// This enum represents the real event after JSON has been parsed.
// Each enum variant stores the struct that contains data for that event.
enum EventTypeEnum {
    Email(EmailEvent),
    Push(PushEvent),
}

trait EventHandler {
    fn print(&self);
}

impl EventHandler for EventTypeEnum {
    fn print(&self) {
        match self {
            EventTypeEnum::Email(email) => email.print(),//static dispatch
            EventTypeEnum::Push(push) => push.print(),
        }
    }
}

//-------------------------------------------------------------------

//---struct1---
struct EmailEvent {
    to: String,
    subject: String,
}

impl EventHandler for EmailEvent {
    fn print(&self) {
        println!("Sending Email");
        println!("To: {}", self.to);
        println!("Subject: {}", self.subject);
        println!();
    }
}

//---struct2---
struct PushEvent {
    device_id: String,
    title: String,
    body: String,
}

impl EventHandler for PushEvent {
    fn print(&self) {
        println!("Sending Push Notification");
        println!("Device: {}", self.device_id);
        println!("Title: {}", self.title);
        println!("Body: {}", self.body);
        println!();
    }
}

//-------------------------------------------------------------------

// This struct matches the loose JSON input format.
// Fields are optional because different event types need different fields.
#[derive(Debug, Deserialize)]
struct RawJsonEvent {
    #[serde(rename = "type")]
    event_kind: String,
    to: Option<String>,
    subject: Option<String>,
    device_id: Option<String>,
    title: Option<String>,
    body: Option<String>,
}

// Convert the loose JSON shape into a strongly typed event.
// The ? operator skips the event if a required field is missing.
fn parse_event(raw_event: RawJsonEvent) -> Option<EventTypeEnum> {
    match raw_event.event_kind.as_str() {
        "email" => Some(EventTypeEnum::Email(EmailEvent {
            to: raw_event.to?,
            subject: raw_event.subject?,
        })),
        "push" => Some(EventTypeEnum::Push(PushEvent {
            device_id: raw_event.device_id?,
            title: raw_event.title?,
            body: raw_event.body?,
        })),
        other => {
            eprintln!("skip: unknown event type: {other}");
            None
        }
    }
}

fn main() {
    let file = File::open("inp.json").expect("failed to open inp.json");
    let raw_events: Vec<RawJsonEvent> = serde_json::from_reader(file).expect("invalid JSON input");

    let mut parsed_events: Vec<EventTypeEnum> = Vec::new();
    let mut skipped = 0;

    for raw_event in raw_events {
        match parse_event(raw_event) {
            Some(event) => parsed_events.push(event),
            None => skipped += 1,
        }
    }

    for event in &parsed_events {
        event.print();
    }

    println!("Processed: {}, Skipped: {}", parsed_events.len(), skipped);
}
