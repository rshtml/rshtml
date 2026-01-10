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
    let mut out = String::with_capacity(256);

    let index_page = index();

    index_page.render(&mut out).unwrap();

    out
}
