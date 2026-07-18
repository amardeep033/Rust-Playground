# Async Web Scraper Service

## Problem

Build an `actix-web` HTTP server with a single route that, on each request, fetches an external page, scrapes it, and returns the result mixed with a path parameter.

The handler should:

1. Accept a `name` path segment (`GET /{name}`).
2. Fetch `https://books.toscrape.com/` with `reqwest`.
3. Parse the HTML with `scraper` and select every book title (`article.product_pod h3 a`, reading the `title` attribute).
4. Respond with `"Hello {name} -- {titles:?}"`.

## Function Signature

```rust
#[get("/{name}")]
async fn index(name: web::Path<String>) -> HttpResponse;

#[actix_web::main]
async fn main() -> std::io::Result<()>;
```

## Constraints

- Server binds to `127.0.0.1:8080`.
- Use `middleware::Compress` on the `App`.
- Scraping happens per-request (no caching) — each call re-fetches the page.
- Selector parsing failures should not panic in production code; current implementation uses `.unwrap()` for brevity — revisit with `?`/error responses if hardening this later.

## Sample Input

```
GET http://127.0.0.1:8080/world
```

## Expected Output

```
Hello world -- ["A Light in the Attic", "Tipping the Velvet", ...]
```
