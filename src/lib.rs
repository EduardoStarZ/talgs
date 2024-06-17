pub mod model;
pub mod controller;
pub mod encryption;
pub mod parser;

use ntex::web;
use askama::Template;

#[derive(Template)]
#[template(path="form.html")]
struct FormTemplate{}

#[web::get("/")]
pub async fn index() -> web::HttpResponse {
    return web::HttpResponse::Ok().body("Hello world");
}

#[web::get("/form")]
pub async fn form() -> web::HttpResponse {
    return web::HttpResponse::Ok().body(FormTemplate{}.render().unwrap())
}
