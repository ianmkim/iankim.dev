use rocket::*;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use rocket_contrib::templates::Template;
use rocket_contrib::uuid::Uuid;

use std::collections::HashMap;


#[derive(Serialize, Deserialize)]
pub struct Response {
    status: String,
    message: String,
}

impl Response {
    fn ok(msg: &str) -> Self {
        Response {
            status: "Success".to_string(),
            message: msg.to_string(),
        }
    }
    fn err(msg: &str) -> Self {
        Response {
            status: "Error".to_string(),
            message: msg.to_string(),
        }
    }
}


#[get("/")]
pub fn index_fn() -> Template{
    let mut context = HashMap::new();
    context.insert("context","string");
    Template::render("index", &context)
}
