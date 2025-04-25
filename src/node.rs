#[derive(Debug, Clone, PartialEq)]
pub enum TextBlockItem {
    Text(String),
    RustExprSimple(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextLineItem {
    Text(String),
    RustExprSimple(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum RustBlockContent {
    Code(String),
    TextLine(Vec<TextLineItem>),
    TextBlock(Vec<TextBlockItem>),
    NestedBlock(Vec<RustBlockContent>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum SectionDirectiveContent {
    Text(String),
    RustExprSimple(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Template(Vec<Node>),              // main template, contains child nodes
    Text(String),                     // plain text content (@@ -> @)
    InnerText(String),                // text inside a block (@@ -> @, @{ -> {, @} -> })
    Comment(String),                  // comment content
    IncludeDirective(String),         // include directive @include("other_view.html")
    ExtendsDirective(String),         // extends directive @extends("layout.html")
    YieldDirective(String),           // yield directive @yield("content")
    RustBlock(Vec<RustBlockContent>), // @{ ... } block content (with trim)
    RustExprSimple(String),           // @expr ... (simple expression)
    RustExprParen(String),
    RustExpr {
        // @if ...  { ... } else { ... } / @for ... { ... }
        clauses: Vec<(String, Vec<Node>)>,
    },
    SectionDirective(String, SectionDirectiveContent), // @section("content")
    SectionBlock(String, Vec<Node>),                   // @section content { ... }
}
