// Q: Do I need `Rc<RefCell<T>>` whenever I want interior mutability?
//    Why not just use `&mut`, and what are `Cell`, `RefCell`, `OnceCell`,
//    and `Rc<RefCell<T>>` each for?

use std::cell::{Cell, OnceCell, RefCell};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct HitCounter {
    hits: Cell<u32>,
}

impl HitCounter {
    fn hit(&self) {
        self.hits.set(self.hits.get() + 1);
    }
}

#[derive(Debug)]
struct PlainLog {
    entries: Vec<String>,
}

impl PlainLog {
    fn record(&mut self, entry: impl Into<String>) {
        self.entries.push(entry.into());
    }
}

#[derive(Debug)]
struct EventLog {
    entries: RefCell<Vec<String>>,
}

impl EventLog {
    fn record(&self, entry: impl Into<String>) {
        self.entries.borrow_mut().push(entry.into());
    }

    fn snapshot(&self) -> Vec<String> {
        self.entries.borrow().clone()
    }
}

type SharedLog = Rc<RefCell<Vec<String>>>;

fn record_from(owner: &str, log: &SharedLog) {
    log.borrow_mut().push(format!("{owner} wrote a line"));
}

fn main() {
    let counter = HitCounter { hits: Cell::new(0) };
    counter.hit();
    counter.hit();
    println!("Cell counter = {}", counter.hits.get());

    let mut plain = PlainLog {
        entries: Vec::new(),
    };

    // Prefer `&mut self` when callers can give you exclusive mutable access.
    plain.record("plain mutable access");
    println!("plain &mut log = {:?}", plain.entries);

    let log = EventLog {
        entries: RefCell::new(Vec::new()),
    };

    // Use `RefCell` when the method must take `&self`, but one field still needs mutation.
    // That happens with caches, counters, logs, mocks, and shared trait-object APIs.
    log.record("created with one owner");
    log.record("mutated through &self");
    println!("single-owner RefCell = {:?}", log.snapshot());

    {
        let view = log.entries.borrow();
        println!("active immutable Ref guard sees {} entries", view.len());

        // let mut edit = log.entries.borrow_mut();
        // edit.push(String::from("overlap"));
        // This would compile, but panic at runtime because `view` is still alive.
    }

    log.record("mutable borrow works after the guard drops");

    // Add `Rc` only when multiple owners must point at the same allocation.
    let shared: SharedLog = Rc::new(RefCell::new(Vec::new()));
    let ui = Rc::clone(&shared);
    let worker = Rc::clone(&shared);

    record_from("ui", &ui);
    record_from("worker", &worker);
    println!("Rc<RefCell<_>> shared log = {:?}", shared.borrow());

    let config = OnceCell::new();
    config.set(String::from("debug")).unwrap();
    println!("OnceCell config = {}", config.get().unwrap());

    let thread_safe = Arc::new(Mutex::new(Vec::new()));
    {
        let mut locked = thread_safe.lock().unwrap();
        locked.push(String::from("safe to share between threads"));
    }
    println!("Arc<Mutex<_>> log = {:?}", thread_safe.lock().unwrap());
}

// A: No. `RefCell<T>` is for runtime-checked borrowing, not ownership. Use it by itself
//    when one owner needs to mutate part of itself through `&self`, such as caches, logs,
//    metrics, or test doubles. If your API can take `&mut self`, prefer that. It keeps
//    borrow mistakes as compile errors instead of runtime panics. Wrap `RefCell` in `Rc`
//    only when several owners need to share the same mutable value in one thread.
//
// ── quick chooser ──
// `&mut T`: first choice when the caller can give exclusive access.
// `Cell<T>`: tiny Copy values; get/set the whole value, no references into it.
// `RefCell<T>`: non-Copy values; borrow/borrow_mut checked at runtime.
// `OnceCell<T>`: set once, then read many times.
// `Rc<T>`: multiple owners in one thread, shared reads only.
// `Rc<RefCell<T>>`: multiple owners in one thread, shared mutation.
// `Arc<Mutex<T>>`: multiple owners across threads, shared mutation.
