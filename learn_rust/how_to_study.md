# How to study so you find your *wrong* understanding

The problem with misconceptions is that they feel exactly like knowledge from the
inside. You "knew" struct lifetimes for 3 years and still missed that a string
literal is `&'static` — because nothing ever *forced* the gap into the open. These
techniques force it.

## the one rule

> Treat every "obviously it…" as an **untested hypothesis**, not a fact.
> The word "obviously" is where misconceptions hide.

## techniques (in order of power)

| # | technique | why it exposes wrong understanding |
|---|---|---|
| 1 | **Predict before you run** | Write down "compiles? / output? / which error?" *before* running. The gap between your prediction and reality IS the misconception. A snippet that runs as expected teaches nothing; a surprise teaches everything. |
| 2 | **Minimal contrasting pairs** | Change ONE thing (`&s` → `"literal"`, `i32` → `String`, `&` → `&mut`) and ask "did behaviour change? why?". If you can't predict which of two near-identical snippets compiles, you found a gap. |
| 3 | **Break it on purpose** | Write the *wrong* version deliberately, run it, read `rustc --explain E0515`, then fix. The compiler errors are the syllabus — don't avoid them, manufacture them. |
| 4 | **Explain the "why" out loud** | Teach it to an imaginary junior. The instant you say "it just… works" you've found a gap. Hand-waving = misconception. |
| 5 | **Verify, don't recall** | For memory-layout / desugaring / "is this Copy?" claims, CHECK the source — not your memory. `size_of`, `cargo expand`, the docs. Your recall is exactly what might be wrong. |
| 6 | **Ask boundary + cost + use** | Not "why does this work" but: "why does the obvious alternative NOT work?", "what does it cost?", "when would I actually reach for this?". Recited facts die on these; real understanding survives. |

## verify-it tools (use instead of guessing)

| tool | reveals |
|---|---|
| `rustc --explain E0502` | the actual rule behind an error code |
| Rust Playground | tweak + share; "Tools → Expand macros / MIR / ASM" |
| `cargo expand` | what `?`, `async`, `#[derive]`, `println!` desugar to |
| `std::mem::size_of::<T>()` / `align_of` | prove a memory-layout claim instead of believing it |
| `dbg!(x)` | value + type at a point, no guessing |
| The Reference + Rustonomicon | the precise rules (elision, coercion, variance, Send/Sync) |
| `cargo clippy` | idioms — teaches you the *better* way you didn't know |

## misconception red flags (catch yourself)

- You say **"it just works"** / **"obviously"** → stop, predict, run.
- You **avoid** writing the broken version → that avoidance is hiding a gap.
- You can state the rule but **can't predict a fresh snippet** → you memorised, didn't understand.
- You know **when to use X** but not **why not Y** → you know the center, not the boundary.
- You believe a **memory-layout / performance** claim you never measured → measure it.

## how to use this repo with the above

1. Read only the **`// Q:`** at the top. Cover the code and the `// A:`.
2. **Predict** out loud: compiles? output? error code?
3. Reveal the code, run it, compare to your prediction. Note every surprise.
4. Read the `// A:` and cross-questions — answer those before reading their answers too.
5. Then **break it**: change one thing, predict again, run. That's where the real learning is.
