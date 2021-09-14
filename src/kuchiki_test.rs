use super::consts;
use super::utils;
use kuchiki;
use kuchiki::traits::*;
use reqwest::{self, Error};

pub fn kuchiki_test() -> Result<Vec<String>, Error> {
    println!("kuchiki start, heading to: {:?}", &consts::SITE);
    let body = reqwest::blocking::get(consts::SITE)?.text()?;
    let doc = kuchiki::parse_html().one(body);
    let table_select = doc.select("table").unwrap();

    let table = table_select
        .into_iter()
        .find(|t| {
            let tr_table = t.as_node().select("tr").unwrap().collect::<Vec<_>>();
            tr_table.len() == 327
        })
        .unwrap()
        .as_node()
        .select("tr")
        .unwrap()
        .collect::<Vec<_>>();

    let mut final_vec = table
        .iter()
        .skip(1)
        .map(|td| {
            let mut city = td
                .as_node()
                .select("td")
                .unwrap()
                .nth(1)
                .unwrap()
                .text_contents();
            city = utils::cleanup_text(city);
            city
        })
        .collect::<Vec<_>>();

    final_vec.sort();
    //println!("{:?}", &final_vec);
    //println!("{:?}", &final_vec.len());
    Ok(final_vec)
}
