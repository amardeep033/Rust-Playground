# Event Processing Engine

## Problem

A system receives events as JSON. Each event has a `type` field and type-specific data. Parse the events and execute them using polymorphism (`Box<dyn Trait>`), not a giant `match` in the processing logic.

## Input

```json
[
    { "type": "email", "to": "alice@example.com", "subject": "Welcome" },
    { "type": "sms", "phone": "+911234567890", "message": "OTP is 1234" },
    { "type": "push", "device_id": "device-123", "title": "Sale!", "body": "50% OFF" }
]
```

## Requirements

1. **Enum** for event kinds:

```rust
enum EventType {
    Email,
    Sms,
    Push,
}
```

2. **Structs** per event: `EmailEvent`, `SmsEvent`, `PushEvent`.

3. **Trait**:

```rust
trait EventHandler {
    fn process(&self);
}
```

4. **Implement the trait** for each event struct. `process()` prints the event's fields (see Expected Output).

5. **Parse JSON** with `serde` + `serde_json`. Deserialize into an intermediate `RawEvent` struct first (all fields optional), then convert.

6. Build a `Box<dyn EventHandler>` for each event based on its `type`, skipping/logging invalid or unknown ones, then call `.process()` on each.

## Constraints

- No `match` inside `process()` — polymorphism decides behavior, not a switch on a type tag.
- Use `Box<dyn EventHandler>` for dynamic dispatch.
- Handle invalid/unknown event types gracefully (don't `panic!`).
- Log how many events were processed successfully vs. skipped.

## Expected Output

```
Sending Email
To: alice@example.com
Subject: Welcome

Sending SMS
Phone: +911234567890
Message: OTP is 1234

Sending Push Notification
Device: device-123
Title: Sale!
Body: 50% OFF

Processed: 3, Skipped: 0
```

## Project Structure

Single file is fine for a ~20-minute pass:

```
src/
 └── main.rs
```

## Bonus (if time permits)

Add a new event type without modifying any existing handler impl (Open/Closed Principle):

```json
{ "type": "webhook", "url": "https://abc.com", "payload": "..." }
```
