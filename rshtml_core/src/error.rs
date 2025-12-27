use crate::parser::Rule;
use pest::Position;
use pest::error::{Error, ErrorVariant};

pub fn rename_rules(err: Error<Rule>) -> Box<Error<Rule>> {
    let error = err.renamed_rules(|rule| match rule {
        Rule::EOI => "EOI".to_string(),
        Rule::WHITESPACE => "WHITESPACE".to_string(),
        Rule::rust_identifier => "rust identifier".to_string(),
        Rule::template => "template".to_string(),
        Rule::template_content => "template content".to_string(),
        Rule::inner_template => "template content".to_string(),
        Rule::tag_template => "template content".to_string(),
        Rule::text => "html, text".to_string(),
        Rule::inner_text => "html, text".to_string(),
        Rule::template_params => "template parameters".to_string(),
        Rule::params => "parameters".to_string(),
        Rule::param_name => "parameter name".to_string(),
        Rule::param_type => "parameter type".to_string(),
        Rule::block => "statement".to_string(),
        Rule::rust_expr => "statement: if, for ..".to_string(),
        Rule::rust_expr_head => "valid expression".to_string(),
        Rule::rust_expr_simple => "expression".to_string(),
        Rule::rust_block => "rust block".to_string(),
        Rule::rust_block_content => "rust block".to_string(),
        Rule::rust_code => "rust code".to_string(),
        Rule::rust_expr_paren => "rust expression in parentheses".to_string(),
        Rule::match_expr => "match expression".to_string(),
        Rule::match_expr_head => "match expression".to_string(),
        Rule::match_expr_arm => "match expression arm".to_string(),
        Rule::match_expr_arm_head => "match expression arm".to_string(),
        Rule::match_inner_text => "html, text".to_string(),
        Rule::string_line => "string literal".to_string(),
        Rule::child_content_directive => "child content directive".to_string(),
        Rule::bool => "bool".to_string(),
        Rule::number => "number".to_string(),
        Rule::string => "string literal".to_string(),
        Rule::component => "component".to_string(),
        Rule::attribute => "attribute".to_string(),
        Rule::attribute_name => "attribute name".to_string(),
        Rule::attribute_value => "attribute value".to_string(),
        Rule::component_tag_identifier => "component tag name".to_string(),
        Rule::raw_block => "raw block".to_string(),
        Rule::raw_content => "raw content".to_string(),
        Rule::use_directive => "use directive".to_string(),
        other => format!("{other:?}"),
    });

    Box::new(error)
}

pub struct Custom {
    message: String,
}

pub struct Parsing {
    positives: Vec<Rule>,
    negatives: Vec<Rule>,
}

pub struct E<T = Parsing> {
    state: T,
}

impl E {
    pub fn mes<S: Into<String>>(message: S) -> E<Custom> {
        E::<Custom> {
            state: Custom {
                message: message.into(),
            },
        }
    }

    pub fn pos(rule: Rule) -> E<Parsing> {
        E::<Parsing> {
            state: Parsing {
                positives: vec![rule],
                negatives: vec![],
            },
        }
    }
}

impl E<Custom> {
    pub fn span(self, span: pest::Span<'_>) -> Box<Error<Rule>> {
        Box::new(Error::new_from_span(
            ErrorVariant::CustomError {
                message: self.state.message,
            },
            span,
        ))
    }

    pub fn position(self, position: Position<'_>) -> Box<Error<Rule>> {
        Box::new(Error::new_from_pos(
            ErrorVariant::CustomError {
                message: self.state.message,
            },
            position,
        ))
    }
}

impl E<Parsing> {
    pub fn span(self, span: pest::Span<'_>) -> Box<Error<Rule>> {
        Box::new(Error::new_from_span(
            ErrorVariant::ParsingError {
                positives: self.state.positives,
                negatives: self.state.negatives,
            },
            span,
        ))
    }
}
