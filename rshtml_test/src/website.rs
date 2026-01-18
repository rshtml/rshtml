mod about;
mod contact;
mod footer;
mod home;
mod index;
mod layout;
mod navbar;
mod scripts;
mod services;

use footer::footer;
use index::index;
use rshtml::traits::View;

pub fn website() -> String {
    let index_page = index();
    let text_size = index_page.text_size();
    println!("text size: {text_size}");

    let mut out = String::with_capacity((text_size as f32 * 1.1) as usize);

    index_page.render(&mut out).unwrap();

    out
}
