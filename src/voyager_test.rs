use super::consts;
use reqwest::{self, Error};
use voyager;

pub fn select_test() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!("heading to: {:?}", &consts::SITE);
    let mut final_vec: Vec<String> = vec![];
    let body = reqwest::blocking::get(consts::SITE)?.bytes()?;
    let document = select::document::Document::from_read(body_bytes)?;

    Ok(final_vec)
}
