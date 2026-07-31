# 7 · references & conversion helpers

Copy one `src/N_*.rs` into `src/main.rs`, then `cargo run`. Files aren't wired
into the binary on their own.

## files

| file | topic |
|---|---|
| `1_ref_as_ref_borrow_as_ptr.rs` | `&x` vs `.as_ref()` vs `.borrow()` vs `.as_ptr()` |
| `2_mut_as_mut_borrow_mut_as_mut_ptr.rs` | `&mut x` vs `.as_mut()` vs `.borrow_mut()` vs `.as_mut_ptr()` |
| `3_deref_as_deref_refcell.rs` | `*x`, `.deref()`, deref coercion, `.as_deref()`, `RefCell::borrow()` |
| `4_raw_pointer_boundary.rs` | raw pointers don't borrow; `unsafe` is where validity is checked by you |
| `5_refcell_without_rc_and_friends.rs` | when to prefer `&mut`, `RefCell` without `Rc`, plus `Cell`, `OnceCell`, shared mutation tools |

## pointers

| topic | point |
|---|---|
| `&x` | borrow this exact value now; type is usually `&T` or `&mut T` |
| coercion | Rust may coerce `&String` → `&str`, `&Vec<T>` → `&[T]`, `&Box<T>` → `&T` at call sites |
| `AsRef` | cheap generic conversion to a shared view; great for APIs that accept many input types |
| `AsMut` | cheap generic conversion to a mutable view; often `Vec<T>` → `&mut [T]` |
| `Borrow` | like `AsRef`, but promises borrowed and owned forms behave the same for Eq/Hash/Ord |
| `BorrowMut` | mutable version of `Borrow`; less common than `AsMut` |
| `as_ptr` | raw pointer to bytes/elements; no borrow rules, no length, no safety guarantee by itself |
| `as_mut_ptr` | mutable raw pointer; using it needs `unsafe` and you must avoid aliasing bugs |
| `*x` | dereference; copies/moves/borrows the value behind a pointer-like thing depending on context |
| `Deref` | lets smart pointers act like references; powers deref coercion |
| `as_deref` | converts `Option<String>` → `Option<&str>` or `Option<Box<T>>` → `Option<&T>` |
| `RefCell` | gives runtime checked borrows; wrong overlap panics instead of failing compile |
| `Cell` | interior mutability for small `Copy` values; get/set the whole value |
| `OnceCell` | interior mutability for set-once values that can later be read through `&self` |
| `Rc` | shared ownership in one thread; combine with `RefCell` only when shared mutation is needed |
| safe vs unsafe | references encode validity in the type system; raw pointers move that promise into `unsafe` code |

## quick chooser

| need | reach for |
|---|---|
| just read this value | `&x` |
| just mutate this value | `&mut x` |
| mutate from your own method and callers can give exclusive access | `&mut self` |
| mutate from an API that only has shared access | `RefCell<T>` or `Cell<T>` |
| generic API accepting `String`, `&str`, `PathBuf`, etc. | `T: AsRef<str/path/etc>` |
| map lookup with owned keys and borrowed lookup keys | `Borrow` (usually already used by std) |
| inspect bytes / pass address to FFI | `.as_ptr()` plus length |
| mutate through FFI / low-level buffer work | `.as_mut_ptr()` inside a tight `unsafe` boundary |
| convert `Option<String>` to `Option<&str>` without moving | `.as_deref()` |
| mutate internal state through `&self` with one owner | `RefCell<T>` or `Cell<T>`; prefer `&mut self` if possible |
| share mutable state between many single-thread owners | `Rc<RefCell<T>>` |
| share mutable state between threads | `Arc<Mutex<T>>` or `Arc<RwLock<T>>` |
