use super::consts;
use super::utils;
use select::{self, predicate::Name};

pub fn select_test() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!("select start, heading to: {:?}", &consts::SITE);

    //Wish there was a from_str method for document, but works well as is.
    let body = reqwest::blocking::get(consts::SITE)?;
    let document = select::document::Document::from_read(body)?;

    //Nice to have a filter method so don't have to reconstruct the <tr> iterator.
    let all_tables = document.find(Name("table")).into_selection();
    let table = all_tables.filter(|t: &select::node::Node| {
        let tr = t.find(Name("tr")).into_selection();
        tr.len() == 327
    });

    //The basic process here is find the correct table (by checking the length),
    //iterate over the rows (skipping <thead>) and pull data from the 2nd column
    //by taking the 2nd element from the <td> iterator.

    let mut final_vec = table
        .find(Name("tr"))
        .iter()
        .skip(1)
        .map(|t| {
            let td = t.find(Name("td")).into_selection().iter().nth(1).unwrap();
            utils::cleanup_text(td.text())
        })
        .collect::<Vec<_>>();

    final_vec.sort();

    assert!(final_vec.len() == 326);

    Ok(final_vec)
}
