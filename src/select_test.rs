use super::consts;
use super::utils;
use reqwest::{self, Error};
use select::{self, predicate::Name};

pub fn select_test() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!("select start, heading to: {:?}", &consts::SITE);
    let body = reqwest::blocking::get(consts::SITE)?;
    let document = select::document::Document::from_read(body)?;

    let all_tables = document.find(Name("table")).into_selection();
    let table = all_tables.filter(|t: &select::node::Node| {
        let tr = t.find(Name("tr")).into_selection();
        tr.len() == 327
    });

    let mut final_vec = table
        .find(Name("tr"))
        .iter()
        .skip(1)
        .map(|t| {
            let td = t.find(Name("td")).into_selection().iter().nth(1).unwrap();
            let city = utils::cleanup_text(td.text());
            city
        })
        .collect::<Vec<_>>();

    final_vec.sort();
    Ok(final_vec)
}
