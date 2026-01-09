mod footer;
mod index;
mod layout;

use footer::footer;
use rshtml::{traits::View, v};

#[test]
fn v_macro() {
    let mut out = String::with_capacity(256);

    let brand = String::from("RsHtml");
    let brand2 = String::from("RsHtml");
    let i = 5;
    let url = "my url";

    let res = v! {
        {brand2}

        {i}

        <p>
            <a href = {url} />
        </p>

        { footer(&brand) }
    };

    res.render(&mut out).unwrap();

    let brand = brand.to_lowercase();
    println!("{brand}");

    println!("{out}");
}
