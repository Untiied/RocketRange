/*
    This file is written as a standalone solution to allow File streaming 
    for the async web framwork "Rocket" for Rust. The biggest problem using
    this custom Responder type is to allow for video streaming to the Safari
    browser.
    
    The issue this was made to solve has been known since 2018 but hasn't seen
    any significant progress. Due to the severity of the issue, I decided to create
    a good standalone implmentation. (https://github.com/SergioBenitez/Rocket/issues/806)
    
    Original Author: Austin Mullins
    Created: 10/04/2020

    (c) Copyright by Tangent Inc.
*/

extern crate chrono;

use rocket::http::Status;
use rocket::request::Request;

use chrono::offset::Utc;
use chrono::DateTime;

pub struct Range<T> {
    data: T
}

impl<'t> Range<std::fs::File> {

    // "Generates" an ETag, this should be fleshed out more for uniquness, but for right now we are
    // just going to use the length of the video.
    pub fn generate_etag(&self) -> String {
        /* We should also add the time last modified from the metadata */
        self.data.metadata().unwrap().len().to_string()
    }

    // This will get only the bytes that were requested form our local data pool.
    pub fn get_data(&self, range: Vec<usize>) -> Vec<u8> {
        use std::io::Read;

        let mut file = self.data.try_clone().unwrap();
        let mut all_bytes: Vec<u8> = vec!();
        file.read_to_end(&mut all_bytes).unwrap();

        all_bytes[range[0] .. range[1] + 1 ].to_vec()
    }

    // This added in support for browsers other than Safari. 
    // This will send the browers the entire file because that's how they choose to handle it.
    pub fn send_whole_file(&self, len: u64, last_edited: String) -> rocket::response::Result<'t> {
        let data = self.get_data(vec!(0, len as usize - 1));

        let response = rocket::response::Response::build()
        .status(Status::new(200, "OK"))
        .header(rocket::http::Header::new("Accept-Ranges", "bytes"))
        .header(rocket::http::Header::new("Content-Type", "video/mp4"))
        .header(rocket::http::Header::new("Content-Length", len.to_string()))
        .header(rocket::http::Header::new("Last-Modified", last_edited))
        .sized_body(std::io::Cursor::new(data))
        .finalize();

        Ok(response)
    }

}

impl<'t, T: 't> Range<T> {
	pub fn new(data: T) -> Range<T> {
		Range { data }
    }
    
    // This shouldn't ever be used, but just in case we can't services a request at all
    // it will tell the client that something was funky.
    pub fn invalidate_stream(&self) -> rocket::response::Result<'t> {
        let response = rocket::response::Response::build()
        .status(Status::new(416, "Range Not Satisfiable"))
        .finalize();

        Ok(response)
    }

    // According to RFC-2616 we can send the client 304 to signify that
    // we haven't changed the file at all and it has the most up to-date one.
    // https://www.w3.org/Protocols/rfc2616/rfc2616-sec13.html#sec13.5.3
    pub fn reassure_stream(&self) -> rocket::response::Result<'t> {
        let response = rocket::response::Response::build()
        .status(Status::new(304, "Not Modified"))
        .finalize();

        Ok(response)
    }

    // This function just takes in the string for the range, and will parse the range 
    // into the corresponding Integer values.
    pub fn convert_range(&self, range_string: String) -> Result<Vec<usize>, std::num::ParseIntError> {
        let starting_pos: usize = range_string.find("=").unwrap();
        let middle_dash_pos: usize = range_string.find("-").unwrap();

        let first = &range_string[(starting_pos + 1) .. middle_dash_pos];
        let second = &range_string[(middle_dash_pos + 1) .. range_string.len()];

        Ok(vec!(first.parse::<usize>()?, second.parse::<usize>()?))
    }
}

/* This is so Rocket can actually interact with the data type. */
impl<'t> rocket::response::Responder<'t> for Range<std::fs::File> {

    /* This is the main function given to use from Responder trait. */
    fn respond_to(self, request: &Request) -> rocket::response::Result<'t> {

        /* We are going to gather all of the meta data prior to actually starting the function */
        let content_size: u64 = self.data.metadata().unwrap().len();
        let last_edited_date: DateTime<Utc> = self.data.metadata().unwrap().modified().unwrap().into();

        // Generate the ETag for the file. 
        // The ETag helps the caching system to not waste bandwidth because out file hasn't
        // changed since the last time we downloaded it.
        let etag = self.generate_etag();

        // Check if our ETag vaules are the same as the clients, if so,
        // just tell the client it has the most up to-date version, and theres no need to update.Utc
        match request.headers().get_one("etag") {
            Some(request_etag) => {
                if request_etag == etag {
                    return self.reassure_stream();
                }
            }
            // Nothing needs to happen here so we can just continue the function.
            None => {}
        }

        /* Here we get the range header from the request */
        let requested_range = match request.headers().get_one("range") {
            Some(range) => range,
            /* If we cannot parse the range from the request then stop trying to stream a video. */
            None => return self.invalidate_stream()
        };

        /* Let's gather the actually range of memory that the client is requesting!*/
        let parsed_range = match self.convert_range(requested_range.to_string()) {
            Ok(range) => range,
            Err(_) => return self.send_whole_file(content_size, last_edited_date.format("%a, %d %b %Y %T %Z").to_string())
        };
        println!("    => Requested range: {:?}", parsed_range);

        /* get the data for the file */
        /* TODO: We are wasting memory here */
        let data = self.get_data(parsed_range.clone());

        /* This is just setting up the values to be send to the client. */
        let content_range_string = String::from("bytes ") + &parsed_range[0].to_string() + "-" + &parsed_range[1].to_string() + "/" + &content_size.to_string();
        let expire_time: DateTime<Utc> = (std::time::SystemTime::now() + std::time::Duration::from_secs(3600)).into();

        // Basis for a response came from: https://philna.sh/blog/2018/10/23/service-workers-beware-safaris-range-request/ &
        // the RFC-2616 standard listed above.
        // I have no clue if we are sending too much information, but this is the call where it worked.
        let response = rocket::response::Response::build()
        .status(Status::new(206, "Partial Content"))
        .header(rocket::http::Header::new("ETag", etag))
        .header(rocket::http::Header::new("Expires", expire_time.format("%a, %d %b %Y %T %Z").to_string()))
        .header(rocket::http::Header::new("Accept-Ranges", "bytes"))
        .header(rocket::http::Header::new("Content-Type", "video/mp4"))
        .header(rocket::http::Header::new("Content-Range", content_range_string))
        .header(rocket::http::Header::new("Content-Length", ((parsed_range[1] - parsed_range[0]) + 1).to_string()))
        .header(rocket::http::Header::new("Last-Modified", last_edited_date.format("%a, %d %b %Y %T %Z").to_string()))
        .streamed_body(std::io::Cursor::new(data))
        .finalize();

        Ok(response)
    }
}