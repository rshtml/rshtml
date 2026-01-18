use rshtml::{traits::View, v};

use crate::website::scripts::scripts;

pub fn layout(title: &str, navbar: impl View, footer: impl View, content: impl View) -> impl View {
    v! {
        // <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="utf-8" />
            <meta http-equiv="X-UA-Compatible" content="IE=edge"/>
            <meta name="viewport" content="width=device-width, initial-scale=1"/>

            <title>Index - {title}</title>

            // <!-- load stylesheets -->
            <link rel="stylesheet" href="http://fonts.googleapis.com/css?family=Open+Sans:300,400"/>
            // <!-- Google web font "Open Sans" -->
            <link rel="stylesheet" href="css/bootstrap.min.css"/>
            <link rel="stylesheet" href="css/style.css"/>

            // <!-- HTML5 shim and Respond.js for IE8 support of HTML5 elements and media queries -->
            // <!-- WARNING: Respond.js doesn't work if you view the page via file:// -->
            // <!--[if lt IE 9]> -->
            <script src="https://oss.maxcdn.com/html5shiv/3.7.2/html5shiv.min.js"></script>
            <script src="https://oss.maxcdn.com/respond/1.4.2/respond.min.js"></script>
            // <!--<![endif]-->
        </head>
        <body>

        <div class="container tm-container">
            {navbar}

            {content}

            {footer}
        </div>

        {scripts()}

        </body>
        </html>
    }
}
