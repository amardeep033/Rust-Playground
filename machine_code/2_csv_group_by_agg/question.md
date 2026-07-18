# CSV Group-By / Aggregate

## Problem

You are given a CSV file where each line has the format:

```
id,name,department,salary
```

Write a function that returns the total salary for each department.

## Function Signature

```rust
fn group_by_department_salary(
    file_path: &str,
) -> Result<HashMap<String, u32>, Box<dyn Error>>
```

## Constraints

- Read the file line by line.
- Ignore the header row.
- Ignore malformed rows.
- Salary is an unsigned integer.
- Return a `HashMap<Department, TotalSalary>`.

## Sample Input (`employees.csv`)

```
id,name,department,salary
1,Alice,Engineering,90000
2,Bob,Sales,60000
3,Charlie,Engineering,95000
4,Dave,Marketing,70000
5,Eve,Sales,65000
6,Frank,Engineering,80000
```

## Expected Output

```
Engineering -> 265000
Sales -> 125000
Marketing -> 70000
```
