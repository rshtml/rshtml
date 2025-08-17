#![allow(unused_imports, dead_code)]

use rshtml::RsHtml;
use serde::Serialize;
use std::fmt::Write;

#[derive(RsHtml)]
//#[rshtml(path = "about.rs.html")]
pub struct HomePage {
    pub title: String,
    pub content: String,
    pub card_count: usize,
    pub my_var: String,
    pub abc: String,
    pub def: String,
    pub inner: String,
    pub hey: String,
    pub is_ok: bool,
    pub users: Vec<User>,
}

#[derive(Serialize, Debug)]
pub struct User {
    pub name: String,
    pub age: usize,
}

impl HomePage {
    fn my_func(&self) -> String {
        format!("{} {}", self.abc, self.def)
    }

    fn get_header(&self, title: &str) -> String {
        format!("Header: {title}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;
    use pest::Parser;
    use rshtml::traits::RsHtml;
    use std::fs;
    use syn::__private::Span;

    #[test]
    fn test_macro() {
        let users = vec![
            User {
                name: "abc".to_string(),
                age: 10,
            },
            User {
                name: "def".to_string(),
                age: 11,
            },
            User {
                name: "hjk".to_string(),
                age: 12,
            },
            User {
                name: "lmo".to_string(),
                age: 13,
            },
        ];

        let mut homepage = HomePage {
            title: "Hello".to_string(),
            content: "World".to_string(),
            card_count: 1,
            my_var: "This is my var".to_string(),
            abc: "abc".to_string(),
            def: "def".to_string(),
            inner: "inner".to_string(),
            hey: "hey".to_string(),
            is_ok: true,
            users,
        };

        let s = homepage.render().unwrap();

        print!("{s}");
    }
}
