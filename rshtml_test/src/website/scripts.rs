use rshtml::{traits::View, v};

pub fn scripts() -> impl View {
    v! {
    <script src="js/jquery-1.11.3.min.js"></script>
    <script src="https://www.atlasestateagents.co.uk/javascript/tether.min.js"></script>
    <script src="js/bootstrap.min.js"></script>
    <script src="js/jquery.singlePageNav.min.js"></script>
    <script src="js/navbarNavigation.js"></script>
    }
}
