use crate::parser::Rule;
use pest::iterators::Pair;
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
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    //IncludeDirective(PathBuf),         // include directive @include("other_view.html")
    Template(Vec<Node>, Position), // main template, contains child nodes
    Text(String, Position),        // plain text content (@@ -> @)
    InnerText(String, Position),   // text inside a block (@@ -> @, @{ -> {, @} -> })
    Comment(String, Position),     // comment content
    ExtendsDirective(PathBuf, Box<Node>, Position), // extends directive @extends("layout.html")
    RenderDirective(String, Position), // yield directive @yield("content")
    RustBlock(String, Position),   // @{ ... } block content (with trim)
    RustExprSimple(String, bool, Position), // @expr ... (simple expression)
    RustExprParen(String, bool, Position), // @(expr) (expression parentheses)
    MatchExpr(
        (String, Position),
        Vec<((String, Position), Vec<Node>)>,
        Position,
    ), // @match expr { ... => ... }
    RustExpr(Vec<((String, Position), Vec<Node>)>, Position), // @if ...  { ... } else { ... } / @for ... { ... }
    SectionDirective(String, SectionDirectiveContent, Position), // @section("content")
    SectionBlock((String, Position), Vec<Node>, Position),    // @section content { ... }
    RenderBody(Position),                                     // @render_body (main body of subpage)
    Component(String, Vec<ComponentParameter>, Vec<Node>, Position), // @componentName(param1 = value1, param2 = value2) { ... } also <CompName p=""/> tags
    ChildContent(Position), // @child_content (component child content)
    Raw(String, Position),  // @raw {} (raw content)
    UseDirective(String, PathBuf, Box<Node>, Position), // @use "component.rs.html" as Component
    ContinueDirective(Position), // @continue for the loops
    BreakDirective(Position), // @break for the loops
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Position((usize, usize), (usize, usize)); // start: (line, col), end: (line, col)

impl From<&Pair<'_, Rule>> for Position {
    fn from(value: &Pair<Rule>) -> Self {
        Self(
            value.as_span().start_pos().line_col(),
            value.as_span().end_pos().line_col(),
        )
    }
}
