# Pattern

Pattern is a Rust library that implements pattern matching tools to detect PII (Personally Identifiable Information).

This crate is a work in progress and is not ready for production use.

It will be able to detect patterns in text and classify them as:

- person: name, last name, full name
- credit card
- email
- social security number
- location: address, city, state, country
- ip address

## Usage

```rust
use pattern::core::PatternRegistry;

fn main() {
    let registry = PatternRegistry::new();
    let pattern = registry.detect_pattern("hello");
    println!("{}", pattern);
}
```
