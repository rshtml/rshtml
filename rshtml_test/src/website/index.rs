use crate::website::{
    about::about, contact::contact, footer, home::home, layout::layout, navbar::navbar,
    services::services,
};
use chrono::Utc;
use rshtml::{traits::View, v};

pub fn index() -> impl View {
    let sections = &[
        ("#tm-section-1", "Home"),
        ("#tm-section-2", "Services"),
        ("#tm-section-3", "About"),
        ("#tm-section-4", "Contact"),
    ];

    let title = "RsHtml";
    let home_time = Utc::now();
    let email = "contact@rshtml.com";

    let content = v! {
        <div class="tm-page-content">
            { home("Introduction", home_time) }
            { services("Our Services") }
            { about("About our company") }
            { contact("Send a message", email) }


            <Contact title="Send a message"/>
        </div>
    };

    layout(title, navbar(sections), footer(home_time), content)
}
