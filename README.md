Parse and validate Web [Content-Security-Policy level 3](https://www.w3.org/TR/CSP/)

# Example

```rust
fn main() {
    let csp_list = CspList::parse("script-src notriddle.com");
    assert_eq!(csp_list.m)
}
```

# Installation

To use `content-security-policy`, add it to your project's `Cargo.toml` file:

```toml
[dependencies]
content-security-policy = "0.0.1"
```
