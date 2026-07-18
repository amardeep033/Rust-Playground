// Q: You need shared ownership AND mutation in a single thread — what combo, and
//    when does it blow up?

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let shared = Rc::new(RefCell::new(0));

    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);
    *a.borrow_mut() += 10;
    *b.borrow_mut() += 5;

    println!("count={} value={}", Rc::strong_count(&shared), shared.borrow());

    // let r = shared.borrow();
    // let w = shared.borrow_mut(); // compiles, but PANICS: already borrowed
}

// A: Rc<RefCell<T>>. Rc gives multiple owners via a (non-atomic) reference count;
//    RefCell gives interior mutability — mutating through a shared &. Its borrow rules
//    are checked at RUNTIME, so overlapping borrow()/borrow_mut() COMPILES but panics.
//
// ── more Q&A ──
// Q: Rc vs Arc?
// A: Rc uses a non-atomic refcount (single thread, faster); Arc uses an atomic refcount
//    (safe across threads). Rc is the multi-thread combo's Arc<Mutex<T>>.
// Q: When are RefCell's borrow rules checked?
// A: At RUNTIME (not compile time) — a violated borrow panics instead of failing to compile.
// Q: Why isn't Rc Send?
// A: Its refcount isn't atomic; two threads cloning/dropping could corrupt the count —
//    a data race — so the compiler forbids sending Rc across threads.
// Q: Will this compile, and what happens at run time?
//        let r = shared.borrow();
//        let w = shared.borrow_mut(); // ???
//    A: It COMPILES (RefCell defers borrow checking to run time) but PANICS at run time:
//    "already borrowed" — you're holding a shared borrow and asking for a mutable one.
//    Fix: drop `r` first, or don't overlap the borrows. (A plain `&`/`&mut` version of
//    this would instead be a compile error.)
