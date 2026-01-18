use std::fmt;

use rshtml::{traits::View, v};

struct Home {
    title: String,
    count: i32,
}

impl View for Home {
    fn render(&self, out: &mut dyn fmt::Write) -> fmt::Result {
        v!(<div>Home Page, title:{&self.title}, count:{self.count}</div>)(out)
    }
}

#[test]
fn view_trait() {
    let mut out = String::with_capacity(24);

    let home = Home {
        title: "home title".to_owned(),
        count: 7,
    };

    home.render(&mut out).unwrap();

    assert_eq!(
        out,
        "<div> Home Page , title : home title , count : 7 </div>"
    )
}

#[test]
fn view_trait_with_v() {
    let home = Home {
        title: "home title".to_owned(),
        count: 7,
    };

    let hello = String::from("Hello");
    let greetings = v!(<p>{hello}</p>);

    let res = v! {
        <h1>RsHtml Title</h1>

        {home}

        {greetings}

        <footer>copyright &copy;</footer>
    };

    println!("text size: {}", res.text_size());

    let mut out = String::with_capacity((res.text_size() as f32 * 1.1) as usize);
    res(&mut out).unwrap();
}
