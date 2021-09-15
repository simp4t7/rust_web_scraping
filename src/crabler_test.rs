use super::consts;
use reqwest::{self, Error};

pub fn crabler_test() -> Result<Vec<String>, Error> {
    println!("crabler start, heading to: {:?}", &consts::SITE);
    let body = reqwest::blocking::get(consts::SITE)?.text()?;
    let doc = crabler::Document::from(body);

    let table_select = doc.select("table");
    let table = table_select.iter().find(|table| {
        let tr_select = table.select("tr");
        tr_select.len() == 327
    });

    //Make sure we only have a single table.
    //It's a little hacky doing it based on the number of rows
    //but there's not any great alternative based on the structure
    //of the HTML.

    let mut final_vec = table
        .unwrap()
        .select("tr")
        .iter()
        .skip(1)
        .map(|tr| {
            //Gets the 2nd column by calling nth(1).
            let mut td = tr.select("td").get(1).unwrap().children();

            //Pretty weird logic here where I have to go down a node, and then back
            //up to select for the tags I want... Could be solved with a clone, or some
            //clever borrow check fighting I don't want to deal with.

            //Because of the return: <Vec<Element>> we can't really use any useful methods...

            //Gets the all the <a> tags within the second <td> column.
            let mut a = td[0].parent().unwrap().select("a");
            //Gets the all the <i> tags within the second <td> column.
            let mut i = td[0].parent().unwrap().select("i");

            //Basically merge everything and then filter it, don't like it but whatever...
            td.append(&mut i);
            td.append(&mut a);

            let city = td
                .iter()
                .find(|city| {
                    //Need to check that we get what we want because we're basically merging
                    //a bunch of junk because couldn't find a way to select everything past
                    //a certain Node.

                    //This is pretty much the worst code among all the candidates but Crabler
                    //wasn't really meant for this so can't complain too much.
                    !city.text().unwrap().is_empty() && !city.text().unwrap().contains('[')
                })
                .unwrap()
                .text()
                .unwrap();
            city
        })
        .collect::<Vec<_>>();

    //sort so that results can be asserted later.
    final_vec.sort();

    assert!(final_vec.len() == 326);

    Ok(final_vec)
}
