use super::consts;
use super::utils;
use reqwest::{self, Error};
use soup;
use soup::prelude::*;
use soup::NodeExt;

struct MyType(String);

impl soup::pattern::Pattern for MyType {
    fn matches(&self, haystack: &str) -> bool {
        self.0.matches(haystack)
    }
}

pub fn soup_test() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!("soup start, heading to: {:?}", &consts::SITE);
    let body = reqwest::blocking::get(consts::SITE)?.text()?;
    let soup = soup::Soup::new(&body);

    let table = soup
        .class("wikitable")
        .find_all()
        .find(|table| table.tag("tr").find_all().into_iter().count() == 327)
        .unwrap();
    let mut tr = table.tag("tr").find_all().collect::<Vec<_>>();
    tr.remove(0);
    let mut final_vec = tr
        .iter()
        .map(|t| {
            let td = t.tag("td").find_all().collect::<Vec<_>>();
            let city = utils::cleanup_text(td[1].text());
            city
        })
        .collect::<Vec<_>>();
    final_vec.sort();
    //println!("{:?}", &final_vec);
    //println!("{:?}", &final_vec.len());
    Ok(final_vec)
}
