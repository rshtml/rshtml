use crate::position::Position;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, String, Position)>,
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
    Template(PathBuf, String, Vec<Function>, Vec<Node>, Position), // main template, contains child nodes (path, name, fns, nodes, position)
    Text(String),                                                  // plain text content (@@ -> @)
    TemplateParams(Vec<(String, String, Position)>, Position),
    RustBlock(String, Position),  // @{ ... } block content (with trim)
    Expr(String, bool, Position), // @expr or @(expr) ... (simple expression) or (expression parentheses)
    MatchExpr(String, Vec<(String, Position, Vec<Node>)>, Position), // @match expr { ... => ... }
    RustExpr(Vec<(String, Position, Vec<Node>)>, Position), // @if ...  { ... } else { ... } / @for ... { ... }
    Component(String, Vec<ComponentParameter>, Vec<Node>, Position), // <ComponentName param1 = value1, param2 = value2> tags
    ChildContent, // @child_content (component child content)
    Raw(String),  // @raw {} (raw content)
    UseDirective(String, PathBuf, Box<Node>, Position), // @use "component.rs.html" as Component
    ContinueDirective, // @continue for the loops
    BreakDirective, // @break for the loops
}
