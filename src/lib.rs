#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_attributes)]

#[macro_use] use rocket::*;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::helmet::SpaceHelmet;
use rocket_contrib::templates::Template;


mod routes;

pub fn rocket_builder() -> rocket::Rocket {
    rocket::ignite()
        .attach(SpaceHelmet::default())
        .attach(Template::fairing())
        .mount("/", routes![routes::index::index_fn])
        .mount("/ping", routes![routes::ping::ping_fn])
        .mount("/static", StaticFiles::from("static/"))
}

