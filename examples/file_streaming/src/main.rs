#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::response::NamedFile;
use rocket_contrib::serve::StaticFiles;

macro_rules! crate_relative {
    ($path:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/", $path)
    };
}

use std::path::{Path, PathBuf};
use std::fs::File;

#[get("/<file..>", rank = 1)]
fn streaming_files(file: PathBuf) -> rocket_range::Range<File> {
    let file = File::open(Path::new(crate_relative!("assets")).join(file)).unwrap();
    return rocket_range::Range::new(file)
}

#[get("/", rank = 1)]
fn index() -> Option<NamedFile> {
    NamedFile::open(crate_relative!("assets/index.html")).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .mount("/", routes![index, streaming_files])
    .mount("/", StaticFiles::from(crate_relative!("/assets")))
}

fn main() {
    rocket().launch();
}