# quikreqwest

Reqwest, but less verbose.
`quikreqwest` helps in writing code that's efficient without boilerplate, improving developer experience.

# Getting started

1. Install this as a dependency by adding this line to `Cargo.toml`

```toml
[dependencies]
quikreqwest = { git = "https://github.com/The-HackeLite/quikreqwest" }
```

2. Create a new client by:

```rust
let client = quikreqwest::Request::new(user_agent, headers);
```

Get started with making requests!

# Making requests

`quikreqwest` supports making `GET` requests as of now and serialize responses to text and JSON.

The functions are named as per the following conventions:

<HTTP-method>_<return-data-format>

1. `get_text`: Returns data in textual format.
2. `get_json`: Returns data in JSON format.
3. `get_text_with_headers`: Returns text, and supports provision of headers.
4. `get_json_with_headers`: Returns JSON, and supports provision of headers.

Providing URL for every request function is mandatory.

# Example

## Getting text data from a website listing
```rust
let content = self.client.get_text("https://example.com").await;
match content {
    Ok(content) => {
        println!("{content:#?}");
    }
    Err(error) => {
        eprintln!("{e:#?}");
    }
}
```
