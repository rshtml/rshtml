use chrono::{DateTime, Datelike, Utc};
use rshtml::{traits::View, v};

pub fn footer(home_time: DateTime<Utc>) -> impl View {
    let year = match home_time.year() {
        2023 => 2023,
        2024 => 2024,
        2025 => 2025,
        2026 => 2026,
        _ => home_time.year(),
    };

    v! {
        <footer class="row tm-footer">
            <div class="col-xs-12 col-sm-12 col-md-12 col-lg-12 col-xl-12">
                <p class="text-xs-center tm-footer-text">Copyright &copy; { year } RsHtml</p>
            </div>
        </footer>
    }
}
