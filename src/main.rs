use std::fs;

mod html;
mod css;

fn main() {
    let html_file = fs::read_to_string("hello_world.html").unwrap();
    let css_file = fs::read_to_string("test.css").unwrap();
    let html_parser = html::parse(html_file);
    let css_parser = css::parse(css_file);

    println!("#{:?}", html_parser);
    println!("#{:?}", css_parser);
}
