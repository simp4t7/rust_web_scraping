use super::consts;
use reqwest::{self, Error};
use scraper::{Html, Selector};

pub fn scraper_test() -> Result<Vec<String>, Error> {
    println!("scraper start, heading to: {:?}", &consts::SITE);
    let body = reqwest::blocking::get(consts::SITE)?.text()?;
    let document = Html::parse_document(&body);

    //A little odd with selectors being so verbose I just throw them all at the
    //top like constants.
    let table_selector = Selector::parse("table").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    //No filter method so have to reconstruct the <tr> iterator.
    let table = document
        .select(&table_selector)
        .into_iter()
        .find(|t| {
            let tr = t.select(&tr_selector);
            let tr_count = tr.into_iter().count();
            tr_count == 327
        })
        .unwrap()
        .select(&tr_selector);

    let mut final_vec = table
        .into_iter()
        //skip <thead>
        .skip(1)
        .map(|tr| {
            let td = tr.select(&td_selector).nth(1).unwrap();
            td.text().next().unwrap().to_string()
            //One of the few that doesn't need text cleanup, because '.text()'
            //returns an iterator over Text nodes.
        })
        .collect::<Vec<_>>();

    final_vec.sort();

    assert!(final_vec.len() == 326);

    Ok(final_vec)
}
