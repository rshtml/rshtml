#![allow(unused_imports, dead_code)]

mod website;

use rshtml::{
    Exp, Expr, RsHtml, ViewFn,
    functions::*,
    traits::{Render, RsHtml, View},
    v,
};
use serde::Serialize;
use std::{
    borrow::Cow,
    fmt::{self, Display, Write},
};

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
    use rshtml::{Block, Exp, traits::View};
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
        let mut out = String::with_capacity(144);

        let mut mys = String::from("hellooo");
        v!({ &mys })(&mut out).unwrap();
        println!("{mys}");
        mys.push_str("def");
        v!({ &mys }).render(&mut out).unwrap();

        let x = 5;
        let mut users = Vec::new();
        for i in 0..10 {
            users.push(v!(move <User age={i} />));
        }

        let s = String::from("heyy");

        let content = if x == 5 {
            v!(<Card/>).boxed()
        } else {
            v!(<SideBar title={&s}/>).boxed()
        };

        // let hold = users.iter().map(|_user| v!(aa <User/>));

        // println!("{s}");

        // let a: V<impl View> = v!(a);
        // let c = card();

        let res = v!(
        {card()}

        // {side_bar(&c)}

        {nonono()}

            {(0..10).filter(|x| x % 2 == 0).map(|x| x * x).sum::<i32>()}

            {&content}

            {&users}

            <div>fsdf sd</div>
            {3+5}

            {users.iter().map(|_user| v!(aa <User/>)).collect::<Vec<_>>()}
            {users.iter().map(|_user| v!(bb <User/>)).collect::<Vec<_>>()}
            {
                // let mut users = Vec::new();
                for i in 0..10 {
                    // users.push(v!(move <User x={i}/>));
                    v!(move <User x={i}/>)(f)?;
                }
                // users
            }
            {
                if x == 5 {
                     v!(< Card/>).boxed()
                } else {
                     v!(<SideBar/>).boxed()
                }
             }

             <p></p>
        );

        res.render(&mut out).unwrap();
        println!("{out}");

        other();
    }
}

fn card() -> impl View {
    let x = 5;
    let s = String::from("oooo");

    return v!(move this is x: {x}, this is s: {&s});
}

fn bar() -> Box<dyn View> {
    let x = 5;
    let s = String::from("oooo");

    if x == 5 {
        v!(move this is x: {x}, this is s: {&s}).boxed()
    } else {
        v!(oooo).boxed()
    }
}

fn side_bar(a: impl View) -> impl View {
    v!(move {&a} is a crazy)
}

fn nonono() -> impl View {
    let s = String::from("abc");
    v!(move { &s })
}

fn other() {
    let s = String::from("fsd");

    let _ = || &s;

    println!("{s}");

    let mut a = Vec::new();

    for i in 0..10 {
        let d = i.to_owned();
        a.push(move || d);
    }

    let mut buffer = String::with_capacity(100);

    let a = v!(<p>{&s}</p>);

    println!("{s}");
    println!("{s}");

    a.render(&mut buffer).unwrap();
    println!("{s}");
}

// fn fn_like() {
//     let mut buffer = String::with_capacity(144);

//     let markup = move |f: &mut dyn Write| -> fmt::Result {
//         write!(f, "{}", "<div></div>")?;

//         Exp({
//             let mut users = Vec::new();
//             for i in 0..10 {
//                 users.push(move |f: &mut dyn Write| -> fmt::Result {
//                     write!(f, "{}", i)?;
//                     Ok(())
//                 });
//             }

//             users
//         })
//         .render(f)?;

//         if 5 == 7 {
//             Exp(move |_f: &mut dyn Write| -> fmt::Result { Ok(()) }).render(f)?;
//         } else {
//             Exp(move |_f: &mut dyn Write| -> fmt::Result { Ok(()) }).render(f)?;
//         }

//         Ok(())
//     };

//     markup.render(&mut buffer).unwrap();

//     println!("buff: {buffer}");
// }

// enum Node<'a> {
//     Template(Vec<Node<'a>>),
//     Text(&'static str),
//     Expr(Box<dyn View + 'a>),
// }

// impl<'a> Display for Node<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         todo!()
//     }
// }

// fn fn_fn() {
//     let users = vec!["a".to_string(), "b".to_string()];
//     let x = 5;

//     let content = if x == 5 {
//         Node::Template(vec![Node::Text("<Card/>")])
//     } else {
//         Node::Template(vec![Node::Text("<Card/>"), Node::Expr(Box::new(Exp(5)))])
//     };

//     let x = Exp(users
//         .iter()
//         .map(|a| {
//             let b = Box::new(Exp(a));
//             Node::Template(vec![Node::Expr(b)])
//         })
//         .collect::<Vec<_>>());

//     // println!("{x}");
//     let y = x;

//     Node::Template(vec![Node::Text(""), Node::Expr(Box::new(Exp(content)))]);
// }
