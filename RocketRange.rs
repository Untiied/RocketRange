use rocket::http::{Status, ContentType, StatusClass};
use rocket::response::{Responder, Response, Body};
use rocket::request::Request;


/*

TODO: Add proper chrome support.

Wrote by: Ausitn Mullins @ Tangent
*/

pub struct Range<T> {
    data: T
}

impl<'t> Range<std::fs::File> {
    pub fn get_data(&self, range: Vec<usize>) -> Vec<u8> {
        use std::io::Read;

        let mut file = self.data.try_clone().unwrap();
        let mut all_bytes: Vec<u8> = vec!();
        file.read_to_end(&mut all_bytes).unwrap();

        all_bytes[range[0] .. range[1] + 1 ].to_vec()
    }
}

impl<'t, T: 't> Range<T> {
	pub fn new(data: T) -> Range<T> {
		Range { data }
    }
    
    pub fn invalidate_stream(&self) -> rocket::response::Result<'t> {
        let response = rocket::response::Response::build()
        .status(Status::new(416, "Range Not Satisfiable"))
        .finalize();

        Ok(response)
    }

    pub fn convert_range(&self, range_string: String) -> Vec<usize> {
        let starting_pos: usize = range_string.find("=").unwrap();
        let middle_dash_pos: usize = range_string.find("-").unwrap();

        let first = &range_string[(starting_pos + 1) .. middle_dash_pos];
        let second = &range_string[(middle_dash_pos + 1) .. range_string.len()];

        vec!(first.parse::<usize>().unwrap(), second.parse::<usize>().unwrap())
    }
}

/* This is so Rocket can actually interact with the data type. */
impl<'t> rocket::response::Responder<'t> for Range<std::fs::File> {

    /* This is the main function given to use from Responder trait. */
    fn respond_to(self, request: &Request) -> rocket::response::Result<'t> {
        let requested_range = match request.headers().get_one("range") {
            Some(range) => range,
            /* If we cannot parse the range from the request then stop trying to stream a video. */
            None => return self.invalidate_stream()
        };

        /* Let's gather the actually range of memory that the client is requesting!*/
        let parsed_range = self.convert_range(requested_range.to_string());
        println!("Range of the video request is: {:?}", parsed_range);

        /* get the data for the file */
        /* TODO: We are wasting memory here */
        let data = self.get_data(parsed_range.clone());

        let content_size: u64 = self.data.metadata().unwrap().len();
        let content_range_string = String::from("bytes ") + &parsed_range[0].to_string() + "-" + &parsed_range[1].to_string() + "/" + &content_size.to_string();

        // Basis for a response came from: https://philna.sh/blog/2018/10/23/service-workers-beware-safaris-range-request/.
        let response = rocket::response::Response::build()
        .status(Status::new(206, "Partial Content"))
       
        //.header(rocket::http::Header::new("Content-Type", "video/mp4"))
        .header(rocket::http::Header::new("Content-Length", ((parsed_range[1] - parsed_range[0]) + 1).to_string()))
        .header(rocket::http::Header::new("Content-Range", content_range_string))
        .streamed_body(std::io::Cursor::new(data))
        //.sized_body(std::io::Cursor::new(data))
        .finalize();

        Ok(response)
    }
}

