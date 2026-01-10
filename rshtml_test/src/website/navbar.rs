use rshtml::{traits::View, v};

pub fn navbar(sections: &[(&str, &str)]) -> impl View {
    v! {
        <div class="row navbar-row">
        <div class="col-xs-12 col-sm-12 col-md-12 col-lg-12 col-xl-12 navbar-container">

          <a href="javascript:void(0)" class="navbar-brand" id="go-to-top">RSHTML</a>
          <nav class="navbar navbar-full">

            <div class="collapse navbar-toggleable-md" id="tmNavbar">

              <ul class="nav navbar-nav">
              {
                for (link, name) in sections.iter() {
                    v!{
                        <li class="nav-item">
                            <a class="nav-link" href={link}>{name}</a>
                        </li>
                    }.render(out)?;
                }
              }
              </ul>

            </div>

           </nav>

          <button class="navbar-toggler hidden-lg-up" type="button" data-toggle="collapse" data-target="#tmNavbar">
            &#9776;
          </button>
        </div>
      </div>
    }
}
