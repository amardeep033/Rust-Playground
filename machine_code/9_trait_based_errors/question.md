# Machine Coding: Trait-Based JSON Processing and Error Handling

## Time Limit

30 minutes

## Problem Statement

Build a small Rust library that receives JSON input representing different API commands, validates the input, executes the corresponding domain operation, and returns a structured response.

The system must distinguish between:

* JSON parsing errors
* Invalid or missing input fields
* Unsupported command types
* Domain/business-rule errors

Your design should use Rust traits to separate JSON-level behavior, domain behavior, and error presentation.

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

---

## Functional Requirements

### 1. Parse JSON commands

Create a function similar to:

```rust
fn process_json(
    input: &str,
    service: &dyn AccountService,
    error_renderer: &dyn ErrorRenderer,
) -> String
```

The function must parse the JSON, execute the requested operation, and return a JSON response.

---

### 2. Support multiple error categories

At minimum, support the following errors.

#### JSON errors

Examples:

* Malformed JSON
* Incorrect JSON field type
* Missing required fields

#### Request errors

Examples:

* Unsupported `"type"` value
* Empty account ID
* Zero or negative transaction amount

#### Domain errors

Examples:

* Account already exists
* Account not found
* Insufficient balance

---

### 3. Local trait implemented for an external type

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

### 4. External trait implemented for a local type

Create a local application error type:

```rust
enum AppError {
    Json(...),
    InvalidRequest(...),
    Domain(...),
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

### 5. Config-driven API behavior

The API must support configurable error-response styles.

Define a trait such as:

```rust
trait ErrorRenderer {
    fn render(&self, error: &AppError) -> String;
}
```

Provide at least two implementations.

#### Detailed renderer

Example response:

```json
{
  "success": false,
  "error": {
    "code": "INSUFFICIENT_BALANCE",
    "message": "Account ACC-101 does not have enough balance",
    "category": "domain"
  }
}
```

#### Minimal renderer

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

## Domain Interface

Define an account-service trait:

```rust
trait AccountService {
    fn create_account(
        &self,
        account_id: &str,
        owner: &str,
        initial_balance: i64,
    ) -> Result<(), DomainError>;

    fn deposit(
        &self,
        account_id: &str,
        amount: i64,
    ) -> Result<i64, DomainError>;

    fn withdraw(
        &self,
        account_id: &str,
        amount: i64,
    ) -> Result<i64, DomainError>;
}
```

Implement an in-memory version using a suitable collection such as:

```rust
HashMap<String, Account>
```

The returned balance from `deposit` and `withdraw` should represent the updated account balance.

---

## Successful Responses

### Account created

```json
{
  "success": true,
  "message": "Account created"
}
```

### Deposit or withdrawal completed

```json
{
  "success": true,
  "balance": 400
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

* Convert `serde_json::Error` using the local extension trait.
* Return an error using the configured renderer.

---

### Unsupported command

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

### Invalid amount

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

### Insufficient balance

Input:

```json
{
  "type": "withdraw",
  "account_id": "ACC-101",
  "amount": 5000
}
```

Expected result:

* Return a domain error.
* Render it according to the configured error mode.

---

## Design Constraints

Your implementation must demonstrate all of the following:

1. A local trait implemented for an external type.
2. An external trait implemented for a local type.
3. Runtime/config-driven behavior using trait objects.
4. Separate JSON, validation, and domain errors.
5. No panics for invalid user input.
6. Meaningful use of `Result` and the `?` operator.
7. Business logic separated from JSON-response formatting.

---

## Suggested Structure

```text
src/
├── main.rs
├── command.rs
├── domain.rs
├── error.rs
└── renderer.rs
```

A single-file implementation is also acceptable for the interview.

---

## Bonus Requirements

Complete these only if time permits:

* Add an error source chain using `std::error::Error::source`.
* Use `#[from]` with the `thiserror` crate.
* Add unit tests for malformed JSON and insufficient balance.
* Add a third renderer that returns HTTP-style status codes.
* Make `process_json` generic instead of using trait objects and explain the trade-off.
