use rshtml::{traits::View, v};

pub fn scripts() -> impl View {
    v! {
      <script src="js/jquery-1.11.3.min.js"></script>
      <script src="https://www.atlasestateagents.co.uk/javascript/tether.min.js"></script>
      <script src="js/bootstrap.min.js"></script>
      <script src="js/jquery.singlePageNav.min.js"></script>

      <script>

        var bigNavbarHeight = 90;
        var smallNavbarHeight = 68;
        var navbarHeight = bigNavbarHeight;

        $(document).ready(function(){

          var topOffset = 180;

          $(window).scroll(function(){

            if($(this).scrollTop() > topOffset) {
              $(".navbar-container").addClass("sticky");
            }
            else {
              $(".navbar-container").removeClass("sticky");
            }

          });

          /* Single page nav
          -----------------------------------------*/

          if($(window).width() < 992) {
            navbarHeight = smallNavbarHeight;
          }

          $("#tmNavbar").singlePageNav({
            "currentClass" : "active",
            offset : navbarHeight,
            filter: ":not(.external)"
          });


          /* Collapse menu after click
             http://stackoverflow.com/questions/14203279/bootstrap-close-responsive-menu-on-click
          ----------------------------------------------------------------------------------------*/

          $("#tmNavbar").on("click", "a", null, function () {
            $("#tmNavbar").collapse("hide");
          });

          // Handle nav offset upon window resize
          $(window).resize(function(){
            if($(window).width() < 992) {
              navbarHeight = smallNavbarHeight;
            } else {
              navbarHeight = bigNavbarHeight;
            }

            $("#tmNavbar").singlePageNav({
              "currentClass" : "active",
              offset : navbarHeight,
              filter: ":not(.external)"
            });
          });


          /*  Scroll to top
              http://stackoverflow.com/questions/5580350/jquery-cross-browser-scroll-to-top-with-animation
          --------------------------------------------------------------------------------------------------*/
          $("#go-to-top").each(function(){
            $(this).click(function(){
              $("html,body").animate({ scrollTop: 0 }, "slow");
              return false;
            });
          });

        });
      </script>
    }
}
