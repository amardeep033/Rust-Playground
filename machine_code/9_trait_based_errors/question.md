# Machine Coding: Trait-Based JSON Processing and Error Handling

## Time Limit

30 minutes

## Problem Statement

Build a small Rust library that receives JSON input representing different API commands, validates the input, and returns a structured JSON response.

The goal is to practice Rust error modeling and trait usage with more breadth than business-logic depth.

The system must distinguish between:

* JSON parsing/type errors
* Invalid or missing input fields
* Unsupported command types

Your design should use Rust traits to separate JSON-level behavior and error presentation.

---

## Input Format

The application receives JSON strings representing account operations.

### Create account

```json
{
  "type": "create_account",
  "account_id": "ACC-101",
  "owner": "Alice",
  "initial_balance": 500
}
```

### Withdraw money

```json
{
  "type": "withdraw",
  "account_id": "ACC-101",
  "amount": 200
}
```

### Deposit money

```json
{
  "type": "deposit",
  "account_id": "ACC-101",
  "amount": 100
}
```

The processor may accept either one command object or an array of command objects.

---

## Functional Requirements

### 1. Parse JSON Commands

Create a function similar to:

```rust
fn process_json(
    input: &str,
    error_renderer: &dyn ErrorRenderer,
) -> String
```

The function must parse the JSON, validate the requested command, and return a JSON response.

No account balances need to be stored or updated. Once a command is valid, return an acknowledgement response.

---

### 2. Support Multiple Error Categories

At minimum, support the following errors.

#### JSON Errors

Examples:

* Malformed JSON
* Incorrect JSON field type

#### Request/Validation Errors

Examples:

* Missing required field
* Empty account ID
* Empty owner for account creation
* Zero or negative transaction amount

#### Command Errors

Examples:

* Unsupported `"type"` value

---

### 3. Local Trait Implemented for an External Type

Define a local trait that adds application-specific behavior to an external JSON-related type.

For example:

```rust
trait JsonErrorDetails {
    fn error_code(&self) -> &'static str;
    fn readable_message(&self) -> String;
}
```

Implement it for an external type such as:

```rust
serde_json::Error
```

The implementation should convert a `serde_json::Error` into an application-specific error code and readable message.

---

### 4. External Trait Implemented for a Local Type

Create a local application error type:

```rust
enum AppError {
    Json(...),
    Validation(...),
    UnsupportedCommand(...),
}
```

Implement at least one external standard-library trait for it.

Possible traits include:

```rust
std::fmt::Display
std::error::Error
```

The error should be printable using:

```rust
println!("{error}");
```

---

### 5. Use `thiserror` and `#[from]`

Use the `thiserror` crate to derive `std::error::Error` for your local error type.

At minimum:

* Add `thiserror` to `Cargo.toml`.
* Use `#[derive(Debug, thiserror::Error)]` on `AppError`.
* Use `#[from]` on the JSON error variant so `serde_json::Error` can be converted into `AppError` automatically.
* Use the `?` operator with `serde_json` parsing calls so the `#[from]` conversion is exercised.

Example shape:

```rust
#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Validation error: {message}")]
    Validation {
        code: &'static str,
        message: String,
    },
}
```

You may still use your local `JsonErrorDetails` trait for rendering JSON error codes/messages.

---

### 6. Config-Driven Error Rendering

The API must support configurable error-response styles.

Define a trait such as:

```rust
trait ErrorRenderer {
    fn render(&self, error: &AppError) -> String;
}
```

Provide at least two implementations.

#### Detailed Renderer

Example response:

```json
{
  "success": false,
  "error": {
    "code": "INVALID_AMOUNT",
    "message": "amount must be greater than zero for deposit",
    "category": "validation"
  }
}
```

#### Minimal Renderer

Example response:

```json
{
  "success": false,
  "error": "Request failed"
}
```

Create the renderer based on configuration:

```rust
enum ErrorMode {
    Detailed,
    Minimal,
}
```

Provide a factory function similar to:

```rust
fn create_error_renderer(mode: ErrorMode) -> Box<dyn ErrorRenderer>
```

The main JSON-processing function should not contain renderer-specific conditionals.

---

## Successful Responses

### Single Command Accepted

```json
{
  "success": true,
  "message": "Command accepted",
  "command": "deposit"
}
```

### Multiple Commands Accepted

```json
{
  "success": true,
  "results": [
    {
      "success": true,
      "message": "Command accepted",
      "command": "create_account"
    }
  ]
}
```

---

## Example Scenarios

### Malformed JSON

Input:

```json
{
  "type": "withdraw",
  "account_id": "ACC-101",
```

Expected result:

* Convert `serde_json::Error` into `AppError` via `#[from]`.
* Use the local `JsonErrorDetails` trait while rendering.
* Return an error using the configured renderer.

---

### Unsupported Command

Input:

```json
{
  "type": "close_account",
  "account_id": "ACC-101"
}
```

Expected result:

* Return an unsupported-command error.
* Do not panic.

---

### Invalid Amount

Input:

```json
{
  "type": "deposit",
  "account_id": "ACC-101",
  "amount": -50
}
```

Expected result:

* Return a request-validation error.

---

## Design Constraints

Your implementation must demonstrate all of the following:

1. A local trait implemented for an external type.
2. An external trait implemented for a local type.
3. `thiserror` with `#[from]`.
4. Runtime/config-driven behavior using trait objects.
5. Separate JSON, validation, and command errors.
6. No panics for invalid user input.
7. Meaningful use of `Result` and the `?` operator.
8. Validation logic separated from JSON-response formatting.

---

## Suggested Structure

```text
src/
├── main.rs
├── err.rs
├── req.rs
└── renderer.rs
```

A single-file implementation is also acceptable for the interview.

---

## Bonus Requirements

Complete these only if time permits:

* Add unit tests for malformed JSON, unsupported command, and invalid amount.
* Add a third renderer that returns HTTP-style status codes.
* Make `process_json` generic instead of using trait objects and explain the trade-off.
