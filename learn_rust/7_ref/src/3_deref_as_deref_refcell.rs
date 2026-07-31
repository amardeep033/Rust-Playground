// Q: Predict — why can `takes_str(&name)` work when `name` is a String, and why
//    does `Option<String>` need `.as_deref()` instead of just `&option`?

use std::cell::RefCell;
use std::ops::Deref;

fn takes_str(text: &str) {
    println!("str view: {text}");
}

fn main() {
    let name = String::from("amar");
    takes_str(&name); // deref coercion: &String -> &str

    let boxed = Box::new(String::from("boxed"));
    takes_str(&boxed); // &Box<String> -> &String -> &str

    let manual: &String = boxed.deref();
    println!("manual deref: {manual}");

    let maybe_name = Some(String::from("rust"));
    let maybe_view: Option<&str> = maybe_name.as_deref();
    println!("as_deref: {maybe_view:?}");

    let missing: Option<String> = None;
    println!("missing as_deref: {:?}", missing.as_deref());

    let names = RefCell::new(vec![String::from("Ada"), String::from("Grace")]);

    {
        let view = names.borrow(); // Ref<Vec<String>>, not exactly &Vec<String>
        println!("first = {}", view[0]); // Ref<T> implements Deref<Target = T>

        // let mut bad = names.borrow_mut(); // would PANIC at runtime, not compile error
        // bad.push(String::from("Linus"));
    }

    names.borrow_mut().push(String::from("Linus"));
    println!("names = {:?}", names.borrow());
}

// A: `Deref` lets smart/owned pointer-ish types expose the thing inside them. Function
//    arguments get deref coercion, so `&String` can become `&str` automatically. But an
//    `Option<String>` is not itself a String; it is an enum around one. `.as_deref()`
//    converts inside the option: `Option<String>` -> `Option<&str>` without moving.
//
// ── more Q&A ──
// Q: Should I call `.deref()` directly?
// A: Rarely. Let deref coercion do it at function boundaries, or use `&*x` when teaching.
//    Direct `.deref()` mostly appears in generic/smart-pointer code.
// Q: Why does `RefCell::borrow()` return `Ref<T>` instead of `&T`?
// A: `Ref<T>` is a guard. It records that a runtime borrow is active, and when the guard
//    drops, the borrow ends. That's how RefCell enforces borrow rules at runtime.
