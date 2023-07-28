use rocket::*;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/")]
pub fn index_fn() -> Template{
    let mut context = HashMap::new();
    context.insert("context","string");
    Template::render("index", &context)
}
