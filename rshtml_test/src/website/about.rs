use rshtml::{traits::View, v};

pub fn about(title: &str) -> impl View {
    v! {move
      <section id="tm-section-3" class="row tm-section">
        <div class="tm-white-curve-left col-xs-12 col-sm-6 col-md-6 col-lg-7 col-xl-6">
          <div class="tm-white-curve-left-rec">

          </div>
          <div class="tm-white-curve-left-circle">

          </div>
          <div class="tm-white-curve-text">
            <h2 class="tm-section-header gray-text">{ title }</h2>
            <p class="thin-font">Praesent consectetur dictum massa eu tincidunt. Nulla facilisi. Nam tincidunt nex diam eget sollicitudin. Quisque tincidunt ex sit amet metus ultricies, sed lobortis purus finibus.</p>
            <p>Morbi nex felis rutrum, faucibus odio sed, ullamcorper risus. Sed id condimentum nequq, at iaculis ex. Praesent faucibus viverra ante id auctor. Pellentesque at risus ut arcu blandit consectetur.</p>
          </div>

        </div>
        <div class="tm-flex-center col-xs-12 col-sm-6 col-md-6 col-lg-5 col-xl-6">
          <img src="img/strip-02.jpg" alt="Image" class="img-fluid tm-img"/>
        </div>
      </section>
    }
}
