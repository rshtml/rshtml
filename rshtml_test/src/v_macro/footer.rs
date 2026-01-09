use rshtml::{traits::View, v};

pub fn footer(brand: &str) -> impl View {
    v!(move <footer> this is footer {brand} </footer>)
}
