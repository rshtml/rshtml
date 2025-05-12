#[cfg(test)]
mod tests {
    use pest::Parser;
    use rshtml::config::Config;
    use rshtml::node::Node;
    use rshtml::parser::{RsHtmlParser, Rule};
    use rshtml::{ast_viewer, viewer};
    use std::fs;

    #[test]
    fn test_template_format() {
        let views = vec![
            "layout.rs.html",
            "index.rs.html",
            "about.rs.html",
            "home.rs.html",
            "header.rs.html",
            "bar.rs.html",
        ];

        let mut config = Config::default();
        config.set_views_base_path("tests/views".to_string());

        let ast = match RsHtmlParser::new().run(views[3], config) {
            Ok(ast) => ast,
            Err(err) => {
                let message = format!("{}", err.to_string());
                println!("{}", message);
                return;
            }
        };

        ast_viewer::view_node(&ast, 0);

        assert!(matches!(ast, Node::Template(_)));
    }

    #[test]
    fn test_template_format_without_parsing() {
        let template = fs::read_to_string("tests/views/home.rs.html").unwrap();
        let pairs = match RsHtmlParser::parse(Rule::template, template.as_str()) {
            Ok(pairs) => pairs,
            Err(err) => {
                let message = format!("{}", err.to_string());
                println!("{}", message);
                return;
            }
        };

        viewer::execute_pairs(pairs, 0, true);
    }
}
