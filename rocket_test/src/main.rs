#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde;
extern crate serde_json;
extern crate rocket_auth_login as auth;
extern crate htmlescape;

use rocket::Data;
use rocket::http::RawStr;
use rocket::response::{NamedFile, Redirect, Flash};
use rocket::request::{FlashMessage, Form};
use rocket::http::{Cookie, Cookies};

use auth::authorization::*;

use std::io;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
const URL: &'static str = "http://localhost:8000";
const LOGIN_URL: &'static str = "http://localhost:8000/login";


struct FormInput {
    username: String,
    password: String,
}
#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}
#[get("/main.js")]
fn js() -> io::Result<NamedFile> {
    NamedFile::open("static/main.js")
}
#[post("/login", data = "<form>")]
fn login(form: Form<FormInput>) -> &'static str {
    "login successful"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            index,
            js
        ])
        .launch();
}
