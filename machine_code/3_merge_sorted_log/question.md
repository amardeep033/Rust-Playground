# Merge 2 Sorted Log Files

## Problem

You are given two log files, each already sorted by timestamp, with lines formatted as:

```
timestamp,message
```

Write a function that merges both files into a single sorted stream (by timestamp) without loading either file fully into memory.

## Function Signature

```rust
fn merge_sorted_logs(
    file_a: &str,
    file_b: &str,
) -> Result<Vec<String>, std::io::Error>
```

## Constraints

- Both input files are sorted ascending by timestamp.
- Stream both files with `BufReader` — don't read either fully into a `Vec` up front.
- Preserve full line content in the output, just interleaved in sorted order.
- If timestamps tie, preserve the line from `file_a` first.

## Sample Input

`a.log`
```
2024-01-01T10:00:01,started service A
2024-01-01T10:00:10,heartbeat A
2024-01-01T10:00:30,shutdown A
```

`b.log`
```
2024-01-01T10:00:05,started service B
2024-01-01T10:00:15,heartbeat B
2024-01-01T10:00:20,heartbeat B
```

## Expected Output

```
2024-01-01T10:00:01,started service A
2024-01-01T10:00:05,started service B
2024-01-01T10:00:10,heartbeat A
2024-01-01T10:00:15,heartbeat B
2024-01-01T10:00:20,heartbeat B
2024-01-01T10:00:30,shutdown A
```
