#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[get("/streaming/<file..>", rank = 1)]
fn streaming_files(file: PathBuf) -> RocketRange::Range<File> {
    let file = File::open(Path::new("assets").join(file)).unwrap();
    return RocketRange::Range::new(file)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .mount("/", routes![streaming_files])
    .mount("/", StaticFiles::from("assets"))
}

fn main() {
    rocket().launch();
}