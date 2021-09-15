use super::consts;
use super::utils;
use reqwest::{self, Error};

pub fn nipper_test() -> Result<Vec<String>, Error> {
    println!("nipper start, heading to: {:?}", &consts::SITE);
    let body = reqwest::blocking::get(consts::SITE)?.text()?;
    let document = nipper::Document::from(&body);

    let table = document
        .select("table")
        .iter()
        .find(|t| t.select("tr").length() == 327)
        .unwrap();

    //Can't get isolate the <tbody> tag so I can skip the table header.
    //Not the worst, can just skip(1) but would be cleaner.

    //Make sure we only get exactly 1 table returned.
    assert_eq!(table.length(), 1);

    let table_iter = table.select("tr").iter().skip(1);
    let mut final_vec = table_iter
        .map(|tr| {
            let column = tr.select("td").next_sibling().first().text();
            let city: String = utils::cleanup_text(column.to_string());
            city
        })
        .collect::<Vec<_>>();

    final_vec.sort();

    assert!(final_vec.len() == 326);

    Ok(final_vec)
}
