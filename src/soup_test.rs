use super::consts;
use super::utils;
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

    //A little odd using hard to follow types from other crates, but otherwise
    //Soup's process is quite similar with cosmetic changes on method names.
    let table = soup
        .class("wikitable")
        .find_all()
        .find(|table| table.tag("tr").find_all().into_iter().count() == 327)
        .unwrap();

    let tr = table.tag("tr").find_all();

    let mut final_vec = tr
        .into_iter()
        .skip(1)
        .map(|t| {
            let td = t.tag("td").find_all().nth(1).unwrap();
            utils::cleanup_text(td.text())
        })
        .collect::<Vec<_>>();

    final_vec.sort();

    assert!(final_vec.len() == 326);

    Ok(final_vec)
}
