mod consts;
mod crabler_test;
mod kuchiki_test;
mod nipper_test;
mod scraper_test;
mod select_test;
mod soup_test;
mod utils;

fn main() {
    let scraper_result = scraper_test::scraper_test().unwrap();
    let select_result = select_test::select_test().unwrap();
    let nipper_result = nipper_test::nipper_test().unwrap();
    let crabler_result = crabler_test::crabler_test().unwrap();
    let kuchiki_result = kuchiki_test::kuchiki_test().unwrap();
    let soup_result = soup_test::soup_test().unwrap();

    assert_eq!(scraper_result, select_result);
    assert_eq!(select_result, nipper_result);
    assert_eq!(nipper_result, crabler_result);
    assert_eq!(crabler_result, kuchiki_result);
    assert_eq!(kuchiki_result, soup_result);
}
