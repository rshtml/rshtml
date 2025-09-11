#[cfg(test)]
mod tests {
    use rshtml::{RsHtml, functions::*, traits::RsHtml};

    #[test]
    pub fn test_empty() {
        #[derive(RsHtml)]
        struct EmptyPage {}

        let mut page = EmptyPage {};
        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_if_else() {
        #[derive(RsHtml)]
        struct IfElsePage {
            is_ok: bool,
            count: i32,
        }

        let mut page = IfElsePage {
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

        let mut page = ForPage {
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

        impl WhilePage {
            fn increment(&mut self) -> String {
                self.count += 1;
                "".to_string()
            }
        }

        let mut page = WhilePage { count: 1 };
        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_match() {
        #[derive(RsHtml)]
        struct MatchPage {
            value: i32,
            data: Option<String>,
        }

        let mut page = MatchPage {
            value: 10,
            data: Some("Hello".to_string()),
        };
        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_comment() {
        #[derive(RsHtml)]
        struct CommentPage {}

        let mut page = CommentPage {};
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

        let mut page = SimpleExpressionPage {
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

        let mut page = ParenthesesExpressionPage {
            value: 10,
            data: "Hello".to_string(),
        };
        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_code_block() {
        #[derive(RsHtml)]
        struct CodeBlockPage {}

        let mut page = CodeBlockPage {};
        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_include() {
        #[derive(RsHtml)]
        struct IncludePage {
            value: i32,
            data: String,
        }

        impl IncludePage {
            fn my_func(&self) -> String {
                let mut hold = "Func".to_string();
                hold.push_str(self.data.clone().as_str());
                hold
            }
        }

        let mut page = IncludePage {
            value: 10,
            data: "Hello".to_string(),
        };
        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_layout() {
        #[derive(RsHtml)]
        struct ExtendsPage {
            value: i32,
            data: String,
            for_escape: String,
        }

        impl ExtendsPage {
            fn my_func(&self) -> String {
                let mut hold = "Func".to_string();
                hold.push_str(self.data.clone().as_str());
                hold
            }
        }

        let mut page = ExtendsPage {
            value: 10,
            data: "Hello".to_string(),
            for_escape: "'<script/>'".to_string(),
        };

        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_layout_2() {
        #[derive(RsHtml)]
        struct Extends2Page {
            value: i32,
            data: String,
        }
        impl Extends2Page {
            fn my_func(&self) -> String {
                let mut hold = "Func".to_string();
                hold.push_str(self.data.clone().as_str());
                hold
            }
        }

        let mut page = Extends2Page {
            value: 10,
            data: "Hello".to_string(),
        };
        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_raw_block() {
        #[derive(RsHtml)]
        struct RawBlockPage {}

        let mut page = RawBlockPage {};
        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_component() {
        #[derive(RsHtml)]
        struct ComponentPage {
            value: i32,
            title: String,
            data: String,
            for_escape: String,
        }

        let mut page = ComponentPage {
            value: 10,
            title: "Component".to_string(),
            data: "Hello".to_string(),
            for_escape: "'<script/>'".to_string(),
        };

        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_continue_break() {
        #[derive(RsHtml)]
        #[rshtml(path = "continue_break.rs.html")]
        struct ContinueBreakPage {
            users: Vec<String>,
        }

        let mut page = ContinueBreakPage {
            users: vec!["Alice".to_string(), "Bob".to_string(), "John".to_string()],
        };
        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_no_layout_with_section() {
        #[derive(RsHtml)]
        struct NoLayoutWithSectionPage {}

        let mut page = NoLayoutWithSectionPage {};
        println!("{}", page.render().unwrap());
    }

    #[test]
    pub fn test_functions() {
        #[derive(RsHtml)]
        struct FunctionsPage {
            date: chrono::DateTime<chrono::Utc>,
            users: Vec<String>,
        }

        let mut page = FunctionsPage {
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

        let mut page = EscapingPage {
            my_var: "<p>This is <strong>bold</strong> text.</p>".to_string(),
        };
        println!("{}", page.render().unwrap());
    }
}
