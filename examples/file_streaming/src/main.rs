#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket_contrib::serve::StaticFiles;
use std::path::{Path, PathBuf};
use std::fs::File;

#[get("/streaming/<file..>", rank = 1)]
fn streaming_files(file: PathBuf) -> rocket_range::Range<File> {
    let file = File::open(Path::new("assets").join(file)).unwrap();
    return rocket_range::Range::new(file)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .mount("/", routes![streaming_files])
    .mount("/", StaticFiles::from("assets"))
}

fn main() {
    rocket().launch();
}