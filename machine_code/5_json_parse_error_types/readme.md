# Comparing `main_wrong.rs` and `main_right.rs`

This example is about one important Rust idea:

> An enum variant name does not automatically connect to a struct with the same name.

Rust allows this:

```rust
enum EventType {
    Email,
    Push,
}

struct Email {
    to: String,
    subject: String,
}
```

But `EventType::Email` and `Email` are different things.

- `EventType::Email` is an enum variant.
- `Email { ... }` creates a struct value.
- Rust does not force the `Email` variant to use the `Email` struct.

## `main_wrong.rs`

In `main_wrong.rs`, the enum is only used like a label:

```rust
enum EventType {
    Email,
    Push,
}
```

Then the actual struct is created later:

```rust
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
```

This works, but the enum does not enforce a relationship with the structs.

For example, Rust would allow you to rename the struct to something else:

```rust
struct EmailMessage {
    to: String,
    subject: String,
}
```

The enum variant could still be named:

```rust
EventType::Email
```

So the name `Email` in the enum is only a label. It is not tied to the `Email` struct.

`main_wrong.rs` also stores events like this:

```rust
Vec<Box<dyn EventHandler>>
```

That means:

- store many different concrete types in one vector
- hide their exact type behind the `EventHandler` trait
- call behavior through dynamic dispatch

This is useful sometimes, but it is more indirect for this example.

## `main_right.rs`

In `main_right.rs`, the enum variants store the real structs:

```rust
enum EventTypeEnum {
    Email(EmailEvent),
    Push(PushEvent),
}
```

Now the relationship is enforced by Rust.

This means:

- `EventTypeEnum::Email` must contain an `EmailEvent`
- `EventTypeEnum::Push` must contain a `PushEvent`
- if `EmailEvent` does not exist, the code will not compile
- if the stored struct does not have the expected fields, the code will not compile

The parsed event is built directly from the JSON:

```rust
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
```

This version stores parsed events like this:

```rust
Vec<EventTypeEnum>
```

That means:

- all values in the vector have one concrete type: `EventTypeEnum`
- each enum value can still hold different event data
- no `Box<dyn EventHandler>` is needed

## Why `main_right.rs` is clearer

The right version models the domain more directly.

The JSON says:

```json
{
    "type": "email",
    "to": "alice@example.com",
    "subject": "Welcome"
}
```

The Rust code converts that loose JSON into a strong Rust type:

```rust
EventTypeEnum::Email(EmailEvent {
    to,
    subject,
})
```

After that, the program does not need to remember a separate enum label and a separate handler object. The enum already carries the correct event data.

## Trait implementation difference

In `main_wrong.rs`, each concrete struct implements the trait:

```rust
impl EventHandler for Email {
    fn printer(&self) {
        // print email
    }
}
```

Then values are stored as:

```rust
Box<dyn EventHandler>
```

In `main_right.rs`, each concrete struct still implements the trait:

```rust
impl EventHandler for EmailEvent {
    fn print(&self) {
        // print email
    }
}
```

But the enum also implements the trait:

```rust
impl EventHandler for EventTypeEnum {
    fn print(&self) {
        match self {
            EventTypeEnum::Email(email) => email.print(),
            EventTypeEnum::Push(push) => push.print(),
        }
    }
}
```

This lets you call:

```rust
event.print();
```

without caring whether the event is an email or push notification.

## Main takeaway

Use this style when you have a known fixed set of event types:

```rust
enum EventTypeEnum {
    Email(EmailEvent),
    Push(PushEvent),
}
```

Use `Box<dyn EventHandler>` when the set of types is more open-ended, for example when many independent modules may define new handlers.

For this JSON parsing example, `main_right.rs` is easier to understand because the enum owns the exact struct data for each event type.
