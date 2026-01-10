use chrono::{DateTime, Utc};
use rshtml::{functions::time, traits::View, v};

pub fn home(title: &str, home_time: DateTime<Utc>) -> impl View {
    v! {move
      <section id="tm-section-1" class="row tm-section">

        <div class="tm-white-curve-left col-xs-12 col-sm-12 col-md-12 col-lg-7 col-xl-6">
          <div class="tm-white-curve-left-rec"></div>
          <div class="tm-white-curve-left-circle"></div>
          <div class="tm-white-curve-text">
            <h2 class="tm-section-header blue-text">{ title }</h2>
            <p>
              Strip CSS Template is free Bootstrap
              HTML layout for any kind of purpose. You may support us by telling your friends about
              RsHtml website. Please if you have any question.
            </p>
            <p>
              This template is last updated on { time(&home_time).pretty() } for main menu with an external link support. You just
              need
              to put external in link class for external URLs or web pages.
            </p>
          </div>
        </div>

        <div class="tm-home-right col-xs-12 col-sm-12 col-md-12 col-lg-5 col-xl-6">
          <h2 class="tm-section-header">Our Mission</h2>
          <p class="thin-font">We provide 100% free responsive Bootstrap templates for everyone. Feel free to use our
            templates for your clients or business websites. You may visit out website for latest and greatest
            HTML CSS layouts.</p>
        </div>

      </section>
    }
}
