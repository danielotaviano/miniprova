use axum::response::{Html, IntoResponse, Response};

pub trait HtmlResponse {
    fn to_html_response(self) -> impl IntoResponse;
}

impl HtmlResponse for String {
    fn to_html_response(self) -> Response {
        Html(self).into_response()
    }
}
