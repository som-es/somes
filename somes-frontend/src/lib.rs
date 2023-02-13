pub use app::App;
use dotenv_codegen::dotenv;
//use gloo_net::http::Request;

mod api;
mod app;
mod error;
mod routes;

pub use api::request;
pub use error::*;

pub const API_ROOT: &str = dotenv!("API_ROOT");

// lazy_static! {
// static ref ENTRIES_ON_PAGE: u64 = (dotenv!("ENTRIES_ON_PAGE") as &str).parse().unwrap();
// }

pub fn image_path(image: &str) -> String {
    format!("{API_ROOT}static/images/{image}")
}

pub fn pdf_path(pdf: &str) -> String {
    format!("{API_ROOT}static/files/{pdf}")
}
