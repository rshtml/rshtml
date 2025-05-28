use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum TextBlockItem {
    Text(String),
    RustExprSimple(String, bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextLineItem {
    Text(String),
    RustExprSimple(String, bool),
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
    RustExprSimple(String, bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ComponentParameterValue {
    Bool(bool),
    Number(String), // TODO: convert int or float
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
    Template(Vec<Node>),                                   // main template, contains child nodes
    Text(String),                                          // plain text content (@@ -> @)
    InnerText(String),                                     // text inside a block (@@ -> @, @{ -> {, @} -> })
    Comment(String),                                       // comment content
    ExtendsDirective(PathBuf, Box<Node>),                  // extends directive @extends("layout.html")
    RenderDirective(String),                               // yield directive @yield("content")
    RustBlock(Vec<RustBlockContent>),                      // @{ ... } block content (with trim)
    RustExprSimple(String, bool),                          // @expr ... (simple expression)
    RustExprParen(String, bool),                           // @(expr) (expression parentheses)
    MatchExpr(String, Vec<(String, Vec<Node>)>),           // @match expr { ... => ... }
    RustExpr(Vec<(String, Vec<Node>)>),                    // @if ...  { ... } else { ... } / @for ... { ... }
    SectionDirective(String, SectionDirectiveContent),     // @section("content")
    SectionBlock(String, Vec<Node>),                       // @section content { ... }
    RenderBody,                                            // @render_body (main body of subpage)
    Component(String, Vec<ComponentParameter>, Vec<Node>), // @componentName(param1 = value1, param2 = value2) { ... } also <CompName p=""/> tags
    ChildContent,                                          // @child_content (component child content)
    Raw(String),                                           // @raw {} (raw content)
    UseDirective(String, PathBuf, Box<Node>),              // @use "component.rs.html" as Component
    ContinueDirective,                                     // @continue for the loops
    BreakDirective,                                        // @break for the loops
}
