# RsHtml

RsHtml is a lightweight and flexible template engine for Rust, designed to seamlessly integrate Rust code within HTML templates. It allows developers to write dynamic templates with embedded Rust expressions and logic, making it easier to generate HTML content programmatically.

## Features
- Embeds Rust expressions and blocks directly into HTML templates using the @ prefix or HTML-like component syntax (e.g., `<Component/>`).
- Supports conditional rendering (`@if`, `@else`), loops (`@for`), and pattern matching (`@match`).
- Supports Rust code blocks (`@{}`), various Rust expression syntaxes (e.g., `@expression`, `@(expression))`, and a broad range of other Rust syntax.
- Includes a section system, layout system, and component system.
- Provides helper functions (e.g., `@time()`).
- Generates efficient Rust code for template rendering at compile time.

## Syntax Overview

### Basic Example
```html
<h1>Welcome to RsHtml</h1>

@if self.is_logged_in {
    <p>Hello, @self.username!</p>
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

    @: this is text line and x is @x 

    <text>this is text block and y is @y</text>
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
rshtml = "0.1.0"
```

## Usage

1. Define your template in an HTML file.
2. Use the `RsHtml` derive macro to parse the template.
3. Render the template with render function.

### main.rs
```rust
use rshtml::RsHtml;

#[derive(RsHtml)]
//#[rshtml(path = "about.rs.html")] // template can change
// otherwise it take struct name without Page section and make lowercase and adds rs.html
struct HomePage {
    title: String,
}

fn main() {
    let mut homepage = HomePage {
        title: "Home Page".to_string()
    };
    
    let result = homepage.render().unwrap();
    
    print!("{}", s);
}
```

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve RsHtml.
