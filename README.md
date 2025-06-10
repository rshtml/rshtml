[![Crates.io Version](https://img.shields.io/crates/v/rshtml.svg)](https://crates.io/crates/rshtml)
[![GitHub Repository](https://img.shields.io/badge/github-repo-blue.svg?logo=github)](https://github.com/rshtml/rshtml)
[![Docs.rs Documentation](https://docs.rs/rshtml/badge.svg)](https://docs.rs/rshtml)
[![Full Documentation](https://img.shields.io/badge/book-rshtml.github.io-blue.svg)](https://rshtml.github.io/)
---
# RsHtml

RsHtml is a compile-time, type-safe, lightweight and flexible template engine for Rust, designed to seamlessly integrate Rust code within HTML templates. It allows developers to write dynamic templates with embedded Rust expressions and logic, making it easier to generate HTML content programmatically.

## Features
- Embeds Rust expressions and blocks directly into HTML templates using the `@` prefix or HTML-like component syntax (e.g., `<Component/>`).
- Supports conditional rendering (`@if`, `@else`), loops (`@for`), and pattern matching (`@match`).
- Supports Rust code blocks (`@{}`), various Rust expression syntaxes (e.g., `@expression`, `@(expression)`, and a broad range of other Rust syntax.
- Includes a `section` system, `layout` system, and `component` system.
- Provides helper functions (e.g., `@time()`).
- Supports raw output with `@raw` blocks and server-side comments with `@* ... *@`.
- Generates `efficient Rust code` for template rendering at compile time.
- **See the [documentation](https://rshtml.github.io/) for a full list of features.**
## Syntax Overview

### Condition, Iteration, Pattern Matching
```razor
<h1>Welcome to RsHtml</h1>
@if self.is_logged_in {
    <p>Hello, @self.username!</p>
} else {
    <p>Please log in to continue.</p>
}

<ul>
    @for item in self.items {
        <li>@item</li>
    }
</ul>

@match self.count {
    0 => <p>this is zero</p>,
    1 => true,
    2 => self.count,
    3 => {
        <p>this is @self.count</p>
        @if self.my_var == "rshtml" {
            <p>rshtml</p>
        }
    },
    _ => <p>other</p>
}
```

### Rust Code Blocks
```razor
@{
    let x = 42;
    let y = x * 2;
    println!("Debug: x = {}, y = {}", x, y);

    @: this is text line and x is @x 

    <text>this is text block and y is @y</text>
}
```

### Comments
```razor
@* This is a comment and will not appear in the output *@
```

### Sections and Layout
##### Section Page:
```razor
@section("title", "Home Page")

@section content {
    <p>content section</p>
}

<p>default content</p>
```
##### Layout Page:
```razor
@render("title")

@render_body   @* renders the default content *@

@if has_section("content") {
    <p>content section defined</p>
}

@render("content")
```

### Components
```razor
@use "Component.rs.html" as Component
@use "Component.rs.html" @* take Component as name *@

<Component title="home" is_ok=true>
    <p>child content</p>
</Component>

@Component(title="home", is_ok=true) {
    <p>child content</p>
}
```

#### And much more..

## Installation

To use RsHtml in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
rshtml = "x.y.z"

# The default folder and layout can be changed. This is the default setup:
[package.metadata.rshtml]
views = { path = "views", layout = "layout.rs.html" }
```

## Usage

1. Define your template in an HTML file (e.g., home.rs.html) in `views` folder.
2. Use the `RsHtml` derive macro to parse the template.
3. Render the template with render function.

### main.rs
By default, #[derive(RsHtml)] infers the template file path from the struct's name. 
It converts StructNamePage to struct_name.rs.html. 
You can override this with #[rshtml(path = "...")].
```rust
use rshtml::RsHtml;

#[derive(RsHtml)]
//#[rshtml(path = "about.rs.html")] // Template can change from rshtml path param, relative to views folder.
struct HomePage { // Looks for home.rs.html in views folder.
    title: String,
}

fn main() {
    let mut homepage = HomePage {
        title: "Home Page".to_string()
    };
    
    let result = homepage.render().unwrap();
    
    print!("{}", result);
}
```

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve RsHtml.
