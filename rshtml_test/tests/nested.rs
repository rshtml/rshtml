use rshtml::v;

#[test]
pub fn nested() {
    let x = "hey";

    v! {
        <p></p>
        <div>
          {"}"}
          { v!(<p>{x}</p>) }
        </div>
    };
}
