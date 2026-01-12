use rshtml::traits::View;
use rshtml::v;
use std::fmt;

fn render_view(view: ::rshtml::ViewFn<impl Fn(&mut dyn fmt::Write) -> fmt::Result>) -> String {
    let mut buffer = String::new();
    let _ = (view.0)(&mut buffer);
    buffer
}

#[test]
fn test_simple_element() {
    let view = v! { <div>Hello</div> };
    assert_eq!(render_view(view), "<div> Hello </div>");
}

#[test]
fn test_nested_elements() {
    let view = v! {
        <ul>
            <li>Item 1</li>
            <li>Item 2</li>
        </ul>
    };

    let output = render_view(view);
    assert!(output.contains("<ul>"));
    assert!(output.contains("<li> Item 1 </li>"));
}

#[test]
fn test_attributes() {
    let view = v! { <div class="container" id="main">Content</div> };
    let output = render_view(view);
    assert!(output.contains(r#"class= "container""#));
    assert!(output.contains(r#"id= "main""#));
}

#[test]
fn test_expression() {
    let name = "World";
    let count = 42;
    let view = v! { <p>Hello {name}, count is {count}</p> };
    assert_eq!(render_view(view), "<p> Hello World , count is 42 </p>");
}

#[test]
fn test_self_closing() {
    let view = v! { <br /><input type="text" /><hr/> };
    assert_eq!(render_view(view), "<br/> <input type= \"text\" /> <hr/>");
}

#[test]
fn test_html_entities() {
    let view = v! { <span>&copy; 2024 &nbsp; &#123;</span> };
    let output = render_view(view);

    assert!(output.contains("&nbsp;"));
    assert!(output.contains("&#123;"));
}

#[test]
fn test_control_flow() {
    let is_admin = true;
    let view = v! {
        <div>
            { if is_admin { "Admin" } else { "User" } }
        </div>
    };
    assert!(render_view(view).contains("Admin"));
}

#[test]
fn test_dynamic_attribute() {
    let active = "\"active\"";
    let view = v! { <button class={active}>Click</button> };
    let output = render_view(view);
    println!("{output}");
    assert!(output.contains(r#"class= &quot;active&quot;"#));
}

#[test]
fn test_reuse() {
    let mut out = String::new();
    let mut my_string = String::from("hello");

    let x = v!(<p> {
        my_string.push_str("hii");
        v!(<div>{my_string}</div>)
    } </p>);

    x.render(&mut out).unwrap();
    x.render(&mut out).unwrap();

    println!("{out}");
}
