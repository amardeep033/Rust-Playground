# Trait-Based Errors

## 0. Trait Implementation Rules

| Trait    | Type     | Allowed? |
| -------- | -------- | -------: |
| Local    | Local    |      Yes |
| Local    | External |      Yes |
| External | Local    |      Yes |
| External | External |       No |

# Enum Dispatch vs `dyn Trait` Dispatch vs Config-Driven Behavior

Notes consolidating the event-handler example (JSON → `EventTypeEnum` vs `Box<dyn EventHandler>`)
and the `Renderer` example, into one reference.

---

## 1. Two ways to route to different behavior in Rust

| | **Enum + `match`** | **`dyn Trait` (trait object)** |
|---|---|---|
| Example | `EventTypeEnum::Email(EmailEvent)` | `Box<dyn EventHandler>` |
| Set of variants | **Closed** — every variant known at compile time, in this crate | **Open** — new implementors can appear without touching/recompiling this code |
| Dispatch mechanism | `match` picks the concrete type → call resolved at compile time | vtable lookup at runtime |
| Dispatch type | **Static** | **Dynamic** |
| Compiler help | Exhaustiveness checked — forgetting a variant is a compile error | No exhaustiveness check possible — there's no fixed list to check against |
| Cost | No heap alloc (unless you choose to Box it), no vtable, can inline | Heap allocation (`Box`) + vtable indirection per call |
| Add a new kind | Requires editing this file, adding a variant, handling it everywhere it's matched | Just implement the trait for a new type and register/construct it — call sites don't change |

**Rule of thumb:** ask *"can someone add a new case without touching or recompiling this specific file?"*
- **No** → closed set → use the enum.
- **Yes** → open set → use `dyn Trait` (a registry/plugin-style system needs this, because a `match` can't be exhaustive over code that doesn't exist yet).

---

## 2. Where static dispatch actually lives (the part that's easy to miss)

```rust
impl EventHandler for EventTypeEnum {
    fn print(&self) {
        match self {
            EventTypeEnum::Email(email) => email.print(),  // <- static dispatch
            EventTypeEnum::Push(push) => push.print(),      // <- static dispatch
        }
    }
}
```

- The `match` itself is **not** dispatch in the trait sense — it's an ordinary control-flow branch on the enum's tag (like a jump table), used to *unpack* the enum into its concrete inner type.
- Once unpacked, `email` has concrete type `&EmailEvent` (not `&dyn EventHandler`). The compiler knows exactly which function `email.print()` calls — resolved at **compile time**. That's the static dispatch.
- Contrast with true dynamic dispatch: if you had `handler: &dyn EventHandler` and called `handler.print()`, the compiler does **not** know at compile time whether it's an `EmailEvent` or `PushEvent` — that's resolved at runtime via a vtable.

So: **"deciding which variant via `match`"** and **"dynamic trait dispatch via vtable"** are two different mechanisms that can solve a similar-looking problem. The event system file uses only the former — there is no `dyn Trait` anywhere in it, so all dispatch in it is static.

---

## 3. What "config-driven" actually means (and what it doesn't)

**Config-driven** = some external data (a string, a JSON field, a config file, a DB row) decides *which* code path runs, instead of the caller hardcoding the choice in Rust. This is true of **both** the enum version and a registry version — it's an orthogonal axis to open/closed sets above.

Two things get conflated under "config-driven," and it's worth keeping them separate:

- **Axis 1 — what selects behavior?** Runtime data (e.g. `raw_event.event_kind.as_str()`, `config.mode.as_str()`) picks a branch. True in both designs.
- **Axis 2 — is the set of possible types fixed or extensible?** This is the axis that actually decides enum vs `dyn Trait` (see table above). "Config-driven" by itself says nothing about this — you can be config-driven with a closed set (pick one of N known handlers) or config-driven with an open set (pick from an arbitrary, pluggable registry).

### Closed-set config-driven (fits the event system and the `Renderer` example)
```rust
enum Renderer {
    Detailed,
    Minimal,
}

impl Renderer {
    fn render(&self, error: &AppError) -> String {
        match self {
            Renderer::Detailed => format!("Detailed error: {error}"),
            Renderer::Minimal => "Request failed".to_string(),
        }
    }
}

// the actual "config chooses" step — usually happens once, e.g. at startup:
let renderer = match config.mode.as_str() {
    "detailed" => Renderer::Detailed,
    _ => Renderer::Minimal,
};
```
Config only picks *which of the known set* to use — it can never introduce a brand-new kind. Every call to `.render()` after that is static dispatch.

### Open-set config-driven (registry / plugin style — NOT needed here, shown for contrast)
```rust
trait ErrorRenderer {
    fn render(&self, error: &AppError) -> String;
}
struct Detailed;
struct Minimal;
impl ErrorRenderer for Detailed { fn render(&self, e: &AppError) -> String { format!("Detailed error: {e}") } }
impl ErrorRenderer for Minimal  { fn render(&self, _: &AppError) -> String { "Request failed".into() } }

let renderer: Box<dyn ErrorRenderer> = match config.mode.as_str() {
    "detailed" => Box::new(Detailed),
    _ => Box::new(Minimal),
};
```
Same observable behavior for two modes — but here new renderer *kinds* could be registered from elsewhere (a plugin crate, a separate module, a `HashMap<String, Box<dyn ErrorRenderer>>` built at startup) without ever touching this match. That flexibility is the entire reason to pay for the vtable + heap allocation. If you only ever have two fixed modes, this version is strictly worse than the enum — same behavior, extra cost, no compiler exhaustiveness check.

---

## 4. Two different jobs an enum can do

Worth keeping distinct, since the two examples in this session used the enum differently:

- **Payload-carrying sum type** — `EventTypeEnum::Email(EmailEvent)`. Different input *and* different behavior bundled per variant. Used when the data itself differs by case.
- **Pure mode/strategy selector** — `Renderer::Detailed` / `Renderer::Minimal`. Same input (`&AppError`) every time; only the *behavior* differs per case. No payload at all.

Both are legitimate, both are enum + static dispatch — just answering slightly different design questions ("what data do I have" vs "what algorithm do I run").

---

## 5. One-line summary

> Config-driven ≠ dynamic dispatch. Config-driven just means *runtime data picks the branch*. Whether that branch is a `match` on a closed enum (static, safe, fast) or a `dyn Trait` lookup in an open registry (dynamic, flexible, costs more) depends entirely on whether the set of possible behaviors is fixed and known to you at compile time, or genuinely open to extension by code you don't control.