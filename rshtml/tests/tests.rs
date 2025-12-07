#[cfg(test)]
mod tests {
    use rshtml::{RsHtml, functions::*, traits::RsHtml};

    #[test]
    pub fn test_empty() {
        #[derive(RsHtml)]
        struct EmptyPage {}

        let page = EmptyPage {};
        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_if_else() {
        #[derive(RsHtml)]
        struct IfElsePage {
            is_ok: bool,
            count: i32,
        }

        let page = IfElsePage {
            is_ok: true,
            count: 10,
        };
        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_for() {
        #[derive(RsHtml)]
        struct ForPage {
            users: Vec<String>,
        }

        let page = ForPage {
            users: vec!["Alice".to_string(), "Bob".to_string()],
        };
        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_while() {
        #[derive(RsHtml)]
        struct WhilePage {
            count: i32,
        }

        let page = WhilePage { count: 5 };
        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_match() {
        #[derive(RsHtml)]
        struct MatchPage {
            value: i32,
            data: Option<String>,
        }

        let page = MatchPage {
            value: 10,
            data: Some("Hello".to_string()),
        };
        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_comment() {
        #[derive(RsHtml)]
        struct CommentPage {}

        let page = CommentPage {};
        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_simple_expression() {
        #[derive(RsHtml)]
        struct SimpleExpressionPage {
            value: i32,
            data: Option<String>,
            for_escape: String,
        }

        impl SimpleExpressionPage {
            fn my_func(&self) -> String {
                "my func".to_string()
            }
        }

        let page = SimpleExpressionPage {
            value: 10,
            data: Some("Hello".to_string()),
            for_escape: "'<script/>'".to_string(),
        };
        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_parentheses_expression() {
        #[derive(RsHtml)]
        struct ParenthesesExpressionPage {
            value: i32,
            data: String,
        }

        let page = ParenthesesExpressionPage {
            value: 10,
            data: "Hello".to_string(),
        };
        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_code_block() {
        #[derive(RsHtml)]
        struct CodeBlockPage {}

        let page = CodeBlockPage {};
        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_raw_block() {
        #[derive(RsHtml)]
        struct RawBlockPage {}

        let page = RawBlockPage {};
        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_component() {
        struct Item {
            name: String,
        }

        #[derive(RsHtml)]
        #[rshtml(no_warn)]
        struct ComponentPage {
            value: i32,
            title: String,
            data: String,
            for_escape: String,
            items: Vec<Item>,
        }

        let mut page = ComponentPage {
            value: 10,
            title: "Component".to_string(),
            data: "Hello".to_string(),
            for_escape: "'<script/>'".to_string(),
            items: vec![
                Item {
                    name: "Jack".to_string(),
                },
                Item {
                    name: "John".to_string(),
                },
            ],
        };

        page.value = 11;

        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_continue_break() {
        #[derive(RsHtml)]
        #[rshtml(path = "continue_break.rs.html")]
        struct ContinueBreakPage {
            users: Vec<String>,
        }

        let page = ContinueBreakPage {
            users: vec!["Alice".to_string(), "Bob".to_string(), "John".to_string()],
        };
        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_functions() {
        #[derive(RsHtml)]
        struct FunctionsPage {
            date: chrono::DateTime<chrono::Utc>,
            users: Vec<String>,
        }

        let page = FunctionsPage {
            date: chrono::Utc::now(),
            users: vec!["Alice".to_string(), "Bob".to_string(), "John".to_string()],
        };
        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_escaping() {
        #[derive(RsHtml)]
        struct EscapingPage {
            my_var: String,
        }

        let page = EscapingPage {
            my_var: "<p>This is <strong>bold</strong> text.</p>".to_string(),
        };
        println!("{}", page.render().unwrap());
    }
}
