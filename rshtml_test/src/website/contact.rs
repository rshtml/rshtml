use rshtml::{traits::View, v};

pub fn contact(title: &str, email: &str) -> impl View {
    v! {move
        <section id="tm-section-4" class="row tm-section">
          <div class="col-xs-12 col-sm-6 col-md-6 col-lg-5 col-xl-6 tm-contact-left">
            <h2 class="tm-section-header thin-font col-xs-12 col-sm-12 col-md-12 col-lg-12 col-xl-12">{ title }</h2>
            <form action="index.html" method="post" class="contact-form">

              <div class="col-xs-12 col-sm-6 col-md-6 col-lg-12 col-xl-6 tm-contact-form-left">
                <div class="form-group">
                  <input type="text" id="contact_name" name="contact_name" class="form-control"
                         placeholder="Name" required/>
                </div>
                <div class="form-group">
                  <input type="email" id="contact_email" name="contact_email" class="form-control"
                         placeholder="Email" required/>
                </div>
                <div class="form-group">
                  <input type="text" id="contact_subject" name="contact_subject" class="form-control"
                         placeholder="Subject" required/>
                </div>
              </div>
              <div class="col-xs-12 col-sm-6 col-md-6 col-lg-12 col-xl-6 tm-contact-form-right">
                <div class="form-group">
                                    <textarea id="contact_message" name="contact_message" class="form-control" rows="6"
                                              placeholder="Message" required></textarea>
                </div>

                <button type="submit" class="btn submit-btn">Send It Now</button>
              </div>

            </form>
          </div>

          <div class="tm-white-curve-right col-xs-12 col-sm-6 col-md-6 col-lg-7 col-xl-6">

            <div class="tm-white-curve-right-circle"></div>
            <div class="tm-white-curve-right-rec"></div>

            <div class="tm-white-curve-text">

              <h2 class="tm-section-header green-text">Contact Us</h2>
              <p>if you need a working contact form script, please follow our contact page.
                Thank you for visiting our website.<br/>&nbsp;</p>

              <h3 class="tm-section-subheader green-text">Our Address</h3>
              <address>
                160-420 Praesent consectetur, Dictum massa 10620
              </address>

              <div class="contact-info-links-container">
                                <span class="green-text contact-info">
                                    Tel: <a href="tel:0100200340" class="contact-info-link">090-080-0760</a></span>
                <span class="green-text contact-info">
                                    Email: <a href="mailto:info@company.com"
                                              class="contact-info-link">{ email }</a></span>
              </div>

            </div>

          </div>
        </section>
    }
}
