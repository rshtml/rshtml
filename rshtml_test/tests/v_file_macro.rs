use rshtml::{traits::View, v_file};

#[test]
fn test_v_file() {
    let my_view = v_file!("views/v_file.rs.html");

    println!("{}", my_view.text_size());
}
