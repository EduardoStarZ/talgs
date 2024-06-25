use ntex::web;
use ntex_session::Session;
use crate::{database::{connection::{AuthPool, KeyPool}, models::ResultCode}, session::controller};
use askama::Template;
use crate::session::model::{LoginInfo, RegisterForm};

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate{}

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate{}

#[web::post("/login")]
pub async fn login_form(session : Session, auth_pool : web::types::State<AuthPool>, key_pool : web::types::State<KeyPool>, form : web::types::Form<LoginInfo>) -> web::HttpResponse {
    let mut auth_connection = match auth_pool.pool.get() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Failed to receive a connection to the database: {err}");
            return web::HttpResponse::InternalServerError().body("");
        }
    };

    let mut key_connection = match key_pool.pool.get() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Failed to receive a connection to the database: {err}");
            return web::HttpResponse::InternalServerError().body("")
        }
    };

    let response = web::block(move || controller::login_handler(&form, &mut auth_connection, &mut key_connection)).await.unwrap();

    match response.result {
        Some(value) => {
            match value {
                ResultCode::UnauthorizedError => return web::HttpResponse::Unauthorized().body(""),
                _ => return web::HttpResponse::InternalServerError().body("")
            }
        },
        None => {
            match response.token {
                Some(value) => {
                    session.set("Auth-Token", &value).unwrap();
                    return web::HttpResponse::Ok().body(format!("Logged in with token: {value}"));
                },
                None => return web::HttpResponse::InternalServerError().body("")
            } 
        }

    }
}

#[web::get("/login")]
pub async fn login() -> web::HttpResponse {
    return web::HttpResponse::Ok().body(LoginTemplate{}.render().unwrap());
}

#[web::get("/register")]
pub async fn register() -> web::HttpResponse {
    return web::HttpResponse::Ok().body(RegisterTemplate{}.render().unwrap());
}

#[web::post("/register")]
pub async fn register_form(form : web::types::Form<RegisterForm>, auth_pool : web::types::State<AuthPool>,key_pool : web::types::State<KeyPool>) -> web::HttpResponse {
    if form.password1 != form.password2 {
        return web::HttpResponse::Unauthorized().body(RegisterTemplate{}.render().unwrap())
    }

    let mut auth_connection = match auth_pool.pool.get() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Failed to receive a connection to the database: {err}");
            return web::HttpResponse::InternalServerError().body("yuck");
        }
    };

    let mut key_connection = match key_pool.pool.get() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Failed to receive a connection to the database: {err}");
            return web::HttpResponse::InternalServerError().body("yuck")
        }
    };

    let response : bool = web::block(move || controller::register_handler(form, &mut auth_connection, &mut key_connection)).await.unwrap();

    if !response {
        return web::HttpResponse::Unauthorized().body(RegisterTemplate{}.render().unwrap());
    }

    return web::HttpResponse::Ok().body("Created account");
}
