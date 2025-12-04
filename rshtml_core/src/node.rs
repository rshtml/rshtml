use crate::position::Position;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone)]
pub enum SectionDirectiveContent {
    Text(String),
    RustExprSimple(String, bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ComponentParameterValue {
    Bool(bool),
    Number(String),
    String(String),
    RustExprParen(String),
    RustExprSimple(String),
    Block(Vec<Node>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ComponentParameter {
    pub name: String,
    pub value: ComponentParameterValue,
    pub position: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Template(String, Vec<Node>, Position), // main template, contains child nodes
    Text(String),                          // plain text content (@@ -> @)
    InnerText(String),                     // text inside a block (@@ -> @, @{ -> {, @} -> })
    Comment(String),                       // comment content
    PropsDirective(Vec<(String, String, Position)>, Position),
    IncludeDirective(PathBuf, Box<Node>), // include directive @include("other_view.rs.html")
    RenderDirective(String),              // render directive @render("content")
    RustBlock(String, Position),          // @{ ... } block content (with trim)
    RustExprSimple(String, bool, Position), // @expr ... (simple expression)
    RustExprParen(String, bool, Position), // @(expr) (expression parentheses)
    MatchExpr(String, Vec<(String, Vec<Node>)>, Position), // @match expr { ... => ... }
    RustExpr(Vec<(String, Vec<Node>)>, Position), // @if ...  { ... } else { ... } / @for ... { ... }
    SectionDirective(String, SectionDirectiveContent, Position), // @section("content")
    SectionBlock(String, Vec<Node>, Position),    // @section content { ... }
    RenderBody,                                   // @render_body (main body of subpage)
    Component(String, Vec<ComponentParameter>, Vec<Node>, Position), // <ComponentName param1 = value1, param2 = value2> tags
    ChildContent, // @child_content (component child content)
    Raw(String),  // @raw {} (raw content)
    UseDirective(String, PathBuf, Box<Node>, Position), // @use "component.rs.html" as Component
    ContinueDirective, // @continue for the loops
    BreakDirective, // @break for the loops
}
