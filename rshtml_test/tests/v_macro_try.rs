use rshtml::{traits::View, v};

#[test]
fn v_macro_try() {
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

    let res = v!(
        {card()}

        {nonono()}

        {bar()}

        {side_bar(bar())}

        {(0..10).filter(|x| x % 2 == 0).map(|x| x * x).sum::<i32>()}

        {&content}

        {&users}

        <div>fsdf sd</div>
        {3+5}

        {users.iter().map(|_user| v!(aa <User/>)).collect::<Vec<_>>()}
        {users.iter().map(|_user| v!(bb <User/>)).collect::<Vec<_>>()}
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

fn card() -> impl View {
    let x = 5;
    let s = String::from("oooo");

    v!(this is x: {x}, this is s: {s})
}

fn bar() -> Box<dyn View> {
    let x = 5;
    let s = String::from("oooo");

    if x == 5 {
        v!(this is x: {x}, this is s: {s}).boxed()
    } else {
        v!(oooo).boxed()
    }
}

fn side_bar(a: impl View) -> impl View {
    v!({a} is a crazy)
}

fn nonono() -> impl View {
    let s = String::from("abc");
    v!({ s })
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
