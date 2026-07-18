# Columnar Filter + Count

## Problem

You are given log data stored in three parallel, index-aligned vectors:

```rust
timestamp: Vec<u64>
service:   Vec<String>
level:     Vec<String>
```

Write a function that returns the count of `"ERROR"` level logs per service.

## Function Signature

```rust
fn count_errors_per_service(
    timestamp: &[u64],
    service: &[String],
    level: &[String],
) -> HashMap<String, usize>
```

## Constraints

- The three slices are the same length and index-aligned.
- Only `"ERROR"` level entries should be counted.
- `timestamp` is not needed for this specific query — don't touch it.

## Sample Input

```rust
timestamp = [1, 2, 3, 4, 5, 6];
service   = ["auth", "billing", "auth", "search", "billing", "auth"];
level     = ["ERROR", "INFO", "ERROR", "ERROR", "ERROR", "INFO"];
```

## Expected Output

```
auth -> 2
billing -> 1
search -> 1
```