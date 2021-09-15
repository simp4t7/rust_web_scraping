use super::consts;
use super::utils;
use kuchiki::traits::*;
use reqwest::{self, Error};

pub fn kuchiki_test() -> Result<Vec<String>, Error> {
    println!("kuchiki start, heading to: {:?}", &consts::SITE);
    let body = reqwest::blocking::get(consts::SITE)?.text()?;

    //.one() comes from 'html5ever' and basically processes a single StrTendril
    let doc = kuchiki::parse_html().one(body);

    let table_select = doc.select("table").unwrap();

    let table = table_select
        .into_iter()
        .find(|t| {
            let tr_table = t.as_node().select("tr").unwrap();
            tr_table.count() == 327
        })
        .unwrap()
        //Need to convert from a 'NodeDataRef<ElementData>' back to a 'Node' to use '.select()',
        //a bit annoying but whatever...
        .as_node()
        .select("tr")
        .unwrap()
        .collect::<Vec<_>>();

    //Here we iterate over <tr> tags, basically a row with multiple <td> tags inside and
    //isolate the 2nd column for the data we want.
    let mut final_vec = table
        .iter()
        .skip(1)
        .map(|td| {
            let mut city = td
                .as_node()
                .select("td")
                .unwrap()
                //nth(1) will select the 2nd column, which we want.
                .nth(1)
                .unwrap()
                .text_contents();
            city = utils::cleanup_text(city);
            city
        })
        .collect::<Vec<_>>();

    final_vec.sort();

    assert!(final_vec.len() == 326);

    Ok(final_vec)
}
