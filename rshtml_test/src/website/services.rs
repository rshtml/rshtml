use rshtml::{traits::View, v};

pub fn services(title: &str) -> impl View {
    v! {
      <section id="tm-section-2" class="row tm-section">
        <div class="tm-flex-center col-xs-12 col-sm-6 col-md-6 col-lg-5 col-xl-6">
          <img src="img/strip-01.jpg" alt="Image" class="img-fluid tm-img"/>
        </div>

        <div class="tm-white-curve-right col-xs-12 col-sm-6 col-md-6 col-lg-7 col-xl-6">

          <div class="tm-white-curve-right-circle"></div>
          <div class="tm-white-curve-right-rec"></div>

          <div class="tm-white-curve-text">
            <h2 class="tm-section-header red-text">{ title }</h2>
            <p>Praesent consectetur dictum massa eu tincidunt. Nulla facilisi. Nam tincidunt nex diam eget sollicitudin. Quisque tincidunt ex sit amet metus ultricies, sed lobortis purus finibus.</p>
            <p class="thin-font">Morbi nex felis rutrum, faucibus odio sed, ullamcorper risus. Sed id condimentum nequq, at iaculis ex. Praesent faucibus viverra ante id auctor. Pellentesque at risus ut arcu blandit consectetur.</p>
          </div>

        </div>
      </section>
    }
}
