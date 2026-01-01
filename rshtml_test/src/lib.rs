#![allow(unused_imports, dead_code)]

use rshtml::{RsHtml, functions::*, traits::RsHtml, v};
use serde::Serialize;
use std::fmt::Write;

#[derive(RsHtml)]
// #[rshtml(path = "bar.rs.html", no_warn)]
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
    use rshtml::{Block, Expr, traits::Render};
    use std::{fmt, fmt::Error, fmt::Write, fs};
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

        let homepage = HomePage {
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

    #[test]
    fn test_function_like() {
        let mut buffer = String::with_capacity(144);

        let markup = move |f: &mut dyn Write| -> fmt::Result {
            write!(f, "{}", "<div></div>")?;

            Expr::<_, false>({
                let mut users = Vec::new();
                for i in 0..10 {
                    users.push(move |f: &mut dyn Write| -> fmt::Result {
                        write!(f, "{}", i)?;
                        Ok(())
                    });
                }

                users
            })
            .render(f, "iiiiiiiiiiii")?;

            if 5 == 7 {
                Expr::<_, false>(move |_f: &mut dyn Write| -> fmt::Result { Ok(()) })
                    .render(f, "555555555555")?;
            } else {
                Expr::<_, false>(move |_f: &mut dyn Write| -> fmt::Result { Ok(()) })
                    .render(f, "7777777777777")?;
            }

            Ok(())
        };

        markup.render(&mut buffer, "aaaaa").unwrap();

        println!("buff: {buffer}");
        /*

            let mut users = Vec::new();
            for i in 0..10 {
                users.push(v!(<User age=@{i} />));
            };

            let content = if self.x == 5 {
                v!(<Card/>)
            } else {
                v!(<SideBar/>)
            };

            v!(
                @(content)

                @(users)

                <div></div>
                @(3+5)

                @{users.map(|user| v!(<User/>))}

                @{
                    let mut users = Vec::new();
                    for i in 0..10 {
                        users.push(v!(<User/>));
                    };

                    users
                }

                @{
                    if self.x == 5 {
                        v!(<Card/>)
                    } else {
                        v!(<SideBar/>)
                    }
                }

                <p></p>
            );
        */
    }
}
