// Q: What is the difference between serde and serde_json?
//    If serde can serialize, why do we need serde_json?

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    active: bool,
}

fn main() {
    let user = User {
        id: 1,
        name: String::from("Amar"),
        active: true,
    };

    // serde gives the traits. serde_json gives a JSON implementation of those traits.
    //
    // The derive above expands to implementations of serde::Serialize and
    // serde::Deserialize for User. It does NOT mean "User is JSON-only".
    // serde_json then uses those generic serde implementations to produce JSON.
    let text = serde_json::to_string_pretty(&user).unwrap();
    println!("json text:\n{text}");

    let back: User = serde_json::from_str(&text).unwrap();
    println!("typed struct again: {back:?}");

    // serde_json can also work with dynamic JSON values when you do not know the
    // final shape at compile time.
    let loose: Value = json!({
        "id": 2,
        "name": "Grace",
        "tags": ["compiler", "math"]
    });

    println!("dynamic JSON name = {}", loose["name"]);
    println!("dynamic JSON tags = {}", loose["tags"].as_array().unwrap().len());
}

// A: `serde` is the generic serialization framework: traits, derives, and the
//    shared data model. `serde_json` is one format crate that knows how to turn
//    serde-compatible Rust values into JSON and JSON back into Rust values.
//
//    Why not use only serde_json?
//    Because serde_json should not be the owner of your Rust data model. If JSON
//    owned the traits, then every format crate would need its own derive system:
//    JsonSerialize, TomlSerialize, YamlSerialize, BincodeSerialize, and so on.
//    serde avoids that. Your type implements serde::Serialize once, then many
//    format crates can use that same implementation.
//
// -- more Q&A --
// Q: Can I use serde without serde_json?
// A: Yes. The same `Serialize`/`Deserialize` derives can work with TOML, YAML,
//    postcard, bincode, MessagePack, and other format crates.
// Q: Can I use serde_json without serde?
// A: Not for typed structs. serde_json depends on serde traits. You can use
//    serde_json::Value for loose dynamic JSON, but once you want `User` ->
//    JSON or JSON -> `User`, serde's traits are the bridge.
// Q: Then why do I write both in Cargo.toml?
// A: `serde` enables the derives and traits your structs implement. `serde_json`
//    provides JSON functions like `to_string`, `from_str`, `Value`, and `json!`.
// Q: When should I use `serde_json::Value`?
// A: Use `Value` for dynamic or unknown JSON. Prefer typed structs when you know the
//    schema, because the compiler then checks field names and types for you.
// Q: Is `json!` from serde or serde_json?
// A: `json!` is from serde_json. It builds a `serde_json::Value`.
