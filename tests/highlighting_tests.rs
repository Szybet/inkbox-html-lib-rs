#[cfg(test)]
// Tests
use std::fs::File;
use std::io::prelude::*;

use reader::highlighting::*;

pub fn load_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

pub fn write_file(path: &str, content: String) {
    let _ = std::fs::remove_file(path);

    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

// dir 2
#[test]
pub fn second_main_one_main_highlight() {
    let main_page = load_file("examples/2/main_page.html");
    let highlight_1 = load_file("examples/2/selection_main.html");

    let final_html = highlight_page(main_page, String::new(), String::new(), highlight_1);

    write_file("tmp/second_main_one_main_highlight.html", final_html);
}

#[test]
pub fn second_main_one_next_highlight() {
    let main_page = load_file("examples/2/main_page.html");
    let next_page = load_file("examples/2/next_page.html");
    let highlight_1 = load_file("examples/2/selection_next.html");

    let final_html = highlight_page(main_page, String::new(), next_page, highlight_1);

    write_file("tmp/second_main_one_next_highlight.html", final_html);
}

// dir 1
#[test]
pub fn main_highlight_easy_test() {
    let main_page = load_file("examples/1/example.html");
    let highlight_1 = load_file("examples/1/selection_1.html");
    let highlight_2 = load_file("examples/1/selection_2.html");

    let joined = format!("{}U+001F{}", highlight_1, highlight_2);
    let final_html = highlight_page(main_page, String::new(), String::new(), joined);

    write_file("tmp/main_highlight_easy.html", final_html);
}

#[test]
pub fn main_highlight_easy_one_test() {
    let main_page = load_file("examples/1/example.html");
    let highlight_1 = load_file("examples/1/selection_1.html");

    let final_html = highlight_page(main_page, String::new(), String::new(), highlight_1);

    write_file("tmp/main_highlight_easy.html", final_html);
}

// Deprecated for now
/*
#[test]
pub fn test_convert_plain_to_html() {
    println!("Calling test_convert_plain_to_html");
    let mut file = File::open("examples/example.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    convert_plain_to_html(&mut contents);
    println!("Converted to html: {}", contents);
}

#[test]
pub fn test_find_text_in_html_code() {
    println!("Calling test_convert_plain_to_html");
    let mut file_selection = File::open("examples/example-selection.html").unwrap();
    let mut selection_string = String::new();
    file_selection
        .read_to_string(&mut selection_string)
        .unwrap();

    let mut file_html = File::open("examples/example.html").unwrap();
    let mut html_string = String::new();
    file_html.read_to_string(&mut html_string).unwrap();

    convert_plain_to_html(&mut selection_string);
    convert_plain_to_html(&mut html_string);

    println!("The html code: {}", html_string);

    find_text_in_html_code(selection_string, html_string);
}

#[test]
pub fn test_highlight_html_code() {
    let mut file_html = File::open("examples/example-selection-html.html").unwrap();
    let mut html_string = String::new();
    file_html.read_to_string(&mut html_string).unwrap();

    convert_plain_to_html(&mut html_string);

    std::fs::remove_file("tmp/plain-html-selection.html");

    let mut file = File::create("tmp/plain-html-selection.html").unwrap();
    file.write_all(html_string.as_bytes()).unwrap();

    let mut file_selection = File::open("examples/example-selection.html").unwrap();
    let mut selection_string = String::new();
    file_selection
        .read_to_string(&mut selection_string)
        .unwrap();

    convert_plain_to_html(&mut selection_string);

    // Important: plain
    let content = highlight_html_code(html_string, selection_string);

    std::fs::remove_file("tmp/highlight.html");

    let mut file = File::create("tmp/highlight.html").unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
*/
