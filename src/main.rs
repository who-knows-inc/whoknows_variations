#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { field: "value" })
}

#[get("/about")]
fn about() -> Template {
    Template::render("about", context! { field: "value" })
}

#[get("/login")]
fn login() -> Template {
    Template::render("login", context! { field: "value" })
}

#[get("/register")]
fn register() -> Template {
    Template::render("register", context! { field: "value" })
}

#[get("/search")]
fn search() -> Template {
    Template::render("search", context! { field: "value" })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, about, login, register, search])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
