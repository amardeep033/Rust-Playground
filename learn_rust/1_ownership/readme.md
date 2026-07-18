# 1 · ownership & borrowing

Copy one `src/N_*.rs` into `src/main.rs`, then `cargo run`. Files aren't wired
into the binary on their own.

## files

| file | topic |
|---|---|
| `1_move_vs_copy.rs` | move (String) vs copy (i32), moving into functions |
| `2_borrowing.rs` | `&T` vs `&mut T` rules, NLL (borrow ends at last use) |
| `3_dangling_and_returns.rs` | can't move while borrowed, return owned not `&local` |

## pointers

| topic | point |
|---|---|
| owner | every value has exactly 1 owner; dropped when owner leaves scope |
| move | non-Copy assign = ownership transfers, original invalid (`String`, `Vec`, `Box`) |
| copy | Copy assign = bit duplicate, both valid (`i32`,`bool`,`char`,`&T`, tuples of Copy) |
| borrow value | you borrow the *value* (`&T`/`&mut T`) or *move* ownership — ownership itself isn't borrowed |
| aliasing | many `&T` OR one `&mut T`, never both at once |
| NLL | borrow lives until its last use, not until `}` |
| dangling | can't return `&` to a local (it drops); return owned `T` instead |
| mutate | mutation needs `let mut` binding AND a `&mut` borrow |
| what it prevents | use-after-free, double-free, dangling pointers, data races |
| UAF vs dangling | dangling pointer = an invalid reference *exists* (state); use-after-free = *dereferencing* it (action) |
