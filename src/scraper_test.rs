use super::consts;
use reqwest::{self, Error};
use scraper::{Html, Selector};

pub fn scraper_test() -> Result<Vec<String>, Error> {
    println!("scraper start, heading to: {:?}", &consts::SITE);
    let body = reqwest::blocking::get(consts::SITE)?.text()?;
    let document = Html::parse_document(&body);
    let table_selector = Selector::parse("table").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    let mut table = document
        .select(&table_selector)
        .into_iter()
        .find(|t| {
            let tr = t.select(&tr_selector);
            let tr_count = tr.into_iter().count();
            tr_count == 327
        })
        .unwrap()
        .select(&tr_selector)
        .into_iter()
        .collect::<Vec<_>>();

    table.remove(0);

    let mut final_vec = table
        .iter()
        .map(|tr| {
            let td = tr.select(&td_selector).nth(1).unwrap();
            let city = td.text().next().unwrap();
            city.to_string()
        })
        .collect::<Vec<_>>();

    final_vec.sort();

    Ok(final_vec)
}
