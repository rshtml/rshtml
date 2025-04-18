# RsHtml

RsHtml is a lightweight and flexible template engine for Rust, designed to seamlessly integrate Rust code within HTML templates. It allows developers to write dynamic templates with embedded Rust expressions and logic, making it easier to generate HTML content programmatically.

## Features

- Embed Rust expressions and blocks directly into HTML templates.
- Support for conditional rendering (`@if`, `@else`) and loops (`@for`).
- Customizable syntax for clean and readable templates.
- Generates efficient Rust code for rendering templates at runtime.

## Syntax Overview

### Basic Example
```html
<h1>Welcome to RsHtml</h1>

@if is_logged_in {
    <p>Hello, @username!</p>
} else {
    <p>Please log in to continue.</p>
}

<ul>
    @for item in items {
        <li>@item</li>
    }
</ul>
```

### Rust Code Blocks
```html
@{
    let x = 42;
    let y = x * 2;
    println!("Debug: x = {}, y = {}", x, y);
}
```

### Comments
```html
@* This is a comment and will not appear in the output *@
```

## Installation

To use RsHtml in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
rshtml = { path = "path/to/rshtml" }
```

## Usage

1. Define your template in an HTML file.
2. Use the `parse_template` function to parse the template into an Abstract Syntax Tree (AST).
3. Render the AST or process it as needed.

### Example
```rust
use rshtml::{parse_template, view_node};

fn main() {
    let template = r#"
        <h1>Hello, @name!</h1>
        @if is_admin {
            <p>Welcome, admin!</p>
        }
    "#;

    let ast = parse_template(template).expect("Failed to parse template");
    view_node(&ast, 0); // Visualize the AST
}
```

## Development

### Running Tests
To run the test suite, use:
```bash
cargo test
```

### Debugging Templates
Use the `view_node` function to visualize the parsed AST for debugging purposes.

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve RsHtml.
