use scraper::{Html,Selector};
use std::fs::File;
use std::io::prelude::*;




#[tokio::main]
async fn main() {
    let html = reqwest::get("https://vanipedia.org/wiki/Category:Essential_Subjects")
    .await.unwrap()
    .text()
    .await;
    let fragment = Html::parse_fragment(&html.unwrap());
    let selector = Selector::parse("div.mw-category-group ul li a").unwrap();
    let mut all_categories_names: Vec<String> = vec![]; // result array
    let mut file = File::create("vanipedia_essential_categories.json").unwrap();

    file.write_all(&format!("[\n").as_bytes()).unwrap();
    for element in fragment.select(&selector) {
        let inner_html = element.inner_html();
        let category_name = inner_html.split(" - an").next().unwrap();
        all_categories_names.push(String::from(category_name));
        file.write_all(&format!("\"{category_name}\",\n").as_bytes()).unwrap();
    }
    file.write_all(&format!("]").as_bytes()).unwrap();



}

