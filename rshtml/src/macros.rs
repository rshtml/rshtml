#[cfg(feature = "axum")]
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_axum {
    (($($impl_generics:tt)*) ($struct_name:ident) ($($type_generics:tt)*) ($($where_clause:tt)*)) => {
        impl $($impl_generics)* ::axum::response::IntoResponse for $struct_name $($type_generics)* $($where_clause)* {
            fn into_response(self) -> ::axum::response::Response {
                use ::axum::http::{StatusCode, header};

                let mut out = String::with_capacity((self.text_size() as f32 * 1.1) as usize);
                match self.render(&mut out) {
                    Ok(_) => ([(header::CONTENT_TYPE, "text/html; charset=utf-8")], out).into_response(),
                    Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to render view").into_response(),
                }
            }
        }
    };
}

#[cfg(not(feature = "axum"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_axum {
    ($($tt:tt)*) => {};
}

#[cfg(feature = "actix-web")]
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_actix {
    (($($impl_generics:tt)*) ($struct_name:ident) ($($type_generics:tt)*) ($($where_clause:tt)*)) => {
        impl $($impl_generics)* ::actix_web::Responder for $struct_name $($type_generics)* $($where_clause)* {
            type Body = ::actix_web::body::BoxBody;

            fn respond_to(self, _req: &::actix_web::HttpRequest) -> ::actix_web::HttpResponse<Self::Body> {
                use ::actix_web::HttpResponse;

                let mut out = String::with_capacity((self.text_size() as f32 * 1.1) as usize);
                match self.render(&mut out) {
                    Ok(_) => HttpResponse::Ok()
                        .content_type("text/html; charset=utf-8")
                        .body(out),
                    Err(_) => HttpResponse::InternalServerError().body("Failed to render view"),
                }
            }
        }
    };
}

#[cfg(not(feature = "actix-web"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_actix {
    ($($tt:tt)*) => {};
}
