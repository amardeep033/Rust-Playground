// Q: Can we stream JSON in Rust, or does serde_json need the whole file/string first?

use serde::Deserialize;
use serde_json::Deserializer;
use std::io::{BufRead, Cursor};

#[derive(Debug, Deserialize)]
struct Event {
    id: u32,
    kind: String,
}

fn main() {
    // Case 1: many JSON values one after another.
    // This shape is common in logs and protocols:
    // {"id":1,"kind":"start"} {"id":2,"kind":"stop"}
    let adjacent_values = r#"
        {"id":1,"kind":"start"}
        {"id":2,"kind":"tick"}
        {"id":3,"kind":"stop"}
    "#;

    let stream = Deserializer::from_str(adjacent_values).into_iter::<Event>();

    for event in stream {
        let event = event.unwrap();
        println!("adjacent value: id={} kind={}", event.id, event.kind);
    }

    // Case 2: newline-delimited JSON (NDJSON).
    // Each line is its own complete JSON value.
    let ndjson = Cursor::new(
        r#"{"id":10,"kind":"open"}
{"id":11,"kind":"close"}
"#,
    );

    for line in ndjson.lines() {
        let line = line.unwrap();
        let event: Event = serde_json::from_str(&line).unwrap();
        println!("ndjson line: id={} kind={}", event.id, event.kind);
    }

    // Case 3: one normal JSON array.
    // This parses the full array as one JSON value.
    let array = r#"[{"id":20,"kind":"a"},{"id":21,"kind":"b"}]"#;
    let events: Vec<Event> = serde_json::from_str(array).unwrap();
    println!("array parsed as Vec: {events:?}");
}

// A: Yes, but "stream JSON" can mean different shapes. `serde_json::from_str`
//    parses one complete JSON value. `serde_json::Deserializer` can read from a
//    string or reader and yield many adjacent JSON values one by one. NDJSON is
//    often even simpler: read each line, parse that line as one value.
//
// -- more Q&A --
// Q: Does a normal JSON array stream element by element automatically?
// A: Not with `from_str::<Vec<T>>()`; that builds the Vec. For truly huge arrays,
//    use a streaming-deserialization helper or write a visitor. For logs and APIs,
//    NDJSON is usually easier.
// Q: Why use `from_reader`?
// A: It accepts any `Read`, such as a file, stdin, TCP stream, or HTTP body. That
//    means serde_json can parse as bytes arrive instead of requiring your code to
//    first create one giant String.
