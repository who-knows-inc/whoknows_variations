use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! { field: "value" })
}

#[get("/about")]
pub fn about() -> Template {
    Template::render("about", context! { field: "value" })
}

#[get("/login")]
pub fn login() -> Template {
    Template::render("login", context! { field: "value" })
}

#[get("/register")]
pub fn register() -> Template {
    Template::render("register", context! { field: "value" })
}

#[get("/search")]
pub fn search() -> Template {
    Template::render("search", context! { field: "value" })
}
