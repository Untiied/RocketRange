use rocket::http::{Status, ContentType, StatusClass};
use rocket::response::{Responder, Response, Body};
use rocket::request::Request;

pub struct Range<T> {
    data: T
}

impl<'t, T: 't> Range<T> {
	pub fn new(data: T) -> Range<T> {
		Range { data }
	}
}


/* This is so Rocket can actually interact with the data type. */
impl<'T> rocket::response::Responder<'T> for Range<std::fs::File> {

    /* This is the main function given to use from Responder trait. */
    fn respond_to(self, request: &Request) -> rocket::response::Result<'T> {
        let range = request.headers().get_one("range");
        println!("Range of the video Request is: {:?}", range);

        let response = rocket::response::Response::build()
        .status(Status::new(404, "There is no implmentation for this yet."))
        .finalize();

        Ok(response)
    }
}

