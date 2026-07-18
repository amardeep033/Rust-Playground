# Shared Counter Service

## Problem

You need to track how many times each service name appears in a stream of concurrent events. Multiple Tokio tasks will each process a batch of events and increment counts in a shared map.

Write a function that spawns multiple async tasks, each incrementing counts for a list of service names into one shared `HashMap<String, usize>`, then returns the final merged counts once all tasks are done.

## Function Signature

```rust
async fn run_counter_service(
    batches: Vec<Vec<String>>,
) -> HashMap<String, usize>
```

## Constraints

- Each inner `Vec<String>` in `batches` is processed by its own spawned Tokio task.
- All tasks share and mutate the same `HashMap<String, usize>`.
- Protect the map correctly — no data races, no lost updates.
- Wait for all tasks to complete before returning the final counts.

## Sample Input

```rust
batches = vec![
    vec!["auth".into(), "auth".into(), "billing".into()],
    vec!["billing".into(), "search".into()],
    vec!["auth".into(), "search".into(), "search".into()],
];
```

## Expected Output

```
auth -> 3
billing -> 2
search -> 3
```
