# Async Pipeline via Channels

## Problem

Implement a 3-stage asynchronous pipeline using Tokio and `tokio::sync::mpsc` channels.

The pipeline consists of three independent tasks:

1. **Producer**
   - Reads numbers from the input vector.
   - Sends each number into Channel 1.
2. **Transformer**
   - Receives numbers from Channel 1.
   - Multiplies each number by 2.
   - Sends the transformed number into Channel 2.
3. **Consumer**
   - Receives numbers from Channel 2.
   - Stores them in a vector.
   - Returns the vector after the pipeline finishes.

Each stage must run in its own `tokio::spawn` task.

## Function Signature

```rust
async fn run_pipeline(input: Vec<i32>) -> Vec<i32>
```

## Constraints

- Use `tokio::sync::mpsc` only.
- Do not use `Arc`, `Mutex`, or `RwLock`.
- Each stage must be a separate spawned task.
- Pipeline should terminate gracefully when producer finishes.
- Preserve ordering.

## Sample Input

```
[1, 2, 3, 4, 5]
```

## Expected Output

```
[2, 4, 6, 8, 10]
```
