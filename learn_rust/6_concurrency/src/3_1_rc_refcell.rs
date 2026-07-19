// Q: Predict — holding `shared.borrow()` and then calling `shared.borrow_mut()` on a
//    RefCell. Does it (a) fail to compile, or (b) compile and panic at runtime?

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let shared = Rc::new(RefCell::new(0)); // Rc = many owners (1 thread); RefCell = mutate via shared &

    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);
    *a.borrow_mut() += 10;
    *b.borrow_mut() += 5;
    println!("owners={} value={}", Rc::strong_count(&shared), shared.borrow()); // owners=3 value=15

    // let r = shared.borrow();
    // let w = shared.borrow_mut(); // ← COMPILES, but PANICS at runtime: "already borrowed"
}

// A: (b) It COMPILES and PANICS at runtime ("already mutably/immutably borrowed"). RefCell
//    moves the borrow check from COMPILE time to RUN time — the same &/&mut aliasing rule, just
//    enforced by a runtime flag instead of the compiler. The trap: it looks fine until it
//    explodes on a code path you didn't test. A plain `&`/`&mut` version would fail to compile instead.
//
// ── more Q&A ──
// Q: Rc vs Arc — why not just always use Arc?
// A: Rc's refcount is non-atomic (cheaper) but single-thread-only; Arc's is atomic (thread-safe)
//    but slightly costlier. Use Rc within one thread; Arc across threads. Rc isn't `Send`.
// Q: What's the multi-threaded equivalent of `Rc<RefCell<T>>`?
// A: `Arc<Mutex<T>>` (or `Arc<RwLock<T>>`) — shared ownership + synchronized mutation, but the
//    Mutex checks are enforced by blocking/locking rather than a panic.
