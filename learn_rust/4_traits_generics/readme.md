# 4 · traits, generics, box & dispatch

Copy one `src/N_*.rs` into `src/main.rs`, then `cargo run`.

## files

| file | topic |
|---|---|
| `1_str_types_and_dst.rs` | `str` vs `&str` vs `String`, memory layout, DSTs behind pointers |
| `2_traits.rs` | trait = contract, default methods, `&impl Trait` param |
| `3_common_traits.rs` | derive `Debug`/`Clone`/`PartialEq`, impl `Display`, `From`/`Into` |
| `4_generics_and_dispatch.rs` | generic bounds, static (monomorphization) vs dynamic (`dyn`+vtable) |
| `5_generics_deep.rs` | generic struct, `where` clause, blanket impl, supertrait |
| `6_box_and_trait_objects.rs` | `Box` heap/recursive types, `Box<dyn Trait>` heterogeneous collection |

## pointers

| topic | point |
|---|---|
| str | raw UTF-8, a DST (unknown size) — never used bare, always behind `&`/`Box` |
| &str | fat pointer (ptr+len), borrowed & immutable; prefer as fn param (accepts `&String` too) |
| String | (ptr+len+cap) heap-owned, growable |
| DSTs | the three: `str`, `[T]`, `dyn Trait` — must live behind a pointer |
| no inheritance | shared behaviour = traits + composition, not `extends` |
| default method | trait can supply a body; impls override or inherit it |
| static dispatch | generics / `impl Trait` → monomorphized, zero overhead, one type per call |
| dynamic dispatch | `dyn Trait` → vtable lookup at runtime, allows mixed types in one `Vec` |
| impl Trait return | one hidden concrete type; can't return two types → use `Box<dyn Trait>` |
| Box why | single heap owner: recursive types (breaks infinite size), trait objects, big values |
| Vec<dyn> fails | elements need equal size; `Box<dyn>` gives every element a same-size fat pointer |
| object safety | no `-> Self` / generic methods in a `dyn` trait (vtable needs fixed layout) → no `dyn Clone` |
| Debug vs Display | `Debug` (`{:?}`) is derivable; `Display` (`{}`) must be hand-written via `fmt::Display` |
| From/Into | impl `From<T>` → get `Into` free; `?` uses `From` to convert error types |
| where clause | same as inline bounds, cleaner for many/complex bounds |
| blanket impl | `impl<T: Bound> Trait for T` — implements a trait for every qualifying type (e.g. `ToString`) |
| supertrait | `trait A: B` — implementors of `A` must also implement `B` |

## why `Vec<Box<dyn T>>`, not `Vec<dyn T>` (memory layout)

A `Vec` stores elements contiguously, so every element must be the **same size**.

```
Vec<String>            every String = (ptr,len,cap), 24B → uniform ✓
  [ (p,l,c) | (p,l,c) ]
       │          └─────► heap: "world"
       └─► heap: "hi"                    (2 heap levels: buffer → each String's bytes)

Vec<dyn Animal>        Dog = 0B, Cat = 24B → different sizes ✗  (won't compile)

Vec<Box<dyn Animal>>   every Box<dyn> = fat pointer (data*, vtable*), 16B → uniform ✓
  [ (d,v) | (d,v) ]
      │        └─► heap: Cat { name }        vtable* → speak() impl
      └─► heap: Dog
```

Box gives each element a **same-size fat pointer** (data ptr + vtable ptr), each
pointing to its own differently-sized value on the heap.

## common questions

**Q: `impl Trait` vs generic `<T>` vs `dyn Trait` vs `Box<dyn>` vs `&dyn` — when each?**

| form | use it when |
|---|---|
| `fn f(x: &impl Trait)` | simplest generic param, don't need to name the type |
| `fn f<T: Trait>(x: &T)` | you must NAME `T` — return it, or force two params to the same type |
| `-> impl Trait` | return ONE hidden concrete type (an iterator/closure), zero cost |
| `dyn Trait` | need MIXED concrete types together, or a type chosen at runtime (behind `&`/`Box`) |
| `Box<dyn Trait>` | OWN a trait object: store in a struct field, return different concrete types, put in a `Vec` |
| `&dyn Trait` | just BORROW a trait object — no ownership, no heap |

**Q: When *only* `dyn` (generics won't do)?**
Heterogeneous collections (`Vec<Box<dyn Trait>>`), returning different concrete types from one function, or plugin-style runtime dispatch. Generics fix one type per instantiation, so they can't hold a mix.

**Q: When *only* `Box` (a reference won't do)?**
When you need **ownership** of heap data: recursive types (a `&` can't own the next node), returning an owned trait object, or storing one in a struct that outlives the caller.

**Q: `String` field vs `&str` field in a struct?**
`String` = the struct OWNS the text (no lifetime needed, simplest). `&str` field = the struct BORROWS text owned elsewhere (needs `<'a>`, can't outlive it) — only worth it to avoid a copy.
